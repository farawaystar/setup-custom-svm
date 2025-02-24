#!/bin/bash

# Configuration
output_file="combined.txt"
extensions=("json" "toml" "rs")  # Modify this array to change file types
top_directory="."  # Uses current working directory

# Get the name of the current folder
current_folder=$(basename "$(pwd)")

# Clear output file
> "$output_file"
current_date=$(date +"%A, %B %d, %Y, %I %p %Z")

# Use git ls-files to respect .gitignore
git ls-files "$top_directory" | grep -E "$(IFS=\|; echo "${extensions[*]}")$" | while read -r file; do
    echo "/* ------------------------------------------------" >> "$output_file"
    echo "My $current_folder/$file is as follows:" >> "$output_file"
    echo "--------------------------------------------------- */" >> "$output_file"
    echo "Current date: $current_date" >> "$output_file"
    echo "" >> "$output_file"
    cat "$file" >> "$output_file"
    echo "" >> "$output_file"
done

echo "Files concatenated: ${extensions[*]}"
echo "Output: $output_file"
