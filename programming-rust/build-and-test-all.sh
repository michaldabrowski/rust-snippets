#!/bin/bash

# Script to build and test all Programming Rust snippets
# Usage: ./build-and-test-all.sh

set -e  # Exit on error

# Color codes for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Get the directory where the script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}Programming Rust - Build and Test All${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# Find all directories containing Cargo.toml
PROJECTS=()
for dir in "$SCRIPT_DIR"/*/; do
    if [ -f "$dir/Cargo.toml" ]; then
        PROJECTS+=("$dir")
    fi
done

# Sort projects for consistent ordering
IFS=$'\n' PROJECTS=($(sort <<<"${PROJECTS[*]}"))
unset IFS

TOTAL=${#PROJECTS[@]}
PASSED=0
FAILED=0
FAILED_PROJECTS=()

echo -e "Found ${YELLOW}${TOTAL}${NC} projects to build and test"
echo ""

# Process each project
for i in "${!PROJECTS[@]}"; do
    PROJECT_DIR="${PROJECTS[$i]}"
    PROJECT_NAME=$(basename "$PROJECT_DIR")
    NUM=$((i + 1))

    echo -e "${BLUE}[$NUM/$TOTAL]${NC} ${YELLOW}Testing: ${PROJECT_NAME}${NC}"
    echo -e "${BLUE}========================================${NC}"

    cd "$PROJECT_DIR"

    # Test the project (this also builds it)
    if cargo test --verbose; then
        echo -e "${GREEN}✓ Build and tests passed${NC}"
        PASSED=$((PASSED + 1))
    else
        echo -e "${RED}✗ Build or tests failed${NC}"
        FAILED=$((FAILED + 1))
        FAILED_PROJECTS+=("$PROJECT_NAME")
    fi

    echo ""
    echo -e "${BLUE}========================================${NC}"
    echo ""
done

# Summary
echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}SUMMARY${NC}"
echo -e "${BLUE}========================================${NC}"
echo -e "Total projects: ${YELLOW}${TOTAL}${NC}"
echo -e "Passed: ${GREEN}${PASSED}${NC}"
echo -e "Failed: ${RED}${FAILED}${NC}"

if [ ${FAILED} -gt 0 ]; then
    echo ""
    echo -e "${RED}Failed projects:${NC}"
    for project in "${FAILED_PROJECTS[@]}"; do
        echo -e "  ${RED}✗${NC} $project"
    done
    exit 1
else
    echo ""
    echo -e "${GREEN}All projects built and tested successfully! 🎉${NC}"
    exit 0
fi
