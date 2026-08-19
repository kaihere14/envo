# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

`envo` — zero-trust encrypted `.env` sync over Nostr. Access is enforced by keypair possession (NIP-44 / ECDH), not by trusting a relay or server. Rust 2024 edition, built on `clap` (derive), `nostr-sdk`, and `dirs`.

**The CLI is exactly three commands: `keygen`, `push`, `pull`.** `init` and `add-user` existed once and were removed as redundant — do not reintroduce them, or any command whose job `push` already covers. `init` did identical work to `push`, and because the event is addressable (same `d` tag replaces the previous version) there is no "first publish" worth naming separately. `add-user` was only ever "add a pubkey to `.env-share`, then push", and `push` re-encrypts for the full trusted set on every run. Before adding a command, check whether it reduces to *edit a local file, then run `push`*.

## Commands

```bash
cargo build
cargo run -- keygen              # writes/reads ~/.envo/keys.json
cargo run -- push <tag>          # reads ./.env and ./.env-share from the cwd
cargo run -- pull <tag> [--owner <npub>]   # decrypts and overwrites ./.env
cargo fmt
cargo clippy
```

There is no test suite yet. When adding one, `cargo test <name>` runs a single test.

## Releases

`build/{linux,macos,windows}/` hold one packaging script per platform; each writes `dist/envo-<target>.tar.gz` (`.zip` on Windows) plus a `.sha256` in `sha256sum` format. `.github/workflows/release.yml` runs all four targets on native runners and publishes them on a `v*` tag. `install.sh` maps `uname` onto those exact asset names, and `install.ps1` hardcodes the Windows one — **the asset naming is a contract between the four: changing it in one place breaks installs.**

There are two installers because they cannot be one. `curl` in PowerShell is an alias for `Invoke-WebRequest` and takes none of curl's flags, so `curl ... | sh` cannot work there, and a Git Bash install lands on a PATH only Git Bash reads — which is why `envo` came back "not found" in the PowerShell terminal every Windows IDE opens. `install.ps1` therefore uses only built-ins (`Invoke-WebRequest`, `Expand-Archive`, `Get-FileHash`), installs to `%LOCALAPPDATA%\envo\bin`, and writes the user PATH in `HKCU\Environment` raw so a `REG_EXPAND_SZ` PATH is not flattened. Keep the two in step: both take `ENVO_VERSION` and `ENVO_INSTALL_DIR`, and both print the same `- / ✓ / ! / X` symbols as `helper::log`.

## Architecture

**Layering — keep `main.rs` thin.** Each `Commands::*` arm in `src/main.rs` should only pattern-match the error out of a helper and delegate. All filesystem access lives in `src/helper/`, and `src/commands/*` functions receive already-parsed data rather than reading files themselves (e.g. `push(tag, &env_contents, &trusted_pubkeys)`). Do not add file I/O or parsing inline in `main.rs`.

**Two file locations, two different concerns:**

- `~/.envo/` — machine-local state, owner-only (`0700`, files `0600`, enforced by `helper::secret_file`). `keys.json` is the user's identity, JSON of `{"npub": ..., "nsec": ...}` in bech32, managed entirely by `src/commands/key_gen.rs` + `src/helper/key_valid.rs`; `gen_key_dir()` creates the directory and an empty file on demand, so the file existing does *not* imply it holds valid keys. `trusted_owners.json` is the `{tag: npub}` owner pin map, managed by `helper::trusted_owners`. Anything written here goes through `write_secret()`, and `envo_dir()` re-tightens permissions on every run so identities written by older versions get repaired.
- `./.env` and `./.env-share` in the current working directory — the secrets to share and the comma-separated list of trusted `npub`s. Read by `helper::env_files::load_project_files()`, which returns both in a `ProjectFiles`.

**`push` always encrypts for the publisher too.** The recipients map is the only copy of the ciphertext, so an entry addressed to the publisher's own key is added on every push whether or not `.env-share` lists them — otherwise they could publish a tag and then be unable to `pull` it back on another machine. It is keyed by canonical bech32 `npub` because that is what `pull` looks itself up by, and a self-entry in `.env-share` (in either npub or hex form) is skipped in the loop so it is not encrypted twice. This makes an empty `.env-share` a valid solo push, so the "no valid recipients" guard counts *teammates*, not map entries.

**`pull` is pinned to one publisher per tag.** A `d` tag is public, so any author can publish a kind-30078 event under someone else's tag, list the victim in the recipients map, and — since events are scanned newest-first — have it decrypted and written over `.env`. The author is therefore never inferred: `fetch_event()` takes an owner `PublicKey` and constrains the filter with `.author()`, and `pull` resolves that owner from `--owner` (a one-time human decision, then cached in `trusted_owners.json`) or from the existing pin. With neither, it fails and asks for `--owner`. Do not add a fallback that picks an author for the user — accepting "whichever event came back" is the exact vulnerability this replaced.

**Key loading is self-healing, but only `keygen` may create keys.** `read_existing_keys()` returns `Option<Keys>`, where `None` means "no usable identity": an empty file, unparseable JSON, keys failing `is_valid_keypair`, or a secret key that won't parse all warn and fall through instead of aborting. Only `key_gen()` acts on that by calling `create_new_keys()`. Every other command goes through `require_keys()`, which errors with `No identity found. Run \`envo keygen\` first.` — silently minting a keypair would let a command publish under a brand new identity unnoticed. `require_keys()` hands back a real `nostr_sdk::Keys` so callers can sign and do ECDH without re-parsing bech32 text.

## Output conventions

All user-facing output goes through `helper::log` — `step` (`-`, work starting), `success` (`✓`), `warn` (`!`, degraded but continuing), `fail` (`X`, cannot finish). No raw `println!`/`eprintln!` outside that module; `print!` is only for interactive prompts such as `confirm()`. Successes and progress land on stdout, warnings and failures on stderr.

**The command owns the narrative.** `src/nostr/*` and `src/helper/*` stay silent about progress and return `Result` instead; only `src/commands/*` decides what the user sees. Keep it to a couple of lines per run — one `step` for what is starting, one `success` for the outcome.

## Error-handling conventions

- Helpers return `Result<_, Box<dyn std::error::Error>>`, building messages with `format!` so the failing path is included (`could not read {path}: {e}`).
- `src/commands/*` entry points return `Result<(), Box<dyn std::error::Error>>` too, propagating with `?` rather than logging and returning early. `Err` means "the thing the command exists to do did not happen"; a `log::warn` and `continue` covers anything it recovered from.
- `main` reports every failure in exactly one place: it matches the command's `Result`, calls `log::fail`, and returns `ExitCode::FAILURE`. It must not return `Err` — Rust would print its own `Error:` line next to ours, and output belongs to `helper::log` alone. Exit status is part of the interface: `envo pull && start-app` must not start the app when the pull failed.
- Avoid `unwrap()` / `expect()` in new code, including in `main.rs`.
- Reserve panics for genuinely unrecoverable state (no home directory, key generation itself failing), matching the existing usage in `key_valid.rs` and `key_gen.rs`.
