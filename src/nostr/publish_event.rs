use nostr_sdk::prelude::*;

pub async fn publish_event(
    keys: &Keys,
    event: Event,
    relay_url: &Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Build the client, tied to your signing keys
    let client = Client::new(keys.clone());

    // 2. Add relay(s)
    for url in relay_url {
        client.add_relay(url).await?;
    }

    // 3. Connect
    client.connect().await;

    // 4. Publish

    match client.send_event(&event).await {
        Ok(_output) => {}
        Err(e) => eprintln!("error: publish failed entirely: {}", e),
    }

    println!("Event published successfully!");

    // 5. Disconnect once done
    client.disconnect().await;

    Ok(())
}
