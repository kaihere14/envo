# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

`envo` — zero-trust encrypted `.env` sync over Nostr. Access is enforced by keypair possession (NIP-44 / ECDH), not by trusting a relay or server. Rust 2024 edition, built on `clap` (derive), `nostr-sdk`, and `dirs`.

Early development: only `keygen` is implemented. `init` is explicitly scratch/testing code (see the comments in `src/commands/init.rs`), and `pull` / `push` / `add-user` are `println!` stubs in `src/main.rs`.

## Commands

```bash
cargo build
cargo run -- keygen              # writes/reads ~/.envo/keys.json
cargo run -- init <tag>          # reads ./.env and ./.env-share from the cwd
cargo fmt
cargo clippy
```

There is no test suite yet. When adding one, `cargo test <name>` runs a single test.

## Architecture

**Layering — keep `main.rs` thin.** Each `Commands::*` arm in `src/main.rs` should only pattern-match the error out of a helper and delegate. All filesystem access lives in `src/helper/`, and `src/commands/*` functions receive already-parsed data rather than reading files themselves (e.g. `init(tag, &env_contents, &trusted_pubkeys)`). Do not add file I/O or parsing inline in `main.rs`.

**Two file locations, two different concerns:**

- `~/.envo/keys.json` — the user's identity, JSON of `{"npub": ..., "nsec": ...}` in bech32. Managed entirely by `src/commands/key_gen.rs` + `src/helper/key_valid.rs`. `gen_key_dir()` creates the directory and an empty file on demand, so the file existing does *not* imply it holds valid keys.
- `./.env` and `./.env-share` in the current working directory — the secrets to share and the comma-separated list of trusted `npub`s. Read by `helper::env_files::load_project_files()`, which returns both in a `ProjectFiles`.

**Key loading is self-healing, but only `keygen` may create keys.** `read_existing_keys()` returns `Option<Keys>`, where `None` means "no usable identity": an empty file, unparseable JSON, keys failing `is_valid_keypair`, or a secret key that won't parse all warn and fall through instead of aborting. Only `key_gen()` acts on that by calling `create_new_keys()`. Every other command goes through `require_keys()`, which errors with `No identity found. Run \`envo keygen\` first.` — silently minting a keypair would let a command publish under a brand new identity unnoticed. `require_keys()` hands back a real `nostr_sdk::Keys` so callers can sign and do ECDH without re-parsing bech32 text.

## Error-handling conventions

- Helpers return `Result<_, Box<dyn std::error::Error>>`, building messages with `format!` so the failing path is included (`could not read {path}: {e}`).
- Callers `match` and `eprintln!("error: ...")` then return early. Avoid `unwrap()` / `expect()` in new code, including in `main.rs`.
- Reserve panics for genuinely unrecoverable state (no home directory, key generation itself failing), matching the existing usage in `key_valid.rs` and `key_gen.rs`.
