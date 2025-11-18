# Perseus Migration: Comprehensive Codebase Analysis
# Phase 1: Discovery & Analysis

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
REPORT_DIR="migration-reports"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
REPORT_FILE="${REPORT_DIR}/analysis_${TIMESTAMP}.md"

# Ensure report directory exists
mkdir -p "$REPORT_DIR"

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}Perseus Sycamore 0.8 → 0.9 Migration${NC}"
echo -e "${BLUE}Phase 1: Codebase Analysis${NC}"
echo -e "${BLUE}========================================${NC}\n"

# Initialize report
cat >"$REPORT_FILE" <<'EOF'
# Perseus Migration Analysis Report

**Generated:** $(date)
**Analysis Tool:** perseus-migration-workflow v1.0

---

EOF

#############################################
# 1. DEPENDENCY AUDIT
#############################################

echo -e "${GREEN}[1/7] Analyzing Cargo dependencies...${NC}"

{
    echo "## 1. Dependency Audit"
    echo ""
    echo "### Current Sycamore Dependencies"
    echo "\`\`\`"
    cargo tree --workspace -p perseus | grep -i sycamore || echo "No sycamore dependencies found in perseus"
    cargo tree --workspace | grep -i sycamore || echo "No sycamore dependencies found"
    echo "\`\`\`"
    echo ""

    echo "### All Cargo.toml Files"
    echo "\`\`\`"
    find . -name "Cargo.toml" -type f
    echo "\`\`\`"
    echo ""
} >>"$REPORT_FILE"

#############################################
# 2. CODE PATTERN ANALYSIS
#############################################

echo -e "${GREEN}[2/7] Scanning for Sycamore 0.8 patterns...${NC}"

# Calculate counts first (outside the redirected block so variables persist)
# FIXED: Changed `awk '{s+=$1} END {print s}' || echo "0"` to `awk '{s+=$1} END {print s+0}'`
SCOPE_COUNT=$(rg "cx:\s*Scope" --type rust -c 2>/dev/null | awk -F: '{s+=$2} END {print s+0}')
HTML_GENERIC_COUNT=$(rg "<.*G:\s*Html.*>" --type rust -c 2>/dev/null | awk -F: '{s+=$2} END {print s+0}')
VIEW_GENERIC_COUNT=$(rg "View<G>" --type rust -c 2>/dev/null | awk -F: '{s+=$2} END {print s+0}')
SIGNAL_CX_COUNT=$(rg "create_signal\(cx," --type rust -c 2>/dev/null | awk -F: '{s+=$2} END {print s+0}')
RC_SIGNAL_COUNT=$(rg "RcSignal" --type rust -c 2>/dev/null | awk -F: '{s+=$2} END {print s+0}')
VIEW_MACRO_COUNT=$(rg "view!\s*\\{\s*cx," --type rust -c 2>/dev/null | awk -F: '{s+=$2} END {print s+0}')
ITERABLE_COUNT=$(rg "iterable\s*=" --type rust -c 2>/dev/null | awk -F: '{s+=$2} END {print s+0}')

{
    echo "## 2. Sycamore 0.8 Pattern Detection"
    echo ""

    # Scope parameters
    echo "### Scope Parameters: **${SCOPE_COUNT}** occurrences"
    echo "\`\`\`"
    rg "cx:\s*Scope" --type rust -n --heading 2>/dev/null || echo "No scope parameters found"
    echo "\`\`\`"
    echo ""

    # Generic Html constraints
    echo "### Generic Html Constraints: **${HTML_GENERIC_COUNT}** occurrences"
    echo "\`\`\`"
    rg "<.*G:\s*Html.*>" --type rust -n --heading 2>/dev/null || echo "No Html generic constraints found"
    echo "\`\`\`"
    echo ""

    # View<G> usage
    echo "### View<G> Generic Types: **${VIEW_GENERIC_COUNT}** occurrences"
    echo "\`\`\`"
    rg "View<G>" --type rust -n --heading 2>/dev/null || echo "No View<G> found"
    echo "\`\`\`"
    echo ""

    # Signal creation with cx
    echo "### create_signal(cx, ...) Calls: **${SIGNAL_CX_COUNT}** occurrences"
    echo "\`\`\`"
    rg "create_signal\(cx," --type rust -n --heading 2>/dev/null || echo "No create_signal(cx,) calls found"
    echo "\`\`\`"
    echo ""

    # RcSignal usage
    echo "### RcSignal Usage: **${RC_SIGNAL_COUNT}** occurrences"
    echo "\`\`\`"
    rg "RcSignal" --type rust -n --heading 2>/dev/null || echo "No RcSignal found"
    echo "\`\`\`"
    echo ""

    # View macro with cx
    echo "### view! { cx, ... } Macros: **${VIEW_MACRO_COUNT}** occurrences"
    echo "\`\`\`"
    rg "view!\s*\\{\s*cx," --type rust -n --heading 2>/dev/null || echo "No view! macros with cx found"
    echo "\`\`\`"
    echo ""

    # Indexed/Keyed iterable
    echo "### Indexed/Keyed iterable=: **${ITERABLE_COUNT}** occurrences"
    echo "\`\`\`"
    rg "iterable\s*=" --type rust -n --heading 2>/dev/null || echo "No iterable= found"
    echo "\`\`\`"
    echo ""

} >>"$REPORT_FILE"

#############################################
# 3. FILE-LEVEL BREAKDOWN
#############################################

echo -e "${GREEN}[3/7] Analyzing file-level distribution...${NC}"

{
    echo "## 3. File-Level Impact Analysis"
    echo ""
    echo "### Files with Sycamore 0.8 Patterns"
    echo ""
    echo "| File | Scope | Generics | Signals | View Macros | Total |"
    echo "|------|-------|----------|---------|-------------|-------|"

    find . -name "*.rs" -type f | while read -r file; do
        scope=$(rg "cx:\s*Scope" "$file" -c 2>/dev/null || echo "0")
        generics=$(rg "<.*G:\s*Html.*>" "$file" -c 2>/dev/null || echo "0")
        signals=$(rg "create_signal\(cx," "$file" -c 2>/dev/null || echo "0")
        views=$(rg "view!\s*\\{\s*cx," "$file" -c 2>/dev/null || echo "0")
        total=$((scope + generics + signals + views))

        if [ "$total" -gt 0 ]; then
            echo "| \`${file}\` | ${scope} | ${generics} | ${signals} | ${views} | **${total}** |"
        fi
    done

    echo ""
} >>"$REPORT_FILE"

#############################################
# 4. MODULE COMPLEXITY ASSESSMENT
#############################################

echo -e "${GREEN}[4/7] Assessing module complexity...${NC}"

{
    echo "## 4. Module Complexity Assessment"
    echo ""

    # Core packages analysis
    for pkg in perseus-core perseus-macro perseus-router perseus-engine perseus-warp perseus-axum; do
        if [ -d "packages/${pkg}" ]; then
            echo "### Package: \`${pkg}\`"

            total_files=$(find "packages/${pkg}" -name "*.rs" -type f | wc -l)
            affected_files=$(find "packages/${pkg}" -name "*.rs" -type f -exec rg -l "cx:\s*Scope" {} \; | wc -l)

            echo "- **Total Rust files:** ${total_files}"
            echo "- **Affected files:** ${affected_files}"

            if [ "$total_files" -gt 0 ]; then
                percentage=$((affected_files * 100 / total_files))
                echo "- **Impact:** ${percentage}% of files affected"
            fi

            # Estimate complexity
            pattern_count=$(find "packages/${pkg}" -name "*.rs" -type f -exec rg "cx:\s*Scope|<.*G:\s*Html|create_signal\(cx," {} \; | wc -l)

            if [ "$pattern_count" -gt 100 ]; then
                echo "- **Complexity:** 🔴 HIGH (${pattern_count} patterns)"
                echo "- **Estimated Effort:** 6-8 hours"
            elif [ "$pattern_count" -gt 30 ]; then
                echo "- **Complexity:** 🟡 MEDIUM (${pattern_count} patterns)"
                echo "- **Estimated Effort:** 3-5 hours"
            else
                echo "- **Complexity:** 🟢 LOW (${pattern_count} patterns)"
                echo "- **Estimated Effort:** 1-2 hours"
            fi

            echo ""
        fi
    done
} >>"$REPORT_FILE"

#############################################
# 5. CRITICAL FILES IDENTIFICATION
#############################################

echo -e "${GREEN}[5/7] Identifying critical files...${NC}"

{
    echo "## 5. Critical Files for Migration"
    echo ""
    echo "### High Priority (Most Changes Required)"
    echo ""

    # Find files with most patterns
    find . -name "*.rs" -type f | while read -r file; do
        count=$(rg "cx:\s*Scope|<.*G:\s*Html|create_signal\(cx,|view!\s*\\{\s*cx," "$file" -c 2>/dev/null || echo "0")
        if [ "$count" -gt 20 ]; then
            echo "- \`${file}\` - **${count} patterns** - 🔴 Critical"
        fi
    done

    echo ""
    echo "### Medium Priority (Moderate Changes)"
    echo ""

    find . -name "*.rs" -type f | while read -r file; do
        count=$(rg "cx:\s*Scope|<.*G:\s*Html|create_signal\(cx,|view!\s*\\{\s*cx," "$file" -c 2>/dev/null || echo "0")
        if [ "$count" -ge 5 ] && [ "$count" -le 20 ]; then
            echo "- \`${file}\` - **${count} patterns** - 🟡 Moderate"
        fi
    done

    echo ""
} >>"$REPORT_FILE"

#############################################
# 6. RISK ASSESSMENT
#############################################

echo -e "${GREEN}[6/7] Performing risk assessment...${NC}"

{
    echo "## 6. Risk Assessment & Recommendations"
    echo ""

    # FIXED: Changed awk logic here as well
    total_patterns=$(rg "cx:\s*Scope|<.*G:\s*Html|create_signal\(cx,|view!\s*\\{\s*cx," --type rust -c 2>/dev/null | awk -F: '{s+=$2} END {print s+0}')
    total_files=$(find . -name "*.rs" -type f | wc -l)
    affected_files=$(find . -name "*.rs" -type f -exec rg -l "cx:\s*Scope" {} \; | wc -l)

    echo "### Overall Statistics"
    echo "- **Total Rust files:** ${total_files}"
    echo "- **Files requiring changes:** ${affected_files}"
    echo "- **Total patterns to migrate:** ${total_patterns}"

    if [ "$total_files" -gt 0 ]; then
        percentage=$((affected_files * 100 / total_files))
        echo "- **Codebase impact:** ${percentage}%"
    fi

    echo ""
    echo "### Risk Factors"

    # Check for macro-heavy code
    macro_files=$(rg "#\[component\]" --type rust -l | wc -l)
    echo "- Component-heavy files: **${macro_files}** (Medium risk - macro transformations)"

    # Check for complex generics
    # FIXED: Changed awk logic here as well
    complex_generics=$(rg "<.*,.*,.*>" --type rust -c 2>/dev/null | awk -F: '{s+=$2} END {print s+0}')
    echo "- Complex generic patterns: **${complex_generics}** (High risk - careful review needed)"

    # Check for lifetime annotations
    # FIXED: Changed awk logic here as well
    lifetimes=$(rg "<'[a-z]" --type rust -c 2>/dev/null | awk -F: '{s+=$2} END {print s+0}')
    echo "- Lifetime annotations: **${lifetimes}** (Medium risk - distinguish Perseus vs Sycamore)"

    echo ""
    echo "### Estimated Timeline"

    if [ "$total_patterns" -gt 500 ]; then
        echo "- **Total effort:** 10-14 days"
        echo "- **Recommended approach:** Phased migration with comprehensive testing"
    elif [ "$total_patterns" -gt 200 ]; then
        echo "- **Total effort:** 5-7 days"
        echo "- **Recommended approach:** Module-by-module migration"
    else
        echo "- **Total effort:** 2-3 days"
        echo "- **Recommended approach:** Single-pass migration possible"
    fi

    echo ""
} >>"$REPORT_FILE"

#############################################
# 7. MIGRATION PLAN
#############################################

echo -e "${GREEN}[7/7] Generating migration plan...${NC}"

{
    echo "## 7. Recommended Migration Plan"
    echo ""
    echo "### Phase 2: Dependency Updates"
    echo "- [ ] Update all \`Cargo.toml\` files to Sycamore 0.9"
    echo "- [ ] Run \`cargo update\` and resolve conflicts"
    echo "- [ ] Document initial compilation errors"
    echo ""

    echo "### Phase 3: Core Migration"
    echo "- [ ] Migrate \`perseus-core\` package"
    echo "- [ ] Migrate \`perseus-macro\` package"
    echo "- [ ] Migrate \`perseus-router\` package"
    echo "- [ ] Run tests after each package"
    echo ""

    echo "### Phase 4: Server Integrations"
    echo "- [ ] Migrate \`perseus-engine\` package"
    echo "- [ ] Migrate \`perseus-warp\` integration"
    echo "- [ ] Migrate \`perseus-axum\` integration"
    echo ""

    echo "### Phase 5: Examples & Testing"
    echo "- [ ] Update all example projects"
    echo "- [ ] Run comprehensive test suite"
    echo "- [ ] Performance benchmarking"
    echo ""

    echo "### Phase 6: Documentation"
    echo "- [ ] Update API documentation"
    echo "- [ ] Create migration guide for Perseus users"
    echo "- [ ] Update examples in docs"
    echo ""
} >>"$REPORT_FILE"

# Summary output
echo ""
echo -e "${BLUE}========================================${NC}"
echo -e "${GREEN}✅ Analysis Complete!${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""
echo -e "${BLUE}Summary:${NC}"
echo -e "- Scope parameters: ${YELLOW}${SCOPE_COUNT}${NC}"
echo -e "- Html generics: ${YELLOW}${HTML_GENERIC_COUNT}${NC}"
echo -e "- Signal calls: ${YELLOW}${SIGNAL_CX_COUNT}${NC}"
echo -e "- View macros: ${YELLOW}${VIEW_MACRO_COUNT}${NC}"
echo ""
echo -e "${BLUE}Next Steps:${NC}"
echo -e "1. Review the full report: ${YELLOW}cat ${REPORT_FILE}${NC}"
echo -e "2. Begin Phase 2: ${YELLOW}./scripts/update-dependencies.sh${NC}"
echo -e "3. Create migration branch: ${YELLOW}git checkout -b migration/sycamore-0.9${NC}"
echo ""
