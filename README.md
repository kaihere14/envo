# climenv

Zero-trust encrypted `.env` sync over Nostr. No relay, server, or local file is ever a trusted source of truth access is enforced entirely by Nostr keypair possession.

## Status
🚧 Early development — not ready for use yet.

## What it does
- Encrypts `.env` secrets individually per trusted teammate using ECDH-derived shared secrets (NIP-44)
- Publishes as a signed, addressable Nostr event — relays only ever see ciphertext
- New teammates get zero retroactive access to old secrets — cryptographically enforced, not policy

## Commands (planned)
- `climenv keygen` — generate a Nostr keypair
- `climenv init <tag>` — first publish for a project
- `climenv pull <tag>` — fetch and decrypt your `.env`
- `climenv push <tag>` — republish after edits (owner only)
- `climenv add-user <tag> --pubkey <npub>` — grant a teammate access (owner only)

## Stack
Rust · [nostr-sdk](https://github.com/rust-nostr/nostr) · secp256k1 · NIP-44
