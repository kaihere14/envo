use nostr_sdk::prelude::*;

pub async fn env_encrypt(
    owner_keys: &Keys,
    recipient_pubkey: &PublicKey,
    env_content: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let encrypted = owner_keys
        .nip44_encrypt(recipient_pubkey, env_content)
        .await?;
    Ok(encrypted)
}
