use crate::helper::event_content::EventContent;
use crate::nostr::fetch_event::fetch_event;
use nostr_sdk::prelude::*; // adjust path to wherever this struct actually lives

pub async fn pull(tag: String) {
    let keys = match crate::commands::key_gen::require_keys() {
        Ok(keys) => keys,
        Err(e) => {
            eprintln!("error: could not load keys: {}", e);
            return;
        }
    };
    println!("Keys loaded successfully fetching events for: {}", &tag);

    let events = match fetch_event(&tag, &keys).await {
        Ok(events) => events,
        Err(e) => {
            eprintln!("error: could not fetch events: {}", e);
            return;
        }
    };

    let my_pubkey = match keys.public_key().to_bech32() {
        Ok(pk) => pk,
        Err(e) => {
            eprintln!("error: could not encode your public key: {}", e);
            return;
        }
    };

    let mut found = false;

    for event in events {
        let content: EventContent = match serde_json::from_str(&event.content) {
            Ok(c) => c,
            Err(_) => {
                eprintln!("warning: skipping event with malformed content");
                continue;
            }
        };

        if let Some(ciphertext) = content.recipients.get(&my_pubkey) {
            let decrypted = match keys.nip44_decrypt(&event.pubkey, ciphertext).await {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("error: found your entry but failed to decrypt: {}", e);
                    continue;
                }
            };

            match std::fs::write(".env", &decrypted) {
                Ok(_) => {
                    println!("Pulled and decrypted successfully. .env written.");
                    found = true;
                }
                Err(e) => {
                    eprintln!(
                        "error: decrypted successfully but failed to write .env: {}",
                        e
                    );
                }
            }

            break;
        }
    }

    if !found {
        eprintln!(
            "error: no event found where you are a trusted recipient for tag '{}'",
            tag
        );
    }
}
