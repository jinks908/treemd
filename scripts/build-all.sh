#!/bin/bash
# Build script for cross-platform binaries
# Usage: ./scripts/build-all.sh

set -e

TARGETS=(
  "x86_64-unknown-linux-gnu"
  "aarch64-unknown-linux-gnu"
  "x86_64-apple-darwin"
  "aarch64-apple-darwin"
  "x86_64-pc-windows-msvc"
)

LINUX_ARM_TARGETS=(
  "aarch64-unknown-linux-gnu"
)

OUTPUT_DIR="target/release-artifacts"
mkdir -p "$OUTPUT_DIR"

echo "Building treemd for multiple platforms..."
echo ""

for target in "${TARGETS[@]}"; do
  echo "Building for $target..."

  # Check if this is a Linux ARM target (requires cross)
  if [[ " ${LINUX_ARM_TARGETS[@]} " =~ " ${target} " ]]; then
    if ! command -v cross &> /dev/null; then
      echo "Error: cross is required for $target. Install with: cargo install cross"
      exit 1
    fi
    BUILDER="cross"
  else
    BUILDER="cargo"
  fi

  $BUILDER build --release --target "$target" 2>&1 | head -20

  # Copy the binary to the release artifacts directory
  if [[ "$target" == *"windows"* ]]; then
    cp "target/$target/release/treemd.exe" "$OUTPUT_DIR/treemd-$target.exe"
  else
    cp "target/$target/release/treemd" "$OUTPUT_DIR/treemd-$target"
    chmod +x "$OUTPUT_DIR/treemd-$target"
  fi

  echo "✓ Built: treemd-$target"
  echo ""
done

echo "All builds completed!"
echo "Artifacts in: $OUTPUT_DIR"
ls -lh "$OUTPUT_DIR"
