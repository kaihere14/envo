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

/// Encrypts `env` for every trusted pubkey and publishes it under `tag`.
///
/// Returns `Err` for anything that stopped the secrets being published, so the
/// process exit status says whether the push happened. A script chaining off
/// `envo push` has no other way to tell.
pub async fn push(
    tag: String,
    env: &[String],
    trusted_content: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    let keys = crate::commands::key_gen::require_keys()?;

    log::step(&format!("Publishing secrets under tag \"{}\"", tag));

    let env_as_string = env.join("\n");

    let owner_npub = keys
        .public_key()
        .to_bech32()
        .map_err(|e| format!("Could not encode your public key: {}", e))?;

    let mut recipients: HashMap<String, String> = HashMap::new();
    let mut teammates = 0;

    for npub_str in trusted_content {
        let recipient_pubkey = match PublicKey::parse(npub_str) {
            Ok(pk) => pk,
            Err(e) => {
                log::warn(&format!("Skipped invalid pubkey '{}': {}", npub_str, e));
                continue;
            }
        };

        // The publisher gets an entry below, keyed by their canonical npub.
        // Listing themselves in .env-share as well would otherwise encrypt
        // the same secrets twice, and if they wrote it in hex the duplicate
        // would be keyed in a form `pull` never looks up.
        if recipient_pubkey == keys.public_key() {
            continue;
        }

        match env_encrypt(&keys, &recipient_pubkey, &env_as_string).await {
            Ok(ciphertext) => {
                recipients.insert(npub_str.clone(), ciphertext);
                teammates += 1;
            }
            Err(e) => {
                log::warn(&format!("Could not encrypt for {}: {}", npub_str, e));
            }
        }
    }

    // Publishing secrets you cannot read back is never what was meant: the
    // recipients map is the only copy of the ciphertext, so without an entry
    // addressed to themselves the publisher could not pull their own tag on
    // another machine. Keyed by canonical bech32 because that is what `pull`
    // looks itself up by.
    match env_encrypt(&keys, &keys.public_key(), &env_as_string).await {
        Ok(ciphertext) => {
            recipients.insert(owner_npub, ciphertext);
        }
        Err(e) => {
            log::warn(&format!("Could not encrypt for yourself: {}", e));
        }
    }

    // An empty .env-share is a legitimate solo push, but entries that all
    // failed to parse mean the file says something the user did not intend.
    if teammates == 0 && !trusted_content.is_empty() {
        return Err("No valid recipients in .env-share, nothing was published".into());
    }

    if recipients.is_empty() {
        return Err("Nothing could be encrypted, nothing was published".into());
    }

    log::success(&format!(
        "Encrypted for {} recipient(s), including you",
        recipients.len()
    ));

    let event = build_and_sign_init_event(&tag, &keys, recipients)
        .await
        .map_err(|e| format!("Could not sign the event: {}", e))?;

    let relay_url: Vec<String> = get_relay_urls();

    publish_event(&keys, event, &relay_url)
        .await
        .map_err(|e| format!("Could not publish the event: {}", e))?;

    log::success(&format!("Published tag \"{}\"", tag));

    Ok(())
}
