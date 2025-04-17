# run this from setup-custom-svm root

# check if package was given as input
package="$1"
if [ "$#" -eq 0 ]; then
    echo "Error: Package name required" >&2
    echo "Usage: $0 <custom-package-name>"
    exit 1
fi
TURQUOISE='\033[38;5;45m'
BOLD_GREEN='\033[1;32m'
RESET='\033[0m'

# STEP 1: create light clone of just Cargo.toml files
printf "${TURQUOISE}Step 1: create light clone${RESET}\n"
chmod +x clone-agave.sh
./clone-agave.sh
echo "connected to agave client. Light clone of just Cargo.toml files created"

# STEP 2: build setup-custom-svm repo
printf "${TURQUOISE}Step 2: build setup-custom-svm repo${RESET}\n"
cargo build --release

# STEP 3: run setup-custom-svm. extract dependencies
printf "${TURQUOISE}Step 3: extract output/dependencies.json${RESET}\n"
cargo run --bin extract_packages ../agave-clone/agave
echo "all dependencies extracted in dependencies.json"

# STEP 4: create sparse checkout git command
printf "${TURQUOISE}Step 4: create git sparse-checkout command for package: ${package} ${RESET}\n"
cargo run --bin create-git-command "$package"

# STEP 5: generate new Cargo.toml file
printf "${TURQUOISE}Step 5: generate new Cargo.toml${RESET}\n"
cargo run --bin update_cargo_toml  

# STEP 6: Clone the repository with --filter=blob:none to avoid downloading large files
printf "${TURQUOISE}Step 6: prepare agave clone with --filer=blob:none. Output in agave-sparse folder${RESET}\n"
cd ..
git clone --filter=blob:none https://github.com/anza-xyz/agave.git agave-sparse
cd agave-sparse

# Enable sparse checkout
git sparse-checkout init --cone

# STEP 7: Set up sparse-checkout to include only the specified folders
printf "${TURQUOISE}Step 7: applying git sparse-checkout${RESET}\n"

# Validate command file exists
if [ ! -f "../setup-custom-svm/output/sparse_checkout_command.sh" ]; then
    echo "Error: sparse_checkout_command.sh not found in setup-custom-svm/output directory" >&2
    exit 1
fi

# Read command file and execute in current context
sparse_command=$(<../setup-custom-svm/output/sparse_checkout_command.sh)
printf "${sparse_command}"

eval "$sparse_command"

# STEP 8: replace Cargo.toml
printf "\n${TURQUOISE}Step 8: replace Cargo.toml${RESET}\n"
echo "replace Cargo.toml"
pwd
echo "copying new Cargo.toml"
cp ../setup-custom-svm/output/Cargo.toml .

echo "Verifying copy..."
diff ../setup-custom-svm/output/Cargo.toml Cargo.toml && echo "Copy successful: Files are identical" || echo "Copy failed: Files differ"
pwd

# Remove the remote to detach from the original agave repo
git remote remove origin

# STEP 9: build the package
printf "${TURQUOISE}Step 9: build agave-spare for package: ${package}${RESET}\n"
# Get all package names in workspace
package_list=$(cargo metadata --format-version=1 | jq -r '.packages[].name')

# Check if package exists
if ! grep -qxF "$package" <<< "$package_list"; then
    echo "Error: Package '$package' not found in workspace" >&2
    echo "Available packages:" >&2
    echo "$package_list" | sed 's/^/- /' >&2
    exit 1
fi
# Proceed with build
cargo build --lib --package "$package"

# STEP 10: Clean up: Remove the temporary agave-clone directory
printf "${TURQUOISE}Step 10: clean up${RESET}\n"
cd ..
rm -rf agave-clone
printf "${BOLD_GREEN}END: Successfully created independent repository in 'agave-sparse' directory for package: ${package}${RESET}\n"
