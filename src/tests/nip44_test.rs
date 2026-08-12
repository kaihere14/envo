use nostr_sdk::prelude::*;

#[tokio::test]
pub async fn test_nip44_roundtrip() {
    let owner_keys = Keys::generate();
    let test_recipient = Keys::generate();
    let plaintext = "arman";

    let encrypted = owner_keys
        .nip44_encrypt(&test_recipient.public_key(), plaintext)
        .await
        .unwrap();

    let decrypted = test_recipient
        .nip44_decrypt(&owner_keys.public_key(), &encrypted)
        .await
        .unwrap();

    assert_eq!(decrypted, plaintext);
}
