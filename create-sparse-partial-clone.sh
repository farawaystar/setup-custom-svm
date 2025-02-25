# STEP 1: create light clone of just Cargo.toml files
./clone-agave.sh
echo "connected to agave client. Light clone of just Cargo.toml files created"

# STEP 2: build agave-monitor repo
cargo build --release

# STEP 3: run agave-monitor. extract dependencies
cargo run --bin extract_packages ../agave-clone/agave
echo "all dependencies extracted in dependencies.json"

# STEP 4: create sparse checkout git command
cargo run --bin create-git-command solana-svm

# STEP 5: generate new Cargo.toml file
cargo run --bin update_cargo_toml  

# STEP 6: Clone the repository with --filter=blob:none to avoid downloading large files
cd ..
git clone --filter=blob:none https://github.com/anza-xyz/agave.git agave-sparse
cd agave-sparse

# Enable sparse checkout
git sparse-checkout init --cone

# STEP 7: Set up sparse-checkout to include only the specified folders

git sparse-checkout set \
    loader-v4-program \
    programs/system \
    compute-budget-program \
    poseidon \
    svm-conformance \
    curve25519 \
    svm \
    type-overrides \
    programs/loader-v4 \
    builtins-default-costs \
    svm-rent-collector \
    svm-transaction \
    programs/config \
    programs/address-lookup-table \
    curves/curve25519 \
    compute-budget \
    timings \
    program-runtime \
    programs/compute-budget \
    vote-program \
    measure \
    metrics \
    compute-budget-instruction \
    system-program \
    programs/stake \
    log-collector \
    programs/bpf_loader \
    config-program \
    address-lookup-table-program \
    programs/vote \
    bpf-loader-program \
    stake-program \


# STEP 8: replace Cargo.toml
echo "replace Cargo.toml"
pwd
echo "copying new Cargo.toml"
cp ../agave-monitor/output/Cargo.toml .

echo "Verifying copy..."
diff ../agave-monitor/output/Cargo.toml Cargo.toml && echo "Copy successful: Files are identical" || echo "Copy failed: Files differ"
pwd

# Remove the remote to detach from the original agave repo
git remote remove origin

# STEP 9: build the package
cargo build --lib --package solana-svm


# Checkout the main branch (or whichever branch you need)
# git checkout master

# Create a new independent repository
# cd ..
# mkdir agave-independent-s
# cd agave-independent-s
# git init

# Pull in the changes from the sparse content
# git remote add origin ../agave-sparse
# git pull origin master

# STEP 10: Clean up: Remove the temporary agave-clone directory
cd ..
rm -rf agave-clone

echo "Independent repository created in 'agave-sparse' directory."
