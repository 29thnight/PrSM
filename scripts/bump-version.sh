#!/usr/bin/env bash
# bump-version.sh — Update version across all PrSM components
#
# Usage:
#   ./scripts/bump-version.sh 0.2.0
#   ./scripts/bump-version.sh 1.0.0 --tag   # also create git tag
#
# Updates:
#   1. crates/refraction/Cargo.toml
#   2. vscode-prsm/package.json
#   3. unity-package/package.json
#   4. crates/refraction/wix/main.wxs
#
# Does NOT update language version (1.0/2.0) — that is managed separately
# in .prsmproject files.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

NEW_VERSION="${1:-}"
CREATE_TAG="${2:-}"

if [[ -z "$NEW_VERSION" ]]; then
    echo "Usage: $0 <version> [--tag]"
    echo "Example: $0 0.2.0 --tag"
    exit 1
fi

# Validate SemVer format
if ! [[ "$NEW_VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "Error: Version must be in SemVer format (X.Y.Z), got: $NEW_VERSION"
    exit 1
fi

echo "Bumping all components to v${NEW_VERSION}..."

# 1. Cargo.toml
CARGO_FILE="$ROOT_DIR/crates/refraction/Cargo.toml"
sed -i "s/^version = \"[0-9]*\.[0-9]*\.[0-9]*\"/version = \"${NEW_VERSION}\"/" "$CARGO_FILE"
echo "  ✓ $CARGO_FILE"

# 2. VS Code extension package.json
VSCODE_FILE="$ROOT_DIR/vscode-prsm/package.json"
sed -i "s/\"version\": \"[0-9]*\.[0-9]*\.[0-9]*\"/\"version\": \"${NEW_VERSION}\"/" "$VSCODE_FILE"
echo "  ✓ $VSCODE_FILE"

# 3. Unity package.json
UNITY_FILE="$ROOT_DIR/unity-package/package.json"
sed -i "s/\"version\": \"[0-9]*\.[0-9]*\.[0-9]*\"/\"version\": \"${NEW_VERSION}\"/" "$UNITY_FILE"
echo "  ✓ $UNITY_FILE"

# 4. WiX installer
WIX_FILE="$ROOT_DIR/crates/refraction/wix/main.wxs"
sed -i "s/Version='[0-9]*\.[0-9]*\.[0-9]*'/Version='${NEW_VERSION}'/" "$WIX_FILE"
echo "  ✓ $WIX_FILE"

echo ""
echo "All components updated to v${NEW_VERSION}"

# Verify
echo ""
echo "Verification:"
grep "^version" "$CARGO_FILE" | head -1
grep '"version"' "$VSCODE_FILE" | head -1
grep '"version"' "$UNITY_FILE" | head -1
grep "Version=" "$WIX_FILE" | head -1

if [[ "$CREATE_TAG" == "--tag" ]]; then
    echo ""
    echo "Creating git tag v${NEW_VERSION}..."
    git add "$CARGO_FILE" "$VSCODE_FILE" "$UNITY_FILE" "$WIX_FILE"
    git commit -m "chore: bump version to ${NEW_VERSION}"
    git tag "v${NEW_VERSION}"
    echo "  ✓ Tag v${NEW_VERSION} created"
    echo ""
    echo "To release: git push origin main && git push origin v${NEW_VERSION}"
fi
