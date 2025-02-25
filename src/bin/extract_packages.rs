use std::env;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};
use toml::{Value, Table};

#[derive(Serialize, Deserialize, Debug)]
struct PackageInfo {
    path: String,
    dependencies: HashMap<String, String>,
    dev_dependencies: HashMap<String, String>,
}

fn analyze_workspace(workspace_root: &str) -> Result<HashMap<String, PackageInfo>, Box<dyn std::error::Error>> {
    eprintln!("[DEBUG] Current directory: {:?}", std::env::current_dir()?);
    let manifest_path = Path::new(workspace_root).join("Cargo.toml");
    eprintln!("[DEBUG] Looking for manifest at: {}", manifest_path.display());

    // validations
    if !manifest_path.exists() {
        return Err(format!(
            "Workspace Cargo.toml missing at: {}", 
            manifest_path.display()
        ).into());
    }
    
    if !manifest_path.is_file() {
        return Err(format!(
            "Path is not a file: {}", 
            manifest_path.display()
        ).into());
    }    

    let _workspace_toml = fs::read_to_string(&manifest_path)
        .map_err(|e| format!("Path error: {} - {}", manifest_path.display(), e))?;

    let manifest_path = Path::new(workspace_root).join("Cargo.toml");
    let workspace_toml = fs::read_to_string(&manifest_path)?;
    let workspace_data: Value = workspace_toml.parse()?;

    let empty_table = Table::new();  // Keep this alive through scope
    let workspace_deps = workspace_data
        .get("workspace")
        .and_then(|w| w.get("dependencies"))
        .and_then(|d| d.as_table())
        .unwrap_or(&empty_table);  // Now references long-lived value

    let mut analysis = HashMap::new();

    if let Some(members) = workspace_data
        .get("workspace")
        .and_then(|w| w.get("members"))
        .and_then(|m| m.as_array())
    {
        for member in members {
            if let Some(member_str) = member.as_str() {
                let member_path = Path::new(workspace_root).join(member_str);
                analyze_member_package(&member_path, workspace_root, workspace_deps, &mut analysis)?;
            }
        }
    }

    Ok(analysis)
}

fn analyze_member_package(
    path: &Path,
    workspace_root: &str,
    workspace_deps: &Table,
    analysis: &mut HashMap<String, PackageInfo>,
) -> Result<(), Box<dyn std::error::Error>> {
    let manifest_path = path.join("Cargo.toml");
    let package_toml = fs::read_to_string(&manifest_path)?;
    let package_data: Value = package_toml.parse()?;

    let package_name = package_data
        .get("package")
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .unwrap_or_default()
        .to_string();

    let (dependencies, dev_dependencies) = process_dependencies(&package_data, workspace_deps);

    analysis.insert(
        package_name,
        PackageInfo {        
            path: path.canonicalize()?
                .strip_prefix(Path::new(workspace_root).canonicalize()?)?
                .to_str()
                .unwrap_or("")
                .replace('\\', "/"),
            dependencies,
            dev_dependencies,
        },
    );

    Ok(())
}

fn process_dependencies(package_data: &Value, workspace_deps: &Table) -> (HashMap<String, String>, HashMap<String, String>) {
    let process = |deps: Option<&Table>| -> HashMap<String, String> {
        deps.map_or_else(HashMap::new, |table| {
            table.iter().filter_map(|(name, spec)| {
                if spec.get("workspace").and_then(|w| w.as_bool()).unwrap_or(false) {
                    workspace_deps.get(name)
                        .and_then(|v| v.get("path"))
                        .and_then(|p| p.as_str())
                        .map(|path| (name.clone(), path.to_string()))
                } else {
                    spec.get("path")
                        .and_then(|p| p.as_str())
                        .map(|path| (name.clone(), path.to_string()))
                }
            }).collect()
        })
    };

    let dependencies = process(package_data.get("dependencies").and_then(|d| d.as_table()));
    let dev_dependencies = process(package_data.get("dev-dependencies").and_then(|d| d.as_table()));

    (dependencies, dev_dependencies)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("[DEBUG] Starting analysis");
    let args: Vec<String> = env::args().collect();
    eprintln!("[DEBUG] Raw args: {:?}", args);
    let workspace_root = args.get(1).ok_or("Missing workspace path argument")?;
    let default_output = "dependencies.json".to_string();
    let output_file = args.iter()
        .position(|a| a == "--output")
        .and_then(|p| args.get(p + 1))
        .unwrap_or(&default_output);
    let output_dir = Path::new("output");
    let output_path = output_dir.join(output_file);

    eprintln!("[DEBUG] Workspace root: {}", workspace_root);
    eprintln!("[DEBUG] Output file: {}", output_path.display());

    // Create directory if needed
    if !output_dir.exists() {
        std::fs::create_dir_all(output_dir)?;
    }

    let analysis = analyze_workspace(&workspace_root)
        .map_err(|e| format!("Failed analyzing {}: {}", workspace_root, e))?;
    let json = serde_json::to_string_pretty(&analysis)?;
    std::fs::write(&output_path, json)?;
    eprintln!("[SUCCESS] Wrote output to {}", output_path.display());
    Ok(())
}

