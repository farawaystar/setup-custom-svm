use std::{fs, fs::File, io::Write};
use toml::{Table, Value};
// use agave_checkout_gen::constants::AGAVE_PATH;

const AGAVE_PATH: &str = "./../agave-clone/agave";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the original Cargo.toml
    let toml_path = format!("{}/Cargo.toml", AGAVE_PATH);
    let cargo_toml_content = fs::read_to_string(toml_path)?;
    let mut cargo_toml: Table = cargo_toml_content.parse()?;

    let checkout_path = format!("./output/sparse_checkout_command.sh");
    let checkout_command = fs::read_to_string(checkout_path)?;
    let mut checked_out: Vec<String> = checkout_command
      .lines()
      .skip(1)// first line is git checkout command
      .filter(|line| !line.is_empty())
      .map(|line| line.trim().to_string().replace(" \\",""))
      .filter(|line| !line.starts_with("curves"))
      .collect();

    if checkout_command.contains("curve") {
        checked_out.push("curves/*".to_string());
    }

    // Update members
    if let Some(workspace) = cargo_toml.get_mut("workspace").and_then(|w| w.as_table_mut()) {
        if let Some(members) = workspace.get("members").and_then(|m| m.as_array()) {
            let filtered_members: Vec<Value> = members
                .iter()
                .filter_map(|member| {
                    let member_str = member.as_str().unwrap();
                    if checked_out.iter().any(|folder| member_str == folder && folder != "sdk") {
                        Some(Value::String(member_str.to_string()))
                    } else {
                        None
                    }
                })
                .collect();
            
            workspace.insert("members".to_string(), Value::Array(filtered_members));
        }
    }

    // Update patches
    if let Some(patch_table) = cargo_toml.get_mut("patch") {
        if let Some(crates_io) = patch_table.get_mut("crates-io").and_then(|c| c.as_table_mut()) {
            let mut new_crates_io = Table::new();
            
            for (key, value) in crates_io.iter() {
                if let Some(path) = value.get("path").and_then(|v| v.as_str()) {
                    let path_dir = path.split('/').next().unwrap_or("");
                    if checked_out.iter().any(|c| c.starts_with(path_dir)) {
                        new_crates_io.insert(key.clone(), value.clone());
                    }
                }
            }
            
            *crates_io = new_crates_io;
        }
    }

    // Write the complete updated Cargo.toml
    let mut output_file = File::create("./output/Cargo.toml")?;
    write!(output_file, "{}", cargo_toml.to_string())?;

    println!("Successfully created new Cargo.toml with filtered members and patches.");
    Ok(())
} 