#!/bin/bash
# Perseus Migration: Batch Pattern Replacement
# Automated replacements for common Sycamore 0.8 → 0.9 patterns

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Configuration
DRY_RUN=false
TARGET_DIR="."
BACKUP_SUFFIX=".pre-migration"

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --dir)
            TARGET_DIR="$2"
            shift 2
            ;;
        --help)
            echo "Usage: $0 [--dry-run] [--dir DIR]"
            echo ""
            echo "Options:"
            echo "  --dry-run    Show changes without applying them"
            echo "  --dir DIR    Target directory (default: current directory)"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}Phase 3: Batch Pattern Replacement${NC}"
echo -e "${BLUE}========================================${NC}\n"

if [ "$DRY_RUN" = true ]; then
    echo -e "${YELLOW}🔍 DRY RUN MODE - No changes will be made${NC}\n"
fi

# Function to apply a replacement pattern
apply_pattern() {
    local description="$1"
    local search="$2"
    local replace="$3"
    
    echo -e "${GREEN}[PATTERN] ${description}${NC}"
    
    if [ "$DRY_RUN" = true ]; then
        # Show what would be changed
        local count=$(rg --type rust -l "$search" "$TARGET_DIR" 2>/dev/null | wc -l)
        echo -e "  ${YELLOW}Would affect ${count} files${NC}"
        
        # Show some examples
        rg --type rust -n --heading "$search" "$TARGET_DIR" 2>/dev/null | head -3 | while read -r line; do
            echo -e "  ${BLUE}Example:${NC} ${line}"
        done
    else
        # Apply the replacement
        local count=0
        rg --type rust -l "$search" "$TARGET_DIR" 2>/dev/null | while read -r file; do
            # Create backup if it doesn't exist
            if [ ! -f "${file}${BACKUP_SUFFIX}" ]; then
                cp "$file" "${file}${BACKUP_SUFFIX}"
            fi
            
            # Apply replacement
            perl -i -pe "s/${search}/${replace}/g" "$file"
            ((count++)) || true
        done
        
        echo -e "  ${GREEN}✓ Applied to ${count} files${NC}"
    fi
    echo ""
}

#############################################
# PATTERN 1: Remove Scope Parameter
#############################################

echo -e "${BLUE}[1/10] Removing Scope Parameters${NC}\n"

# Pattern 1a: ", cx: Scope<'a>" in function parameters
apply_pattern \
    "Remove ', cx: Scope<'a>' from parameters" \
    ",\s*cx:\s*Scope<'[a-z]>" \
    ""

# Pattern 1b: ", cx: Scope" without lifetime
apply_pattern \
    "Remove ', cx: Scope' from parameters" \
    ",\s*cx:\s*Scope" \
    ""

# Pattern 1c: "cx: Scope<'a>," at start of parameters
apply_pattern \
    "Remove 'cx: Scope<'a>,' from start of parameters" \
    "cx:\s*Scope<'[a-z]>,\s*" \
    ""

# Pattern 1d: "cx: Scope," at start
apply_pattern \
    "Remove 'cx: Scope,' from start of parameters" \
    "cx:\s*Scope,\s*" \
    ""

#############################################
# PATTERN 2: Remove Generic Html Constraints
#############################################

echo -e "${BLUE}[2/10] Removing Generic Html Constraints${NC}\n"

# Pattern 2a: "<'a, G: Html>" combined lifetime and generic
apply_pattern \
    "Remove '<'a, G: Html>' from function signatures" \
    "<'[a-z],\s*G:\s*Html>" \
    ""

# Pattern 2b: "<G: Html>" just generic
apply_pattern \
    "Remove '<G: Html>' from function signatures" \
    "<G:\s*Html>" \
    ""

# Pattern 2c: "View<G>" return types
apply_pattern \
    "Change 'View<G>' to 'View'" \
    "View<G>" \
    "View"

#############################################
# PATTERN 3: Remove Lifetimes (Careful!)
#############################################

echo -e "${BLUE}[3/10] Removing Component Lifetimes${NC}\n"

# Pattern 3a: Remove "'a" from component annotations (careful not to affect data lifetimes)
echo -e "${YELLOW}⚠️  Lifetime removal requires careful manual review${NC}"
echo -e "${YELLOW}   Skipping automatic lifetime removal to prevent data structure corruption${NC}\n"

# Instead, just report where lifetimes exist
if [ "$DRY_RUN" = false ]; then
    echo -e "${BLUE}Lifetimes found in these files (manual review needed):${NC}"
    rg "<'[a-z]" --type rust -l "$TARGET_DIR" | head -10 || echo "No lifetimes found"
fi
echo ""

#############################################
# PATTERN 4: Update Signal API
#############################################

echo -e "${BLUE}[4/10] Updating Signal API${NC}\n"

# Pattern 4a: create_signal(cx, value)
apply_pattern \
    "Remove 'cx,' from create_signal calls" \
    "create_signal\\(cx,\s*" \
    "create_signal("

# Pattern 4b: create_rc_signal → create_signal
apply_pattern \
    "Replace 'create_rc_signal' with 'create_signal'" \
    "create_rc_signal" \
    "create_signal"

# Pattern 4c: RcSignal type annotations
apply_pattern \
    "Replace 'RcSignal' type with 'Signal'" \
    "RcSignal<" \
    "Signal<"

# Pattern 4d: .get().clone() → .get_clone()
apply_pattern \
    "Replace '.get().clone()' with '.get_clone()'" \
    "\\.get\\(\\)\\.clone\\(\\)" \
    ".get_clone()"

#############################################
# PATTERN 5: Update Reactive Primitives
#############################################

echo -e "${BLUE}[5/10] Updating Reactive Primitives${NC}\n"

# Pattern 5a: create_effect(cx, ||
apply_pattern \
    "Remove 'cx,' from create_effect calls" \
    "create_effect\\(cx,\s*" \
    "create_effect("

# Pattern 5b: create_memo(cx, ||
apply_pattern \
    "Remove 'cx,' from create_memo calls" \
    "create_memo\\(cx,\s*" \
    "create_memo("

# Pattern 5c: create_selector(cx, ||
apply_pattern \
    "Remove 'cx,' from create_selector calls" \
    "create_selector\\(cx,\s*" \
    "create_selector("

#############################################
# PATTERN 6: Update View Macro
#############################################

echo -e "${BLUE}[6/10] Updating View Macro Syntax${NC}\n"

# Pattern 6a: view! { cx,
apply_pattern \
    "Remove 'cx,' from view! macros" \
    "view!\s*\\{\s*cx," \
    "view! {"

#############################################
# PATTERN 7: Update Indexed/Keyed Lists
#############################################

echo -e "${BLUE}[7/10] Updating Indexed/Keyed Lists${NC}\n"

# Pattern 7a: iterable= → list=
apply_pattern \
    "Replace 'iterable=' with 'list=' in Indexed/Keyed" \
    "iterable\s*=" \
    "list="

#############################################
# PATTERN 8: Update Rust Keywords
#############################################

echo -e "${BLUE}[8/10] Updating Rust Keywords in View Macros${NC}\n"

# Pattern 8a: ref= → r#ref=
echo -e "${YELLOW}⚠️  Keyword replacements require context-aware processing${NC}"
echo -e "${YELLOW}   Skipping automatic keyword updates (manual review recommended)${NC}\n"

#############################################
# PATTERN 9: Remove RcSignal Imports
#############################################

echo -e "${BLUE}[9/10] Cleaning Up Imports${NC}\n"

# Pattern 9a: Remove RcSignal imports
apply_pattern \
    "Remove 'use sycamore::reactive::RcSignal' imports" \
    "use\s+sycamore::reactive::RcSignal;\n" \
    ""

# Pattern 9b: Remove RcSignal from multi-imports
apply_pattern \
    "Remove 'RcSignal' from use statements" \
    ",\s*RcSignal" \
    ""

#############################################
# PATTERN 10: Update Builder API
#############################################

echo -e "${BLUE}[10/10] Builder API Updates${NC}\n"

echo -e "${YELLOW}⚠️  Builder API changes are complex and require manual review${NC}"
echo -e "${YELLOW}   Common patterns:${NC}"
echo -e "${YELLOW}     .c()    → .children()${NC}"
echo -e "${YELLOW}     .t()    → .children() for text${NC}"
echo -e "${YELLOW}     .view() → .into()${NC}\n"

#############################################
# Summary and Validation
#############################################

echo -e "${BLUE}========================================${NC}"

if [ "$DRY_RUN" = true ]; then
    echo -e "${YELLOW}✓ Dry run complete - no changes made${NC}"
    echo -e "${BLUE}========================================${NC}"
    echo ""
    echo -e "${BLUE}To apply changes:${NC}"
    echo -e "  ${YELLOW}./scripts/batch-replace.sh${NC}"
    echo ""
else
    echo -e "${GREEN}✅ Batch replacements applied!${NC}"
    echo -e "${BLUE}========================================${NC}"
    echo ""
    
    # Count modified files
    MODIFIED_COUNT=$(find "$TARGET_DIR" -name "*${BACKUP_SUFFIX}" -type f | wc -l)
    echo -e "${BLUE}Modified files: ${YELLOW}${MODIFIED_COUNT}${NC}"
    echo ""
    
    # Test compilation
    echo -e "${GREEN}Testing compilation...${NC}"
    if cargo check --workspace 2>&1 | head -20; then
        echo -e "\n${GREEN}🎉 Compilation successful!${NC}"
    else
        echo -e "\n${YELLOW}⚠️  Compilation errors remain (expected)${NC}"
        echo -e "${BLUE}Run for full error list:${NC} ${YELLOW}cargo check --workspace 2>&1 | tee migration-reports/errors/phase3_errors.log${NC}"
    fi
    
    echo ""
    echo -e "${BLUE}Backups created with suffix: ${YELLOW}${BACKUP_SUFFIX}${NC}"
    echo ""
    echo -e "${BLUE}Next Steps:${NC}"
    echo -e "1. Review changes: ${YELLOW}git diff${NC}"
    echo -e "2. Manual fixes needed for:"
    echo -e "   - Lifetime parameters (check each carefully)"
    echo -e "   - Rust keyword attributes (ref → r#ref)"
    echo -e "   - Builder API patterns"
    echo -e "3. Run tests: ${YELLOW}./scripts/run-tests.sh${NC}"
    echo ""
    echo -e "${YELLOW}To rollback all changes:${NC}"
    echo -e "  find . -name '*${BACKUP_SUFFIX}' -type f -exec sh -c 'mv \"\$1\" \"\${1%$BACKUP_SUFFIX}\"' _ {} \\;"
    echo ""
fi
