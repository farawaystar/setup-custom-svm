# Rust dependency generator

A dependency generator for Rust workspaces of any size. Tested on large workspaces like Solana's Agave validator client. Generates a JSON dependency map of local packages.


## Features

- Analyzes Cargo.toml files in a Rust workspace. In this case [Solana Agave repo](https://github.com//agave-monitor.git)
- Generates structured JSON output with:
  - Package paths
  - Dependencies
  - Development dependencies
- Handles workspace inheritance

## Installation

### step 1: clone all Cargo.toml files from agave repo
```
# Clone Agave repository
./clone-agave.sh
```

### step 2: clone this repo and build
```
git clone https://github.com//agave-monitor.git
cd agave-monitor
cargo build --release
```

### step 3: run
```
cargo run --bin extract_packages ../agave-clone/agave
```
output will be in output/dependencies.json

### Advanced Options (optional)
```
# you can also specify custom output file
cargo run --bin extract_packages ../agave-clone/agave --output custom-deps.json
```

## Output Format
Output files are saved to `output/` directory. Example JSON structure:
```
{
  "package-name": {
    "path": "relative/path/from/workspace",
    "dependencies": {
      "local-dep": "./path/to/dependency"
    },
    "dev_dependencies": {
      "dev-dep": "./path/to/dev-dependency"
    }
  }
}
```

## Project Structure
```
parent-folder/
├── agave-clone/
│   └── agave/                     # clone of agave with all Cargo.toml files
└── agave-monitor/                 # build and run from here
    ├── output/                    # output JSON file
    ├── src/bin
    │   └── extract_packages.rs    # Dependency extractor logic
    └── clone-agave.sh             # Repository cloning script
           
```


## Troubleshooting

### Common Issues
- **Missing Cargo.toml files**: Run `./clone-agave.sh` to refresh repository
- **Empty JSON output**: Verify Agave repository contains valid workspace members
- **Path errors**: Use absolute paths for workspace directory

``` 
refresh agave-clone
rm -rf ../agave-clone
./clone-agave.sh

```

```
if issue with giving path of agave-clone, set the below once and run
FULL_PATH="$(cd ../agave-clone/agave && pwd)"
cargo run --release -- "$FULL_PATH"
cargo run --bin extract_packages "$FULL_PATH" --output custom-deps.json
```

