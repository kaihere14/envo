use crate::helper::event_content::EventContent;
use crate::helper::log;
use crate::helper::trusted_owners::{get_trusted_owner, save_trusted_owner};
use crate::nostr::fetch_event::fetch_event;
use nostr_sdk::prelude::*;

/// Fetches the secrets published under `tag` by its trusted owner and writes
/// them to `.env`.
///
/// Returns `Err` unless `.env` was actually written, so `envo pull && start`
/// cannot run against secrets that never arrived.
pub async fn pull(tag: String, owner: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let keys = crate::commands::key_gen::require_keys()?;

    let owner_pubkey = resolve_owner(&tag, owner.as_deref())?;

    let my_pubkey = keys
        .public_key()
        .to_bech32()
        .map_err(|e| format!("Could not encode your public key: {}", e))?;

    log::step(&format!("Fetching secrets for tag \"{}\"", tag));

    let events = fetch_event(&tag, &keys, owner_pubkey)
        .await
        .map_err(|e| format!("Could not reach the relays: {}", e))?;

    if events.is_empty() {
        return Err(format!(
            "No secrets published under tag \"{}\" by its trusted owner",
            tag
        )
        .into());
    }

    for event in events {
        let content: EventContent = match serde_json::from_str(&event.content) {
            Ok(content) => content,
            Err(_) => {
                log::warn("Skipped an event with malformed content");
                continue;
            }
        };

        let Some(ciphertext) = content.recipients.get(&my_pubkey) else {
            continue;
        };

        // Another event may carry a version we can read, so a failure here
        // moves on to the next candidate rather than ending the command.
        let decrypted = match keys.nip44_decrypt(&event.pubkey, ciphertext).await {
            Ok(decrypted) => decrypted,
            Err(e) => {
                log::warn(&format!("Skipped an entry that would not decrypt: {}", e));
                continue;
            }
        };

        std::fs::write(".env", &decrypted)
            .map_err(|e| format!("Decrypted the secrets but could not write .env: {}", e))?;

        log::success(&format!("Wrote .env from tag \"{}\"", tag));
        return Ok(());
    }

    Err(format!("No secrets under tag \"{}\" list you as a recipient", tag).into())
}

/// Works out which pubkey is allowed to publish `tag`, and remembers it.
///
/// `--owner` is the human trust decision, so it is taken as given and pinned
/// for next time. Without it the pin is the only answer available: guessing,
/// or accepting whichever author replied first, is exactly the substitution
/// this is here to prevent.
fn resolve_owner(tag: &str, owner: Option<&str>) -> Result<PublicKey, Box<dyn std::error::Error>> {
    if let Some(owner) = owner {
        let owner_pubkey = PublicKey::parse(owner)
            .map_err(|e| format!("'{}' is not a valid public key: {}", owner, e))?;

        // Stored in canonical bech32 so the same key typed as hex and as npub
        // does not read back as a different owner.
        let canonical = owner_pubkey.to_bech32()?;

        // Re-pointing a tag at a different identity is a real trust change,
        // and the user asked for it, but it should not happen quietly.
        if let Some(pinned) = get_trusted_owner(tag)
            && pinned != canonical
        {
            log::warn(&format!(
                "Tag \"{}\" was trusted to {}, now trusting {}",
                tag, pinned, canonical
            ));
        }

        // The pull itself is already safe: it uses the pubkey the user just
        // supplied. Only the convenience of not retyping it is lost here.
        if let Err(e) = save_trusted_owner(tag, &canonical) {
            log::warn(&format!(
                "Could not remember the owner for next time: {}",
                e
            ));
        }

        return Ok(owner_pubkey);
    }

    let Some(pinned) = get_trusted_owner(tag) else {
        return Err(format!(
            "No trusted owner for tag \"{}\". Run `envo pull {} --owner <npub>` once to set it.",
            tag, tag
        )
        .into());
    };

    let owner_pubkey = PublicKey::parse(&pinned).map_err(|e| {
        format!(
            "The stored owner for tag \"{}\" is unreadable ({}). Run `envo pull {} --owner <npub>` to set it again.",
            tag, e, tag
        )
    })?;

    Ok(owner_pubkey)
}
