#!/usr/bin/env sh
#
# envo installer.
#
#   curl -fsSL https://raw.githubusercontent.com/kaihere14/climenv/main/install.sh | sh
#
# Detects the OS and CPU, downloads the matching binary from the latest
# GitHub release, verifies its checksum and drops it on your PATH.
#
# Environment:
#   ENVO_VERSION      release tag to install (default: latest)
#   ENVO_INSTALL_DIR  where to put the binary (default: $HOME/.local/bin)

set -eu

REPO="kaihere14/climenv"
BIN="envo"
VERSION="${ENVO_VERSION:-latest}"
INSTALL_DIR="${ENVO_INSTALL_DIR:-$HOME/.local/bin}"

# Same symbols the CLI itself uses, so the install reads like an envo run.
step() { printf -- '- %s\n' "$1"; }
success() { printf '%s %s\n' '✓' "$1"; }
warn() { printf '%s %s\n' '!' "$1" >&2; }
fail() {
    printf '%s %s\n' 'X' "$1" >&2
    exit 1
}

# Maps `uname` output onto the release asset names the build scripts produce.
detect_target() {
    os="$(uname -s)"
    arch="$(uname -m)"

    case "$os" in
    Linux)
        case "$arch" in
        x86_64 | amd64) target="x86_64-unknown-linux-gnu" ;;
        *) fail "no prebuilt binary for Linux $arch yet — build from source with \`cargo install --git https://github.com/$REPO\`" ;;
        esac
        ext="tar.gz"
        bin_file="$BIN"
        ;;
    Darwin)
        case "$arch" in
        x86_64) target="x86_64-apple-darwin" ;;
        arm64 | aarch64) target="aarch64-apple-darwin" ;;
        *) fail "no prebuilt binary for macOS $arch yet" ;;
        esac
        ext="tar.gz"
        bin_file="$BIN"
        ;;
    MINGW* | MSYS* | CYGWIN* | Windows_NT)
        # $HOME/.local/bin is on the PATH Git Bash builds, not the Windows
        # one, so a binary installed here is missing from PowerShell, cmd and
        # most IDE terminals. install.ps1 puts it somewhere all of them look.
        warn "on Windows, prefer the PowerShell installer so envo lands on the Windows PATH:"
        warn "  irm https://raw.githubusercontent.com/$REPO/main/install.ps1 | iex"
        target="x86_64-pc-windows-msvc"
        ext="zip"
        bin_file="$BIN.exe"
        ;;
    *)
        fail "unsupported operating system: $os"
        ;;
    esac
}

# curl and wget are both common enough that requiring one specifically would
# fail installs for no reason.
download() {
    url="$1"
    dest="$2"

    if command -v curl >/dev/null 2>&1; then
        curl -fsSL "$url" -o "$dest"
    elif command -v wget >/dev/null 2>&1; then
        wget -qO "$dest" "$url"
    else
        fail "need curl or wget to download $url"
    fi
}

# A truncated or tampered download should never end up on the PATH, but a
# machine without a sha256 tool can still install — it just says so.
verify_checksum() {
    archive="$1"
    checksum_file="$2"

    expected="$(awk '{print $1}' "$checksum_file")"

    if command -v sha256sum >/dev/null 2>&1; then
        actual="$(sha256sum "$archive" | awk '{print $1}')"
    elif command -v shasum >/dev/null 2>&1; then
        actual="$(shasum -a 256 "$archive" | awk '{print $1}')"
    else
        warn "no sha256 tool found, skipping checksum verification"
        return 0
    fi

    if [ "$expected" != "$actual" ]; then
        fail "checksum mismatch: expected $expected, got $actual"
    fi
}

unpack() {
    archive="$1"
    dir="$2"

    case "$ext" in
    tar.gz)
        tar -xzf "$archive" -C "$dir"
        ;;
    zip)
        if command -v unzip >/dev/null 2>&1; then
            unzip -q "$archive" -d "$dir"
        else
            fail "need unzip to unpack $archive"
        fi
        ;;
    esac
}

main() {
    detect_target

    asset="$BIN-$target.$ext"

    if [ "$VERSION" = "latest" ]; then
        # This path redirects to the newest release, so no API call and no
        # rate limit for unauthenticated installs.
        base_url="https://github.com/$REPO/releases/latest/download"
    else
        base_url="https://github.com/$REPO/releases/download/$VERSION"
    fi

    tmp="$(mktemp -d)"
    trap 'rm -rf "$tmp"' EXIT INT TERM

    step "Downloading $BIN $VERSION for $target"
    download "$base_url/$asset" "$tmp/$asset" ||
        fail "could not download $base_url/$asset — check that a release exists for $VERSION"
    download "$base_url/$asset.sha256" "$tmp/$asset.sha256"

    verify_checksum "$tmp/$asset" "$tmp/$asset.sha256"

    unpack "$tmp/$asset" "$tmp"

    mkdir -p "$INSTALL_DIR" || fail "could not create $INSTALL_DIR"
    mv "$tmp/$bin_file" "$INSTALL_DIR/$bin_file" || fail "could not write to $INSTALL_DIR"
    chmod +x "$INSTALL_DIR/$bin_file"

    success "Installed $BIN to $INSTALL_DIR/$bin_file"

    case ":$PATH:" in
    *":$INSTALL_DIR:"*)
        step "Run \`$BIN keygen\` to create your identity"
        ;;
    *)
        warn "$INSTALL_DIR is not on your PATH — add this to your shell profile:"
        warn "  export PATH=\"$INSTALL_DIR:\$PATH\""
        ;;
    esac
}

main "$@"
