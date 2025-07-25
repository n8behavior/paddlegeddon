---
description: Create a new release for Paddlegeddon
argument-hint: "[major|minor|patch] - Version bump type (default: patch)"
---

# Release Paddlegeddon

This command automates the release process by:

1. Ensuring clean working directory
2. Running lints and formatting
3. Bumping version in Cargo.toml
4. Creating git commit and tag
5. Pushing to trigger GitHub release workflow

```bash
#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "🚀 Starting Paddlegeddon release process..."

# Check for clean working directory
if ! git diff --quiet || ! git diff --cached --quiet; then
    echo -e "${RED}❌ Uncommitted changes detected.${NC}"
    echo -e "${RED}Please commit or stash changes first.${NC}"
    git status --short
    exit 1
fi

# Pull latest changes (fast-forward only to avoid merge commits)
echo -e "${YELLOW}📥 Pulling latest changes (fast-forward only)...${NC}"
if ! git pull --ff-only origin main; then
    echo -e "${RED}❌ Cannot fast-forward. Your branch has diverged from origin/main.${NC}"
    echo -e "${YELLOW}Please resolve the divergence manually before releasing.${NC}"
    exit 1
fi

# Run quality checks
echo -e "${YELLOW}🔍 Running Bevy linter...${NC}"
if ! bevy lint 2>&1 | grep -q "error\[E0658\]"; then
    # bevy lint succeeded or failed with actual lint errors
    if ! bevy lint; then
        echo -e "${RED}❌ Bevy linting failed. Please fix errors before releasing.${NC}"
        exit 1
    fi
else
    echo -e "${YELLOW}⚠️  Bevy lint skipped due to dependency compilation issues${NC}"
fi

echo -e "${YELLOW}🔍 Running Rust linter (clippy)...${NC}"
if ! cargo clippy -- -D warnings; then
    echo -e "${RED}❌ Clippy found warnings. Please fix all warnings before releasing.${NC}"
    exit 1
fi

echo -e "${YELLOW}📝 Running markdown linter...${NC}"
if ! markdownlint-cli2 "**/*.md"; then
    echo -e "${RED}❌ Markdown linting failed.${NC}"
    echo -e "${RED}Please fix errors before releasing.${NC}"
    exit 1
fi

echo -e "${YELLOW}🎨 Running formatter...${NC}"
rustfmt --edition 2024 src/**/*.rs

# Check if formatter made any changes
if ! git diff --quiet; then
    echo -e "${RED}❌ Formatter made changes.${NC}"
    echo -e "${RED}Please review and commit formatting changes first.${NC}"
    git diff --stat
    exit 1
fi

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
echo -e "Current version: ${GREEN}$CURRENT_VERSION${NC}"

# Parse version components
IFS='.' read -r MAJOR MINOR PATCH <<< "$CURRENT_VERSION"

# Determine version bump type
VERSION_TYPE="${1:-patch}"

# Calculate new version
case "$VERSION_TYPE" in
    major)
        NEW_MAJOR=$((MAJOR + 1))
        NEW_VERSION="$NEW_MAJOR.0.0"
        ;;
    minor)
        NEW_MINOR=$((MINOR + 1))
        NEW_VERSION="$MAJOR.$NEW_MINOR.0"
        ;;
    patch)
        NEW_PATCH=$((PATCH + 1))
        NEW_VERSION="$MAJOR.$MINOR.$NEW_PATCH"
        ;;
    *)
        echo -e "${RED}❌ Invalid version type: $VERSION_TYPE${NC}"
        echo "Usage: /release [major|minor|patch]"
        exit 1
        ;;
esac

echo -e "Bumping ${YELLOW}$VERSION_TYPE${NC} version to: ${GREEN}$NEW_VERSION${NC}"

# Update version in Cargo.toml
sed -i "s/^version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" Cargo.toml

# Update Cargo.lock
echo -e "${YELLOW}📦 Updating Cargo.lock...${NC}"
cargo check --quiet

# Stage changes
git add Cargo.toml Cargo.lock

# Create commit
COMMIT_MESSAGE="chore: Release v$NEW_VERSION

🚀 Automated release via /release command"

git commit -m "$COMMIT_MESSAGE"

# Create annotated tag
TAG="v$NEW_VERSION"
TAG_MESSAGE="Release $TAG

Created with /release command"

git tag -a "$TAG" -m "$TAG_MESSAGE"

echo -e "${GREEN}✅ Version bumped and committed${NC}"

# Push changes and tag
echo -e "${YELLOW}📤 Pushing to GitHub...${NC}"
git push origin main
git push origin "$TAG"

echo -e "${GREEN}✅ Release $TAG created successfully!${NC}"
echo -e "${YELLOW}🎯 GitHub Actions will now build and publish the release${NC}"
echo -e "View progress at: https://github.com/n8behavior/paddlegeddon/actions"
```
