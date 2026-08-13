use nostr_sdk::prelude::*;

use crate::helper::relay_provider::get_relay_urls;

use crate::helper::log;

/// Queries every configured relay for the addressable events published under
/// `tag`. Reports only what the user can act on: the calling command owns the
/// progress reporting for the run as a whole.
pub async fn fetch_event(tag: &str, keys: &Keys) -> Result<Vec<Event>, Box<dyn std::error::Error>> {
    let client = Client::new(keys.clone());
    let relay_urls = get_relay_urls();

    let mut connected_count = 0;
    for url in &relay_urls {
        match client.add_relay(url).await {
            Ok(_) => {
                connected_count += 1;
            }
            Err(e) => {
                log::warn(&format!("Relay {} unavailable: {}", url, e));
            }
        }
    }

    if connected_count == 0 {
        return Err("no relays could be reached".into());
    }

    client.connect().await;

    let filter = Filter::new()
        .kind(Kind::Custom(30078))
        .identifier(tag.to_owned());

    let events = client
        .fetch_events(filter, std::time::Duration::from_secs(10))
        .await?;

    let events_vec: Vec<Event> = events.into_iter().collect();

    client.disconnect().await;

    Ok(events_vec)
}
