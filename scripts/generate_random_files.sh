#!/bin/bash
# Root directory for test files (adjust to your EFS mount point)
ROOT_DIR="/mnt/efs-test"
TOTAL_FILES=1000000

# Create root directory if it doesn't exist
mkdir -p "$ROOT_DIR"

# Function to create random path up to 4 levels deep
random_path() {
  local depth=$((RANDOM % 4 + 1)) # 1 to 4 levels
  local path="$ROOT_DIR"
  for ((i=1; i<=depth; i++)); do
    path="$path/dir_$((RANDOM % 1000))"
  done
  echo "$path"
}

echo "Generating $TOTAL_FILES files in $ROOT_DIR..."
count=0

while [ $count -lt $TOTAL_FILES ]; do
  dir=$(random_path)
  mkdir -p "$dir"
  
  # Random file size between 1KB and 4KB
  size=$(( (RANDOM % 4 + 1) * 1024 ))
  
  # Unique filename
  file="$dir/file_${count}.bin"
  
  # Generate random content
  head -c $size /dev/urandom > "$file"
  
  count=$((count + 1))
  
  # Print progress every 10k files
  if (( count % 10000 == 0 )); then
    echo "$count files created..."
  fi
done

echo "Done! Generated $TOTAL_FILES files."
