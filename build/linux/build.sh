#!/usr/bin/env bash
# Builds the Linux release binary and packages it the way install.sh expects:
# dist/envo-<target>.tar.gz plus a matching .sha256.
#
#   build/linux/build.sh [target-triple]
set -euo pipefail

TARGET="${1:-x86_64-unknown-linux-gnu}"
BIN="envo"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DIST="$ROOT/dist"

cd "$ROOT"

# Only needed when cross-compiling; harmless when the target is the host.
if command -v rustup >/dev/null 2>&1; then
  rustup target add "$TARGET" >/dev/null
fi

echo "- Building $BIN for $TARGET"
cargo build --release --locked --target "$TARGET"

mkdir -p "$DIST"
ARCHIVE="$BIN-$TARGET.tar.gz"

# -C so the archive holds a bare `envo`, not the target/ path it was built in.
tar -czf "$DIST/$ARCHIVE" -C "target/$TARGET/release" "$BIN"
(cd "$DIST" && sha256sum "$ARCHIVE" >"$ARCHIVE.sha256")

echo "✓ Packaged dist/$ARCHIVE"
