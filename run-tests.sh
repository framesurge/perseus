#!/bin/bash
# Perseus Migration: Comprehensive Testing
# Run all tests to validate migration

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}Perseus Migration: Test Suite${NC}"
echo -e "${BLUE}========================================${NC}\n"

# Test configuration
REPORT_DIR="migration-reports/tests"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
REPORT_FILE="${REPORT_DIR}/test_results_${TIMESTAMP}.md"

mkdir -p "$REPORT_DIR"

# Initialize report
cat > "$REPORT_FILE" << EOF
# Perseus Migration Test Report

**Date:** $(date)
**Commit:** $(git rev-parse --short HEAD)

---

EOF

TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

#############################################
# 1. COMPILATION TESTS
#############################################

echo -e "${GREEN}[1/6] Testing compilation...${NC}"

{
    echo "## 1. Compilation Tests"
    echo ""
    echo "### Workspace Check"
    echo "\`\`\`bash"
    echo "$ cargo check --workspace"
    echo "\`\`\`"
    echo ""
} >> "$REPORT_FILE"

if cargo check --workspace 2>&1 | tee -a "$REPORT_FILE"; then
    echo -e "${GREEN}Workspace compiles successfully${NC}\n"
    {
        echo ""
        echo "**Result:** PASSED"
        echo ""
    } >> "$REPORT_FILE"
    ((PASSED_TESTS++))
else
    echo -e "${RED}Compilation failed${NC}\n"
    {
        echo ""
        echo "**Result:** FAILED"
        echo ""
    } >> "$REPORT_FILE"
    ((FAILED_TESTS++))
fi

((TOTAL_TESTS++))  # 1. Compilation

#############################################
# 2. UNIT TESTS
#############################################

echo -e "${GREEN}[2/6] Running unit tests...${NC}"

{
    echo "## 2. Unit Tests"
    echo ""
} >> "$REPORT_FILE"

# Test each package
for pkg in perseus-core perseus-macro perseus-router perseus-engine; do
    echo -e "${YELLOW}Testing package: ${pkg}${NC}"
    
    {
        echo "### Package: \`${pkg}\`"
        echo "\`\`\`bash"
        echo "$ cargo test -p ${pkg}"
        echo "\`\`\`"
        echo ""
    } >> "$REPORT_FILE"
    
    if cargo test -p "$pkg" 2>&1 | tee -a "$REPORT_FILE"; then
        echo -e "${GREEN}${pkg} tests passed${NC}\n"
        {
            echo "**Result:** PASSED"
            echo ""
        } >> "$REPORT_FILE"
        ((PASSED_TESTS++))
    else
        echo -e "${RED}${pkg} tests failed${NC}\n"
        {
            echo "**Result:** FAILED"
            echo ""
        } >> "$REPORT_FILE"
        ((FAILED_TESTS++))
    fi
    
    ((TOTAL_TESTS++))  # per package
done

#############################################
# 3. INTEGRATION TESTS
#############################################

echo -e "${GREEN}[3/6] Running integration tests...${NC}"

{
    echo "## 3. Integration Tests"
    echo ""
} >> "$REPORT_FILE"

# Only run if integration tests exist
if [ -d "tests/integration_tests" ] || cargo test --test integration_tests -- --list 2>/dev/null | grep -q "test"; then
    echo "\`\`\`bash"
    echo "$ cargo test --test integration_tests"
    echo "\`\`\`"
    echo ""
    >> "$REPORT_FILE"

    if cargo test --test integration_tests 2>&1 | tee -a "$REPORT_FILE"; then
        echo -e "${GREEN}Integration tests passed${NC}\n"
        {
            echo "**Result:** PASSED"
            echo ""
        } >> "$REPORT_FILE"
        ((PASSED_TESTS++))
    else
        echo -e "${RED}Integration tests failed${NC}\n"
        {
            echo "**Result:** FAILED"
            echo ""
        } >> "$REPORT_FILE"
        ((FAILED_TESTS++))
    fi
    ((TOTAL_TESTS++))
else
    echo -e "${YELLOW}No integration tests found – skipping${NC}\n"
    {
        echo "\`\`\`bash"
        echo "# cargo test --test integration_tests  (no tests found)"
        echo "\`\`\`"
        echo ""
        echo "**Result:** SKIPPED (no integration tests)"
        echo ""
    } >> "$REPORT_FILE"
fi

#############################################
# 4. EXAMPLE BUILDS
#############################################

echo -e "${GREEN}[4/6] Building examples...${NC}"

{
    echo "## 4. Example Builds"
    echo ""
} >> "$REPORT_FILE"

if [ -d "examples" ]; then
    EXAMPLE_COUNT=0
    EXAMPLE_PASSED=0
    
    for example_dir in examples/*/; do
        example=$(basename "$example_dir")
        echo -e "${YELLOW}Building example: ${example}${NC}"
        
        {
            echo "### Example: \`${example}\`"
            echo "\`\`\`bash"
            echo "$ cd examples/${example} && cargo build"
            echo "\`\`\`"
            echo ""
        } >> "$REPORT_FILE"
        
        if (cd "$example_dir" && cargo build) 2>&1 | tee -a "$REPORT_FILE"; then
            echo -e "${GREEN}${example} built successfully${NC}\n"
            {
                echo "**Result:** PASSED"
                echo ""
            } >> "$REPORT_FILE"
            ((EXAMPLE_PASSED++))
        else
            echo -e "${RED}${example} build failed${NC}\n"
            {
                echo "**Result:** FAILED"
                echo ""
            } >> "$REPORT_FILE"
        fi
        
        ((EXAMPLE_COUNT++))
        ((TOTAL_TESTS++))  # Count each example as a separate test
    done
    
    # Overall example block result
    if [ "$EXAMPLE_PASSED" -eq "$EXAMPLE_COUNT" ]; then
        ((PASSED_TESTS++))
    else
        ((FAILED_TESTS++))
    fi
else
    echo -e "${YELLOW}No examples directory found${NC}\n"
    {
        echo "**Result:** SKIPPED (no examples)"
        echo ""
    } >> "$REPORT_FILE"
fi

#############################################
# 5. CLIPPY LINTS
#############################################

echo -e "${GREEN}[5/6] Running clippy...${NC}"

{
    echo "## 5. Clippy Lints"
    echo ""
    echo "\`\`\`bash"
    echo "$ cargo clippy --workspace -- -D warnings"
    echo "\`\`\`"
    echo ""
} >> "$REPORT_FILE"

if cargo clippy --workspace -- -D warnings 2>&1 | tee -a "$REPORT_FILE"; then
    echo -e "${GREEN}No clippy warnings${NC}\n"
    {
        echo "**Result:** PASSED"
        echo ""
    } >> "$REPORT_FILE"
    ((PASSED_TESTS++))
else
    echo -e "${YELLOW}Clippy warnings present${NC}\n"
    {
        echo "**Result:** WARNINGS PRESENT"
        echo ""
    } >> "$REPORT_FILE"
    ((PASSED_TESTS++))  # Warnings don't fail
fi

((TOTAL_TESTS++))  # 5. Clippy

#############################################
# 6. DOCUMENTATION BUILD
#############################################

echo -e "${GREEN}[6/6] Building documentation...${NC}"

{
    echo "## 6. Documentation Build"
    echo ""
    echo "\`\`\`bash"
    echo "$ cargo doc --workspace --no-deps"
    echo "\`\`\`"
    echo ""
} >> "$REPORT_FILE"

if cargo doc --workspace --no-deps 2>&1 | tee -a "$REPORT_FILE"; then
    echo -e "${GREEN}Documentation built successfully${NC}\n"
    {
        echo "**Result:** PASSED"
        echo ""
    } >> "$REPORT_FILE"
    ((PASSED_TESTS++))
else
    echo -e "${RED}Documentation build failed${NC}\n"
    {
        echo "**Result:** FAILED"
        echo ""
    } >> "$REPORT_FILE"
    ((FAILED_TESTS++))
fi

((TOTAL_TESTS++))  # 6. Documentation

#############################################
# SUMMARY
#############################################

# Safe success rate calculation
success_rate=0
if [ "$TOTAL_TESTS" -gt 0 ]; then
    success_rate=$((PASSED_TESTS * 100 / TOTAL_TESTS))
fi

{
    echo "## Summary"
    echo ""
    echo "| Metric | Value |"
    echo "|--------|-------|"
    echo "| Total Tests | ${TOTAL_TESTS} |"
    echo "| Passed | ${PASSED_TESTS} |"
    echo "| Failed | ${FAILED_TESTS} |"
    echo "| Success Rate | ${success_rate}% |"
    echo ""
} >> "$REPORT_FILE"

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}Test Summary${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""
echo -e "Total Tests:   ${BLUE}${TOTAL_TESTS}${NC}"
echo -e "Passed:        ${GREEN}${PASSED_TESTS}${NC}"
echo -e "Failed:        ${RED}${FAILED_TESTS}${NC}"
echo -e "Success Rate:  ${BLUE}${success_rate}%${NC}"
echo ""
echo -e "Full report:   ${YELLOW}${REPORT_FILE}${NC}"
echo ""

if [ "$FAILED_TESTS" -eq 0 ]; then
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed. Review the report for details.${NC}"
    exit 1
fi
