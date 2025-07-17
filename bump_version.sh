#!/bin/bash

# Simple version bumping script for alchemy
# Usage: ./bump_version.sh [major|minor|patch]

set -e

BUMP_TYPE=${1:-patch}

if [ "$BUMP_TYPE" != "major" ] && [ "$BUMP_TYPE" != "minor" ] && [ "$BUMP_TYPE" != "patch" ]; then
    echo "Usage: $0 [major|minor|patch]"
    echo "  major: 1.0.0 -> 2.0.0"
    echo "  minor: 1.0.0 -> 1.1.0" 
    echo "  patch: 1.0.0 -> 1.0.1"
    exit 1
fi

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
echo "Current version: $CURRENT_VERSION"

# Parse version components
IFS='.' read -ra VERSION_PARTS <<< "$CURRENT_VERSION"
MAJOR=${VERSION_PARTS[0]}
MINOR=${VERSION_PARTS[1]}
PATCH=${VERSION_PARTS[2]}

# Bump version based on type
case $BUMP_TYPE in
    major)
        MAJOR=$((MAJOR + 1))
        MINOR=0
        PATCH=0
        ;;
    minor)
        MINOR=$((MINOR + 1))
        PATCH=0
        ;;
    patch)
        PATCH=$((PATCH + 1))
        ;;
esac

NEW_VERSION="$MAJOR.$MINOR.$PATCH"
echo "New version: $NEW_VERSION"

# Update Cargo.toml
sed -i.bak "s/version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" Cargo.toml
rm Cargo.toml.bak

# Update Cargo.lock
cargo check --quiet

echo "✅ Version bumped to $NEW_VERSION"
echo "📝 Now commit the changes and push to main:"
echo "   git add Cargo.toml Cargo.lock"
echo "   git commit -m \"bump: version $NEW_VERSION\""
echo "   git push origin main"
echo ""
echo "🚀 The release will be automatically created when you push to main."