use crate::helper::event_content::EventContent;
use crate::helper::log;
use crate::nostr::fetch_event::fetch_event;
use nostr_sdk::prelude::*;

pub async fn pull(tag: String) {
    let keys = match crate::commands::key_gen::require_keys() {
        Ok(keys) => keys,
        Err(e) => {
            log::fail(&format!("{}", e));
            return;
        }
    };

    let my_pubkey = match keys.public_key().to_bech32() {
        Ok(pubkey) => pubkey,
        Err(e) => {
            log::fail(&format!("Could not encode your public key: {}", e));
            return;
        }
    };

    log::step(&format!("Fetching secrets for tag \"{}\"", tag));

    let events = match fetch_event(&tag, &keys).await {
        Ok(events) => events,
        Err(e) => {
            log::fail(&format!("Could not reach the relays: {}", e));
            return;
        }
    };

    if events.is_empty() {
        log::fail(&format!("No secrets published under tag \"{}\"", tag));
        return;
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

        if let Err(e) = std::fs::write(".env", &decrypted) {
            log::fail(&format!(
                "Decrypted the secrets but could not write .env: {}",
                e
            ));
            return;
        }

        log::success(&format!("Wrote .env from tag \"{}\"", tag));
        return;
    }

    log::fail(&format!(
        "No secrets under tag \"{}\" list you as a recipient",
        tag
    ));
}
