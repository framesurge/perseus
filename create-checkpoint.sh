#!/bin/bash
# Perseus Migration: Git Checkpoint Creation
# Create named checkpoints during migration for easy rollback

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Get checkpoint name from argument
CHECKPOINT_NAME="${1:-checkpoint-$(date +%Y%m%d_%H%M%S)}"
CHECKPOINT_TAG="migration/${CHECKPOINT_NAME}"

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}Creating Git Checkpoint${NC}"
echo -e "${BLUE}========================================${NC}\n"

echo -e "${YELLOW}Checkpoint name: ${CHECKPOINT_TAG}${NC}\n"

# Check if we are in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo -e "${RED}Not a git repository${NC}"
    exit 1
fi

# Check for uncommitted changes
if ! git diff-index --quiet HEAD --; then
    echo -e "${YELLOW}Uncommitted changes detected${NC}"
    echo -e "${BLUE}Would you like to commit them first? (y/n)${NC}"
    read -r response

    if [[ "$response" =~ ^[Yy]$ ]]; then
        echo -e "\n${GREEN}Creating commit...${NC}"

        # Show changes
        echo -e "${BLUE}Changes to commit:${NC}"
        git status --short
        echo ""

        # Get commit message
        echo -e "${BLUE}Enter commit message:${NC}"
        read -r commit_message

        # Commit
        git add -A
        git commit -m "${commit_message}"

        echo -e "${GREEN}Changes committed${NC}\n"
    else
        echo -e "${YELLOW}Proceeding without commit${NC}\n"
    fi
fi

# Get current commit info
CURRENT_COMMIT=$(git rev-parse HEAD)
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
COMMIT_MSG=$(git log -1 --pretty=%B)

echo -e "${BLUE}Current state:${NC}"
echo -e "Branch:  ${YELLOW}${CURRENT_BRANCH}${NC}"
echo -e "Commit:  ${YELLOW}${CURRENT_COMMIT:0:8}${NC}"
echo -e "Message: ${YELLOW}${COMMIT_MSG}${NC}"
echo ""

# Check if tag already exists
if git rev-parse "$CHECKPOINT_TAG" >/dev/null 2>&1; then
    echo -e "${YELLOW}Checkpoint '${CHECKPOINT_TAG}' already exists${NC}"
    echo -e "${BLUE}Overwrite? (y/n)${NC}"
    read -r response

    if [[ "$response" =~ ^[Yy]$ ]]; then
        git tag -d "$CHECKPOINT_TAG"
        echo -e "${GREEN}Old checkpoint deleted${NC}\n"
    else
        echo -e "${RED}Aborted${NC}"
        exit 1
    fi
fi

# Create annotated tag
echo -e "${GREEN}Creating checkpoint tag...${NC}"

# Capture compilation status safely
if cargo check --workspace > /dev/null 2>&1; then
    COMPILATION_STATUS="passing"
else
    COMPILATION_STATUS="failing"
fi

git tag -a "$CHECKPOINT_TAG" -m "Migration checkpoint: ${CHECKPOINT_NAME}

Created: $(date)
Branch: ${CURRENT_BRANCH}
Commit: ${CURRENT_COMMIT}

Status at this checkpoint:
- Migration phase: ${CHECKPOINT_NAME}
- Compilation: ${COMPILATION_STATUS}
"

echo -e "${GREEN}Checkpoint created: ${CHECKPOINT_TAG}${NC}\n"

# Show all migration checkpoints
echo -e "${BLUE}All migration checkpoints:${NC}"
git tag -l "migration/*" | while IFS= read -r tag; do
    # Skip empty lines
    [[ -z "$tag" ]] && continue

    TAG_DATE=$(git log -1 --format=%ai "$tag")
    # `-n1` prints tag name + first line of message; strip name
    TAG_MSG=$(git tag -l --format='%(contents:subject)' "$tag")

    echo -e "  ${YELLOW}${tag}${NC}"
    echo -e "    ${BLUE}${TAG_DATE}${NC}"
    echo -e "    ${TAG_MSG}"
    echo ""
done

echo -e "${BLUE}========================================${NC}"
echo -e "${GREEN}Checkpoint Created Successfully${NC}"
echo -e "${BLUE}========================================${NC}\n"

echo -e "${BLUE}Checkpoint Info:${NC}"
echo -e "Tag:    ${YELLOW}${CHECKPOINT_TAG}${NC}"
echo -e "Commit: ${YELLOW}${CURRENT_COMMIT:0:8}${NC}"
echo ""

echo -e "${BLUE}To restore this checkpoint:${NC}"
echo -e "  ${YELLOW}git reset --hard ${CHECKPOINT_TAG}${NC}"
echo ""

echo -e "${BLUE}To compare with this checkpoint:${NC}"
echo -e "  ${YELLOW}git diff ${CHECKPOINT_TAG}${NC}"
echo ""

echo -e "${BLUE}To list all checkpoints:${NC}"
echo -e "  ${YELLOW}git tag -l 'migration/*'${NC}"
echo ""
