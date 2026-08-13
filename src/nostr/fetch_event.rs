use nostr_sdk::prelude::*;

use crate::helper::relay_provider::get_relay_urls;

pub async fn fetch_event(
    tag: &String,
    keys: &Keys,
) -> Result<Vec<Event>, Box<dyn std::error::Error>> {
    println!("Connecting to relays to fetch tag: {}", tag);

    let client = Client::new(keys.clone());
    let relay_urls = get_relay_urls();

    let mut connected_count = 0;
    for url in &relay_urls {
        match client.add_relay(url).await {
            Ok(_) => {
                connected_count += 1;
            }
            Err(e) => {
                eprintln!("warning: could not add relay {}: {}", url, e);
            }
        }
    }
    println!("Added {}/{} relay(s)", connected_count, relay_urls.len());

    client.connect().await;
    println!("Connected, querying for tag '{}'...", tag);

    let filter = Filter::new()
        .kind(Kind::Custom(30078))
        .identifier(tag.clone());

    let events = client
        .fetch_events(filter, std::time::Duration::from_secs(10))
        .await?;

    let events_vec: Vec<Event> = events.into_iter().collect();
    println!(
        "Found {} candidate event(s) for tag '{}'",
        events_vec.len(),
        tag
    );

    client.disconnect().await;
    println!("Disconnected from relays");

    Ok(events_vec)
}
