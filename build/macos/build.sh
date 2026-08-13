#!/usr/bin/env bash
# Builds the macOS release binary and packages it the way install.sh expects:
# dist/envo-<target>.tar.gz plus a matching .sha256.
#
#   build/macos/build.sh [target-triple]
#
# Defaults to the host architecture, so Apple Silicon produces the aarch64
# build and an Intel machine produces the x86_64 one.
set -euo pipefail

if [ "$(uname -m)" = "x86_64" ]; then
  DEFAULT_TARGET="x86_64-apple-darwin"
else
  DEFAULT_TARGET="aarch64-apple-darwin"
fi

TARGET="${1:-$DEFAULT_TARGET}"
BIN="envo"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DIST="$ROOT/dist"

cd "$ROOT"

if command -v rustup >/dev/null 2>&1; then
  rustup target add "$TARGET" >/dev/null
fi

echo "- Building $BIN for $TARGET"
cargo build --release --locked --target "$TARGET"

mkdir -p "$DIST"
ARCHIVE="$BIN-$TARGET.tar.gz"

tar -czf "$DIST/$ARCHIVE" -C "target/$TARGET/release" "$BIN"

# macOS ships `shasum` rather than GNU coreutils' `sha256sum`; both write the
# same "<hash>  <file>" line that install.sh reads back.
(cd "$DIST" && shasum -a 256 "$ARCHIVE" >"$ARCHIVE.sha256")

echo "✓ Packaged dist/$ARCHIVE"
