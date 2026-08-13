use crate::{
    helper::log,
    helper::relay_provider::get_relay_urls,
    nostr::{
        build_and_sign_event::build_and_sign_init_event, encrypter::env_encrypt,
        publish_event::publish_event,
    },
};
use nostr_sdk::prelude::*;
use std::collections::HashMap;

pub async fn push(tag: String, env: &[String], trusted_content: &[String]) {
    let keys = match crate::commands::key_gen::require_keys() {
        Ok(keys) => keys,
        Err(e) => {
            log::fail(&format!("{}", e));
            return;
        }
    };

    log::step(&format!("Publishing secrets under tag \"{}\"", tag));

    let env_as_string = env.join("\n");

    let mut recipients: HashMap<String, String> = HashMap::new();

    for npub_str in trusted_content {
        let recipient_pubkey = match PublicKey::parse(npub_str) {
            Ok(pk) => pk,
            Err(e) => {
                log::warn(&format!("Skipped invalid pubkey '{}': {}", npub_str, e));
                continue;
            }
        };

        match env_encrypt(&keys, &recipient_pubkey, &env_as_string).await {
            Ok(ciphertext) => {
                recipients.insert(npub_str.clone(), ciphertext);
            }
            Err(e) => {
                log::warn(&format!("Could not encrypt for {}: {}", npub_str, e));
            }
        }
    }

    if recipients.is_empty() {
        log::fail("No valid recipients in .env-share, nothing was published");
        return;
    }

    log::success(&format!("Encrypted for {} recipient(s)", recipients.len()));

    let event = build_and_sign_init_event(&tag, &keys, recipients).await;

    let relay_url: Vec<String> = get_relay_urls();

    match event {
        Ok(event) => {
            if let Err(e) = publish_event(&keys, event, &relay_url).await {
                log::fail(&format!("Could not publish the event: {}", e));
                return;
            }
            log::success(&format!("Published tag \"{}\"", tag));
        }
        Err(e) => {
            log::fail(&format!("Could not sign the event: {}", e));
        }
    }
}
