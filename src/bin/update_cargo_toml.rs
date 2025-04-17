use std::{fs, fs::File, io::Write, env, collections::HashSet};
use toml::{Table, Value};
// use agave_checkout_gen::constants::AGAVE_PATH;

const AGAVE_PATH: &str = "./../agave-clone/agave";

fn collect_dependencies(workspace: &Table, package: &str, collected: &mut HashSet<String>) {
    if collected.contains(package) {
        return;
    }
    collected.insert(package.to_string());

    if let Some(deps) = workspace.get("dependencies").and_then(|d| d.as_table()) {
        // Check if this package has any dependencies
        if let Some(pkg_deps) = deps.get(package).and_then(|p| p.as_table()) {
            // Look at path dependencies
            if let Some(path) = pkg_deps.get("path").and_then(|p| p.as_str()) {
                let pkg_name = path.split('/').last().unwrap_or(path);
                collect_dependencies(workspace, pkg_name, collected);
            }
            // Look at version dependencies
            if let Some(_version) = pkg_deps.get("version").and_then(|v| v.as_str()) {
                collect_dependencies(workspace, package, collected);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the absolute path to the workspace directory
    let workspace_dir = env::current_exe()?
        .parent().ok_or("Failed to get parent directory")?
        .parent().ok_or("Failed to get parent directory")?
        .parent().ok_or("Failed to get workspace directory")?
        .to_path_buf();

    // Read the original Cargo.toml
    let toml_path = format!("{}/Cargo.toml", AGAVE_PATH);
    let cargo_toml_content = fs::read_to_string(&toml_path)
        .map_err(|e| format!("Failed to read {}: {}", toml_path, e))?;
    let mut cargo_toml: Table = cargo_toml_content.parse()?;

    let checkout_path = workspace_dir.join("output").join("sparse_checkout_command.sh");
    let checkout_command = fs::read_to_string(&checkout_path)
        .map_err(|e| format!("Failed to read {}: {}", checkout_path.display(), e))?;
    
    // Get all components that need to be checked out
    let checked_out: Vec<String> = checkout_command
        .lines()
        .skip(1)// first line is git checkout command
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().to_string().replace(" \\",""))
        .collect();

    // Create a list of all components to check against, including both path and package versions
    let mut all_components = Vec::new();
    for path in &checked_out {
        all_components.push(path.clone());
        if !path.starts_with("solana-") {
            all_components.push(format!("solana-{}", path));
        }
    }

    // Keep track of needed dependencies
    let mut needed_deps = HashSet::new();

    // Update members and collect dependencies
    if let Some(workspace) = cargo_toml.get_mut("workspace").and_then(|w| w.as_table_mut()) {
        if let Some(members) = workspace.get("members").and_then(|m| m.as_array()) {
            let filtered_members: Vec<Value> = members
                .iter()
                .filter_map(|member| {
                    let member_str = member.as_str().unwrap();
                    // Keep the member if it matches any of our components
                    let is_match = all_components.iter().any(|c| {
                        if c.ends_with("/*") {
                            let base = c.trim_end_matches("/*");
                            member_str.starts_with(base)
                        } else {
                            member_str == c || member_str.starts_with(&format!("{}/", c))
                        }
                    });
                    if is_match && member_str != "sdk" {
                        // Add to needed deps and collect their dependencies
                        let member_name = if member_str.contains('/') {
                            member_str.split('/').next().unwrap_or(member_str)
                        } else {
                            member_str
                        };
                        let pkg_name = format!("solana-{}", member_name);
                        collect_dependencies(workspace, &pkg_name, &mut needed_deps);
                        Some(Value::String(member_str.to_string()))
                    } else {
                        None
                    }
                })
                .collect();
            
            workspace.insert("members".to_string(), Value::Array(filtered_members));
        }
    }

    // Always include curve25519 if it's in the original patches
    // This is safer than trying to determine if it's needed
    let mut keep_curve25519 = false;
    if let Some(patch_table) = cargo_toml.get("patch") {
        if let Some(crates_io) = patch_table.get("crates-io").and_then(|c| c.as_table()) {
            keep_curve25519 = crates_io.contains_key("solana-curve25519");
        }
    }

    // Update patches
    if let Some(patch_table) = cargo_toml.get_mut("patch") {
        if let Some(crates_io) = patch_table.get_mut("crates-io").and_then(|c| c.as_table_mut()) {
            let mut new_crates_io = Table::new();
            
            for (key, value) in crates_io.iter() {
                if let Some(path) = value.get("path").and_then(|v| v.as_str()) {
                    let path_dir = path.split('/').next().unwrap_or("");
                    let keep_patch = all_components.iter().any(|c| {
                        if c.ends_with("/*") {
                            let base = c.trim_end_matches("/*");
                            path_dir.starts_with(base)
                        } else {
                            path_dir == c || path_dir.starts_with(&format!("{}/", c))
                        }
                    }) || needed_deps.iter().any(|d| key.starts_with(d))
                    || (keep_curve25519 && key == "solana-curve25519");
                    
                    if keep_patch {
                        new_crates_io.insert(key.clone(), value.clone());
                    }
                }
            }
            
            if new_crates_io.is_empty() {
                cargo_toml.remove("patch");
            } else {
                *crates_io = new_crates_io;
            }
        }
    }

    // Write the complete updated Cargo.toml
    let mut output_file = File::create("./output/Cargo.toml")?;
    write!(output_file, "{}", cargo_toml.to_string())?;

    Ok(())
} 