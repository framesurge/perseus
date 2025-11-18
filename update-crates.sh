#!/bin/bash
# Perseus Migration: Dependency Updates
# Phase 2: Update Sycamore to 0.9

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}Phase 2: Dependency Updates${NC}"
echo -e "${BLUE}========================================${NC}\n"

# Backup current state
echo -e "${GREEN}[1/5] Creating backup of current Cargo.toml files...${NC}"
find . -name "Cargo.toml" -type f -exec cp {} {}.backup \;
echo -e "${GREEN}✅ Backups created${NC}\n"

# Find all Cargo.toml files
CARGO_FILES=$(find . -name "Cargo.toml" -type f)

echo -e "${GREEN}[2/5] Updating Sycamore dependencies to 0.9...${NC}"

for file in $CARGO_FILES; do
    echo -e "${YELLOW}Updating: ${file}${NC}"
    
    # Update sycamore dependency
    if grep -q 'sycamore.*=.*"0\.8' "$file"; then
        sed -i.tmp 's/sycamore.*=.*"0\.8[^"]*"/sycamore = "0.9"/' "$file"
        echo -e "  ${GREEN}✓${NC} Updated sycamore to 0.9"
    fi
    
    # Update sycamore-web if present
    if grep -q 'sycamore-web.*=.*"0\.8' "$file"; then
        sed -i.tmp 's/sycamore-web.*=.*"0\.8[^"]*"/sycamore-web = "0.9"/' "$file"
        echo -e "  ${GREEN}✓${NC} Updated sycamore-web to 0.9"
    fi
    
    # Update sycamore-router if present
    if grep -q 'sycamore-router.*=.*"0\.8' "$file"; then
        sed -i.tmp 's/sycamore-router.*=.*"0\.8[^"]*"/sycamore-router = "0.9"/' "$file"
        echo -e "  ${GREEN}✓${NC} Updated sycamore-router to 0.9"
    fi
    
    # Update sycamore-reactive if present
    if grep -q 'sycamore-reactive.*=.*"0\.8' "$file"; then
        sed -i.tmp 's/sycamore-reactive.*=.*"0\.8[^"]*"/sycamore-reactive = "0.9"/' "$file"
        echo -e "  ${GREEN}✓${NC} Updated sycamore-reactive to 0.9"
    fi
    
    # Clean up temp files
    rm -f "${file}.tmp"
done

echo -e "\n${GREEN}✅ Dependency versions updated${NC}\n"

# Update Cargo.lock
echo -e "${GREEN}[3/5] Updating Cargo.lock...${NC}"
if cargo update -p sycamore 2>&1 | tee /tmp/cargo_update.log; then
    echo -e "${GREEN}✅ Cargo.lock updated successfully${NC}\n"
else
    echo -e "${RED}⚠️  Cargo update encountered issues (see /tmp/cargo_update.log)${NC}\n"
fi

# Check for dependency conflicts
echo -e "${GREEN}[4/5] Checking for dependency conflicts...${NC}"
if cargo tree -p perseus | grep -i sycamore; then
    echo -e "${YELLOW}Current sycamore dependencies:${NC}"
    cargo tree -p perseus | grep -i sycamore | sort -u
else
    echo -e "${YELLOW}No perseus package found, checking workspace...${NC}"
    cargo tree --workspace | grep -i sycamore | sort -u || echo "No sycamore dependencies found"
fi
echo ""

# Initial compilation check
echo -e "${GREEN}[5/5] Running initial compilation check...${NC}"
echo -e "${YELLOW}This will likely produce many errors - this is expected!${NC}\n"

# Create error log directory
mkdir -p migration-reports/errors

# Try to compile and capture errors
if cargo check --workspace 2>&1 | tee "migration-reports/errors/phase2_initial_errors.log"; then
    echo -e "\n${GREEN}🎉 Unexpected success! No compilation errors found.${NC}"
else
    echo -e "\n${YELLOW}⚠️  Compilation errors detected (as expected)${NC}"
    echo -e "${BLUE}Analyzing error patterns...${NC}\n"
    
    # Categorize errors
    SCOPE_ERRORS=$(grep -c "cannot find value \`cx\`" migration-reports/errors/phase2_initial_errors.log || echo "0")
    GENERIC_ERRORS=$(grep -c "cannot infer type for type parameter" migration-reports/errors/phase2_initial_errors.log || echo "0")
    LIFETIME_ERRORS=$(grep -c "expected.*lifetime" migration-reports/errors/phase2_initial_errors.log || echo "0")
    
    echo -e "${BLUE}Error Summary:${NC}"
    echo -e "- Scope-related errors: ${YELLOW}${SCOPE_ERRORS}${NC}"
    echo -e "- Generic type errors: ${YELLOW}${GENERIC_ERRORS}${NC}"
    echo -e "- Lifetime errors: ${YELLOW}${LIFETIME_ERRORS}${NC}"
    echo -e "\nFull error log: ${YELLOW}migration-reports/errors/phase2_initial_errors.log${NC}"
fi

echo ""
echo -e "${BLUE}========================================${NC}"
echo -e "${GREEN}✅ Phase 2 Complete!${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""
echo -e "${BLUE}What was done:${NC}"
echo -e "1. ✅ Backed up all Cargo.toml files"
echo -e "2. ✅ Updated Sycamore to version 0.9"
echo -e "3. ✅ Updated Cargo.lock"
echo -e "4. ✅ Documented initial errors"
echo ""
echo -e "${BLUE}Next Steps:${NC}"
echo -e "1. Review error log: ${YELLOW}cat migration-reports/errors/phase2_initial_errors.log${NC}"
echo -e "2. Begin code migration: ${YELLOW}./scripts/migrate-package.sh perseus-core${NC}"
echo -e "3. Or use batch replacements: ${YELLOW}./scripts/batch-replace.sh${NC}"
echo ""
echo -e "${YELLOW}To rollback:${NC}"
echo -e "  find . -name 'Cargo.toml.backup' -exec sh -c 'mv \"\$1\" \"\${1%.backup}\"' _ {} \\;"
echo ""
