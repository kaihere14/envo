use crate::{
    helper::relay_provider::get_relay_urls,
    nostr::{
        build_and_sign_event::build_and_sign_init_event, encrypter::env_encrypt,
        publish_event::publish_event,
    },
};
use nostr_sdk::prelude::*;
use std::collections::HashMap;

pub async fn init(tag: String, env: &[String], trusted_content: &[String]) {
    let keys = match crate::commands::key_gen::require_keys() {
        Ok(keys) => keys,
        Err(e) => {
            eprintln!("error: could not load keys: {}", e);
            return;
        }
    };

    let env_as_string = env.join("\n");

    let mut recipients: HashMap<String, String> = HashMap::new();

    for npub_str in trusted_content {
        let recipient_pubkey = match PublicKey::parse(npub_str) {
            Ok(pk) => pk,
            Err(e) => {
                eprintln!("error: invalid pubkey '{}': {}", npub_str, e);
                continue;
            }
        };

        match env_encrypt(&keys, &recipient_pubkey, &env_as_string).await {
            Ok(ciphertext) => {
                recipients.insert(npub_str.clone(), ciphertext);
            }
            Err(e) => {
                eprintln!("error: failed to encrypt for {}: {}", npub_str, e);
            }
        }
    }

    println!("Encrypted for {} recipient(s)", recipients.len());

    let event = build_and_sign_init_event(&tag, &keys, recipients).await;

    let relay_url: Vec<String> = get_relay_urls();

    match event {
        Ok(event) => {
            println!("Event built and signed successfully");
            if let Err(e) = publish_event(&keys, event, &relay_url).await {
                eprintln!("error: failed to publish event: {}", e);
            }
        }
        Err(e) => {
            eprintln!("error: {}", e);
        }
    }
}
