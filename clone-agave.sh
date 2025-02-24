#!/bin/bash
# clone-agave.sh

# Create target directory outside analyzer repo
mkdir -p ../agave-clone
cd ../agave-clone || exit 1

# Clone with sparse checkout
git clone \
  --depth 1 \
  --filter=blob:none \
  --sparse \
  https://github.com/anza-xyz/agave.git

cd agave || exit 1

git sparse-checkout init --no-cone
git sparse-checkout set \
  'Cargo.toml' \          # Root manifest
  '/*/Cargo.toml' \       # First-level subdirectories
  '/**/Cargo.toml'        # All nested directories

# Initialize workspace members
git submodule update --init --recursive --depth 1

echo "Agave Cargo.toml files cloned to: $(pwd)"

