use std::{
  collections::{HashMap, HashSet},
  fs,
  io::Write,
  env,
};

/// Represents a Solana module with its path and dependencies
#[derive(Debug, serde::Deserialize)]
struct Module {
  path: String,
  dependencies: HashMap<String, String>,
  dev_dependencies: HashMap<String, String>,
}

const CLONE_COMMAND_STR: &str = "git clone --filter=blob:none https://github.com/anza-xyz/agave.git && cd agave && ";
/// Collects all dependencies for a given module recursively
/// 
/// # Arguments
/// * `module` - The name of the module to collect dependencies for
/// * `dependencies` - HashMap containing all module information
/// * `collected_deps` - Set to store collected dependency paths
fn collect_dependencies(
  module: &str,
  dependencies: &HashMap<String, Module>,
  collected_deps: &mut HashSet<String>,
) {
  // Skip if module name contains dots (relative paths)
  if module.contains(".") {
      println!("Skipping module with name '.' or '..'");
      return;
  }

  // Add the module name without the "solana-" prefix to collected dependencies
  collected_deps.insert(module.replace("solana-", "").to_string());

  if let Some(module_data) = dependencies.get(module) {

      // Add regular dependencies
      for (key, value) in &module_data.dependencies {
          if collected_deps.insert(value.clone()) {
              collect_dependencies(key, dependencies, collected_deps);
          }
      }

      // Add development dependencies
      for (key, value) in &module_data.dev_dependencies {
          
          if !value.contains(".") && collected_deps.insert(value.clone()) {
              collect_dependencies(key, dependencies, collected_deps);
          }
          
      }
  }
}

fn main() {
  // Get package name from command line arguments
  let args: Vec<String> = env::args().collect();
  let package_name = args.get(1).expect("Please provide a package name");

  let file_content = fs::read_to_string("./output/dependencies.json")
      .expect("Failed to read file");
  let dependencies: HashMap<String, Module> =
      serde_json::from_str(&file_content).expect("Failed to parse JSON");

  // Check if the package exists
  if !dependencies.contains_key(package_name) {
      eprintln!("Package '{}' not found", package_name);
      std::process::exit(1);
  }

  let mut collected_deps = HashSet::new();
  collect_dependencies(package_name, &dependencies, &mut collected_deps);
  
  // Add the package's own path
  if let Some(module_data) = dependencies.get(package_name) {
      collected_deps.insert(module_data.path.clone());
  }

  // Create output file
  let mut output_file = fs::File::create("./output/filter-repo-command.sh")
      .expect("Failed to create output file");

  // Write the git command
  writeln!(output_file, "{}{}", CLONE_COMMAND_STR, "git filter-repo \\").expect("Failed to write to file");

  // Write paths to the output file
  for path in collected_deps {
      writeln!(output_file, "   --path {} \\", path).expect("Failed to write path");
  }

  println!("Generated filter-repo command in filter-repo-command.sh");
}
