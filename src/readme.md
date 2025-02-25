Note: Follow [this link for instructions](https://github.com/farawaystar/agave-monitor/) to setup Github Actions

# agave-monitor

[![Crates.io](https://img.shields.io/crates/v/agave-monitor)](https://crates.io/crates/agave-monitor)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A Rust workspace analyzer for Solana's Agave validator client that generates a JSON dependency map of local packages.

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
cargo run --release -- ../agave-clone/agave
```
output will be in output/dependencies.json

### Advanced Options (optional)
```
# you can also specify custom output file
cargo run --release -- ../agave-clone/agave --output custom-deps.json

# Generate output in debug mode
RUST_LOG=debug cargo run -- ../agave-clone/agave
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
│   └── agave/                # clone of agave with all Cargo.toml files
└── agave-monitor/            # all build and run from here
    ├── output/               # output JSON file
    ├── src/
    │   └── main.rs           # Dependency extractor logic
    │   └── README.md         # This document
    ├── clone-agave.sh        # Repository cloning script
    ├── Cargo.toml            
    └── README.md             
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
```

## Contributing
Contributions welcome! Please follow these steps:
1. Fork the repository
2. Create a feature branch
3. Submit a pull request
4. Include tests for new functionality

## License
Apache License 2.0

Copyright [2024] [farawaystar]

Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0 Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
