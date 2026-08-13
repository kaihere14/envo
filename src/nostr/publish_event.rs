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

    // 4. Publish. The error travels back to the caller so it is the one
    //    deciding what the user sees.
    let result = client.send_event(&event).await;

    // 5. Disconnect once done, whether or not the send worked
    client.disconnect().await;

    result?;

    Ok(())
}
