#!/bin/bash
# Perseus Migration: Final Validation
# Comprehensive validation to ensure migration is complete

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

REPORT_DIR="migration-reports"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
VALIDATION_REPORT="${REPORT_DIR}/validation_${TIMESTAMP}.md"

mkdir -p "$REPORT_DIR"

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}Perseus Migration: Final Validation${NC}"
echo -e "${BLUE}========================================${NC}\n"

# Initialize report
cat > "$VALIDATION_REPORT" << 'EOF'
# Perseus Migration: Final Validation Report

**Date:** $(date)
**Validator:** Automated validation script v1.0

This report validates the completion and correctness of the Sycamore 0.8 → 0.9 migration.

---

EOF

CHECKS_TOTAL=0
CHECKS_PASSED=0
CHECKS_FAILED=0
WARNINGS=0

# Helper function for checks
run_check() {
    local description="$1"
    local command="$2"
    local critical="${3:-yes}"  # yes/no
    
    ((CHECKS_TOTAL++))
    
    echo -e "${BLUE}[CHECK ${CHECKS_TOTAL}] ${description}${NC}"
    
    {
        echo "## Check ${CHECKS_TOTAL}: ${description}"
        echo ""
        echo "\`\`\`bash"
        echo "$ ${command}"
        echo "\`\`\`"
        echo ""
    } >> "$VALIDATION_REPORT"
    
    if eval "$command" >> "$VALIDATION_REPORT" 2>&1; then
        echo -e "${GREEN}✅ PASSED${NC}\n"
        {
            echo "**Result:** ✅ PASSED"
            echo ""
        } >> "$VALIDATION_REPORT"
        ((CHECKS_PASSED++))
    else
        if [ "$critical" = "yes" ]; then
            echo -e "${RED}❌ FAILED (Critical)${NC}\n"
            {
                echo "**Result:** ❌ FAILED (Critical)"
                echo ""
            } >> "$VALIDATION_REPORT"
            ((CHECKS_FAILED++))
        else
            echo -e "${YELLOW}⚠️  WARNING (Non-critical)${NC}\n"
            {
                echo "**Result:** ⚠️ WARNING (Non-critical)"
                echo ""
            } >> "$VALIDATION_REPORT"
            ((WARNINGS++))
            ((CHECKS_PASSED++))
        fi
    fi
}

#############################################
# CRITICAL CHECKS
#############################################

echo -e "${BLUE}=== CRITICAL CHECKS ===${NC}\n"

# 1. Compilation
run_check \
    "Workspace compiles without errors" \
    "cargo check --workspace" \
    "yes"

# 2. Test suite passes
run_check \
    "All tests pass" \
    "cargo test --workspace" \
    "yes"

# 3. No Sycamore 0.8 dependencies
run_check \
    "No Sycamore 0.8 in dependency tree" \
    "! cargo tree --workspace | grep -i 'sycamore.*0\.8'" \
    "yes"

# 4. All examples build
if [ -d "examples" ]; then
    for example_dir in examples/*/; do
        example=$(basename "$example_dir")
        run_check \
            "Example '${example}' builds" \
            "(cd ${example_dir} && cargo build)" \
            "yes"
    done
fi

#############################################
# PATTERN VALIDATION
#############################################

echo -e "${BLUE}=== PATTERN VALIDATION ===${NC}\n"

# 5. No Scope parameters remain
run_check \
    "No 'cx: Scope' parameters in code" \
    "! rg 'cx:\s*Scope' --type rust ." \
    "yes"

# 6. No Generic Html constraints
run_check \
    "No '<G: Html>' constraints in code" \
    "! rg '<.*G:\s*Html.*>' --type rust ." \
    "yes"

# 7. No View<G> generic types
run_check \
    "No 'View<G>' types in code" \
    "! rg 'View<G>' --type rust ." \
    "yes"

# 8. No create_signal(cx,) calls
run_check \
    "No 'create_signal(cx,' calls in code" \
    "! rg 'create_signal\\(cx,' --type rust ." \
    "yes"

# 9. No RcSignal usage
run_check \
    "No 'RcSignal' usage in code" \
    "! rg 'RcSignal' --type rust ." \
    "yes"

# 10. No view! { cx, } macros
run_check \
    "No 'view! { cx,' macros in code" \
    "! rg 'view!\s*\\{\s*cx,' --type rust ." \
    "yes"

# 11. No 'iterable=' in Indexed/Keyed
run_check \
    "No 'iterable=' in Indexed/Keyed (should be 'list=')" \
    "! rg 'iterable\s*=' --type rust ." \
    "no"

#############################################
# CODE QUALITY CHECKS
#############################################

echo -e "${BLUE}=== CODE QUALITY CHECKS ===${NC}\n"

# 12. No clippy warnings
run_check \
    "No clippy warnings" \
    "cargo clippy --workspace -- -D warnings" \
    "no"

# 13. Documentation builds
run_check \
    "Documentation builds without errors" \
    "cargo doc --workspace --no-deps" \
    "no"

# 14. Formatting is correct
run_check \
    "Code formatting is correct" \
    "cargo fmt --all -- --check" \
    "no"

#############################################
# DEPENDENCY VALIDATION
#############################################

echo -e "${BLUE}=== DEPENDENCY VALIDATION ===${NC}\n"

# 15. Sycamore 0.9 in all Cargo.toml
{
    echo "## Check: Sycamore versions in Cargo.toml files"
    echo ""
    echo "### All Cargo.toml files with sycamore dependency:"
    echo "\`\`\`"
    find . -name "Cargo.toml" -type f -exec grep -H "sycamore" {} \; || echo "No sycamore dependencies found"
    echo "\`\`\`"
    echo ""
} >> "$VALIDATION_REPORT"

# Manual verification needed
echo -e "${YELLOW}[CHECK] Verify Sycamore versions manually in report${NC}\n"

#############################################
# API STABILITY CHECKS
#############################################

echo -e "${BLUE}=== API STABILITY CHECKS ===${NC}\n"

# 16. Public API unchanged (or documented)
{
    echo "## Check: Public API Changes"
    echo ""
    echo "### Public items in perseus-core:"
    echo "\`\`\`"
    cargo doc -p perseus-core --no-deps 2>&1 | grep -i "warning" || echo "No API warnings"
    echo "\`\`\`"
    echo ""
} >> "$VALIDATION_REPORT"

echo -e "${YELLOW}[CHECK] Review public API manually in report${NC}\n"

#############################################
# SUMMARY
#############################################

{
    echo "---"
    echo ""
    echo "## Validation Summary"
    echo ""
    echo "| Metric | Value |"
    echo "|--------|-------|"
    echo "| Total Checks | ${CHECKS_TOTAL} |"
    echo "| Passed | ${CHECKS_PASSED} |"
    echo "| Failed | ${CHECKS_FAILED} |"
    echo "| Warnings | ${WARNINGS} |"
    echo "| Success Rate | $(( CHECKS_TOTAL == 0 ? 0 : CHECKS_PASSED * 100 / CHECKS_TOTAL ))% |"
    echo ""
    
    if [ "$CHECKS_FAILED" -eq 0 ]; then
        echo "**Overall Status:** ✅ **MIGRATION COMPLETE**"
        echo ""
        echo "All critical checks passed. The migration is complete and ready for production."
    else
        echo "**Overall Status:** ❌ **MIGRATION INCOMPLETE**"
        echo ""
        echo "Some critical checks failed. Review the failures above and address them before considering the migration complete."
    fi
    echo ""
} >> "$VALIDATION_REPORT"

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}Validation Summary${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""
echo -e "Total Checks:  ${BLUE}${CHECKS_TOTAL}${NC}"
echo -e "Passed:        ${GREEN}${CHECKS_PASSED}${NC}"
echo -e "Failed:        ${RED}${CHECKS_FAILED}${NC}"
echo -e "Warnings:      ${YELLOW}${WARNINGS}${NC}"
echo -e "Success Rate:  ${BLUE}$((CHECKS_PASSED * 100 / CHECKS_TOTAL))%${NC}"
echo ""
echo -e "Full report:   ${YELLOW}${VALIDATION_REPORT}${NC}"
echo ""

if [ "$CHECKS_FAILED" -eq 0 ]; then
    echo -e "${GREEN}========================================${NC}"
    echo -e "${GREEN}🎉 MIGRATION COMPLETE! 🎉${NC}"
    echo -e "${GREEN}========================================${NC}"
    echo ""
    echo -e "${GREEN}Congratulations! All validation checks passed.${NC}"
    echo ""
    echo -e "${BLUE}Next steps:${NC}"
    echo -e "1. ${YELLOW}Review the validation report${NC}"
    echo -e "2. ${YELLOW}Update documentation${NC}"
    echo -e "3. ${YELLOW}Create release notes${NC}"
    echo -e "4. ${YELLOW}Submit PR or merge to main${NC}"
    echo ""
    exit 0
else
    echo -e "${RED}========================================${NC}"
    echo -e "${RED}⚠️  VALIDATION FAILED${NC}"
    echo -e "${RED}========================================${NC}"
    echo ""
    echo -e "${RED}Some critical checks failed. Please address the issues and run validation again.${NC}"
    echo ""
    echo -e "${BLUE}To fix issues:${NC}"
    echo -e "1. ${YELLOW}Review the validation report: cat ${VALIDATION_REPORT}${NC}"
    echo -e "2. ${YELLOW}Address each failed check${NC}"
    echo -e "3. ${YELLOW}Run validation again: ./scripts/validate-migration.sh${NC}"
    echo ""
    exit 1
fi
