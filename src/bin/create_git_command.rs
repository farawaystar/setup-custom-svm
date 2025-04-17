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
      // Get package names from command line arguments
      let args: Vec<String> = env::args().collect();
      if args.len() < 2 {
          eprintln!("Please provide one or more package names, separated by spaces");
          std::process::exit(1);
      }
  
      let packages = &args[1..];
  
      // Get the absolute path to the workspace directory
      let workspace_dir = env::current_exe()
          .expect("Failed to get executable path")
          .parent()
          .expect("Failed to get parent directory")
          .parent()
          .expect("Failed to get parent directory")
          .parent()
          .expect("Failed to get workspace directory")
          .to_path_buf();
  
      let json_path = workspace_dir.join("output").join("dependencies.json");
      
      let file_content = fs::read_to_string(&json_path)
          .expect(&format!("Failed to read file: {}", json_path.display()));
      let dependencies: HashMap<String, Module> =
          serde_json::from_str(&file_content).expect("Failed to parse JSON");
  
      // Check if all packages exist
      for package_name in packages {
          if !dependencies.contains_key(package_name) {
              eprintln!("Package '{}' not found", package_name);
              std::process::exit(1);
          }
      }
  
      let mut collected_deps = HashSet::new();
      
      // Collect dependencies for all packages
      for package_name in packages {
          collect_dependencies(package_name, &dependencies, &mut collected_deps);
          
          // Add the package's own path
          if let Some(module_data) = dependencies.get(package_name) {
              collected_deps.insert(module_data.path.clone());
          }
      }
  
      // Create output file
      let output_path = workspace_dir.join("output").join("sparse_checkout_command.sh");
      let mut output_file = fs::File::create(&output_path)
          .expect(&format!("Failed to create output file: {}", output_path.display()));
  
      // Write the git command
      writeln!(output_file, "git sparse-checkout set \\").expect("Failed to write to file");
  
      // Write paths to the output file
      let paths: Vec<_> = collected_deps.into_iter().collect();
      for (i, path) in paths.iter().enumerate() {
          if i == paths.len() - 1 {
              writeln!(output_file, "    {}", path).expect("Failed to write path");
          } else {
              writeln!(output_file, "    {} \\", path).expect("Failed to write path");
          }
      }
  
      println!("Generated sparse checkout command in sparse_checkout_command.sh");
  }