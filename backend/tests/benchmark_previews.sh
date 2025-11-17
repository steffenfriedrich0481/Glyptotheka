#!/bin/bash
# Benchmark script for STL preview generation
# Tests preview generation with stl-thumb library integration

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
EXAMPLE_DIR="$PROJECT_ROOT/example"
CACHE_DIR="$PROJECT_ROOT/cache"
OUTPUT_FILE="$SCRIPT_DIR/benchmark_results.txt"

echo "STL Preview Benchmark"
echo "====================="
echo ""
echo "Testing preview generation with stl-thumb library integration"
echo "Cache directory: $CACHE_DIR"
echo "Example files: $EXAMPLE_DIR"
echo ""

# Clear cache to ensure fresh generation
if [ -d "$CACHE_DIR/previews" ]; then
    echo "Clearing preview cache..."
    rm -rf "$CACHE_DIR/previews"/*
    echo "Cache cleared."
    echo ""
fi

# Find up to 20 STL files for testing
echo "Finding STL files for testing..."
mapfile -t STL_FILES < <(find "$EXAMPLE_DIR" -name "*.stl" -type f | head -20)
NUM_FILES=${#STL_FILES[@]}

echo "Found $NUM_FILES STL files for benchmarking"
echo ""

# Initialize results
echo "Benchmark Results" > "$OUTPUT_FILE"
echo "=================" >> "$OUTPUT_FILE"
echo "Date: $(date)" >> "$OUTPUT_FILE"
echo "Files tested: $NUM_FILES" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

# Note: This script just prepares the test files
# Actual benchmarking will be done through API calls once backend is running
echo "Test files prepared:"
for i in "${!STL_FILES[@]}"; do
    file="${STL_FILES[$i]}"
    size=$(stat -c%s "$file" 2>/dev/null || stat -f%z "$file" 2>/dev/null || echo "0")
    size_mb=$(awk "BEGIN {printf \"%.2f\", $size / 1048576}")
    echo "$((i+1)). $(basename "$file") - ${size_mb}MB"
    echo "$((i+1)). $file - ${size_mb}MB" >> "$OUTPUT_FILE"
done

echo ""
echo "Benchmark file list saved to: $OUTPUT_FILE"
echo ""
echo "To run benchmark:"
echo "1. Start the backend server (cargo run)"
echo "2. Configure root path to point to example/"
echo "3. Run scan to generate previews"
echo "4. Check generation times in logs"
