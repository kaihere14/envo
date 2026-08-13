
# envo

[![release](https://img.shields.io/github/v/release/kaihere14/climenv?label=release&sort=semver)](https://github.com/kaihere14/climenv/releases/latest)
[![build](https://img.shields.io/github/actions/workflow/status/kaihere14/climenv/release.yml?label=build)](https://github.com/kaihere14/climenv/actions/workflows/release.yml)
[![installs](https://img.shields.io/github/downloads/kaihere14/climenv/total?label=installs)](https://github.com/kaihere14/climenv/releases)

Zero-trust encrypted `.env` sync over Nostr. No relay, server, or local file is ever a trusted source of truth access is enforced entirely by Nostr keypair possession.

## Status
🚧 Early Beta — please raise any issues you find while trying.

Prebuilt binaries are published from the `release` workflow on every `v*` tag; the badges above track the newest published release, whether that build is green, and the total number of release-asset downloads.

## Install

```bash
curl -fsSL https://raw.githubusercontent.com/kaihere14/climenv/main/install.sh | sh
```

Detects your OS and CPU, downloads the matching binary from the latest release, verifies its SHA-256 and installs it to `~/.local/bin`.

- `ENVO_VERSION=v0.1.0` pins a release tag.
- `ENVO_INSTALL_DIR=/usr/local/bin` changes where the binary lands.

Prebuilt for Linux x86_64, macOS (Intel and Apple Silicon) and Windows x86_64. On Windows use Git Bash or WSL, or download the `.zip` from the [releases page](https://github.com/kaihere14/climenv/releases).

## Building from source

```bash
build/linux/build.sh                          # or build/macos/build.sh
pwsh -File build/windows/build.ps1            # on Windows
```

Each script writes `dist/envo-<target>.tar.gz` (`.zip` on Windows) plus a `.sha256`. Pushing a `v*` tag runs all four builds in CI and publishes them as release assets.

## What it does

- Generates a Nostr keypair and stores it in `~/.envo/keys.json`. Only the public key (`npub`) is ever printed.
- Encrypts your `.env` once per trusted teammate using ECDH-derived shared secrets (NIP-44) and publishes it as a signed, addressable Nostr event.
- New teammates get zero retroactive access to old secrets — cryptographically enforced, not policy.

## Commands

- `envo keygen` — generate your Nostr keypair
- `envo push <tag>` — encrypt `.env` for everyone in `.env-share`, plus yourself, and publish it
- `envo pull <tag> [--owner <npub>]` — fetch the event you are a recipient of and write `.env`

There is no separate command to add a teammate: put their `npub` in `.env-share` and run `envo push` again. Because the event is addressable, pushing the same tag always replaces the previous version.

You never need to list yourself in `.env-share` — `push` always encrypts a copy for your own key, so you can pull your own tags back on another machine. An empty `.env-share` is a valid solo push.

### Trusting a publisher

A tag is just a label on a public relay, so anyone can publish an event under it and address it to you. `pull` therefore only accepts events signed by a pubkey you have named:

```bash
envo pull my-project --owner npub1...   # first time: pin who publishes this tag
envo pull my-project                    # afterwards: pinned owner is remembered
```

The pin lives in `~/.envo/trusted_owners.json`. Passing `--owner` again re-pins the tag and warns if that changes who you were trusting.

## Stack
Rust · [nostr-sdk](https://github.com/rust-nostr/nostr) · secp256k1 · NIP-44
