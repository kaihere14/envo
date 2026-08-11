
# envo

Zero-trust encrypted `.env` sync over Nostr. No relay, server, or local file is ever a trusted source of truth access is enforced entirely by Nostr keypair possession.

## Status
🚧 Early development — not ready for use yet.

## What it does

- Currently generates a Nostr keypair and prints the public (`npub`) and private (`nsec`) keys in bech32 format.
- Future functionality will encrypt `.env` secrets individually per trusted teammate using ECDH‑derived shared secrets (NIP‑44) and publish them as signed, addressable Nostr events.
- New teammates will get zero retroactive access to old secrets — cryptographically enforced, not policy.

## Commands (planned)

- `envo keygen` — generate a Nostr keypair (implemented)
- `envo init <tag>` — first publish for a project (stub)
- `envo pull <tag>` — fetch and decrypt your `.env` (stub)
- `envo push <tag>` — republish after edits (owner only) (stub)
- `envo add-user <tag> --pubkey <npub>` — grant a teammate access (owner only) (stub)

## Stack
Rust · [nostr-sdk](https://github.com/rust-nostr/nostr) · secp256k1 · NIP-44
