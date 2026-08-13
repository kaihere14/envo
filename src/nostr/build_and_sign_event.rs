use nostr_sdk::prelude::*;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
struct EventContent {
    version: u32,
    recipients: HashMap<String, String>,
}

pub async fn build_and_sign_init_event(
    tag: &str,
    owner_keys: &Keys,
    recipients: HashMap<String, String>,
) -> Result<Event, Box<dyn std::error::Error>> {
    // Step 1 — serialize recipients into content JSON
    let content = EventContent {
        version: 1,
        recipients: recipients.clone(),
    };
    let content_json = serde_json::to_string(&content)?;

    // Step 2 — build tags
    let mut tags: Vec<Tag> = vec![
        Tag::identifier(tag.to_string()),
        Tag::custom(TagKind::Custom("v".into()), vec!["1".to_string()]),
    ];

    for pubkey_str in recipients.keys() {
        let pubkey = PublicKey::parse(pubkey_str)?;
        tags.push(Tag::public_key(pubkey));
    }

    // Step 3 — build event and sign
    let event_builder = EventBuilder::new(Kind::Custom(30078), content_json).tags(tags);

    let event = event_builder.sign_with_keys(owner_keys)?;

    Ok(event)
}
