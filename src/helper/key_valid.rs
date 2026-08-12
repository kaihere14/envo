use std::path::PathBuf;

/// Checks that both keys are present and look like bech32 Nostr keys.
pub fn is_valid_keypair(public_key: &str, private_key: &str) -> bool {
    !public_key.is_empty()
        && !private_key.is_empty()
        && public_key.len() == private_key.len()
        && public_key.starts_with("npub1")
        && private_key.starts_with("nsec1")
}

/// Returns the path to `~/.envo/keys.json`, creating the directory and an
/// empty key file if they are missing.
pub fn gen_key_dir() -> PathBuf {
    let home_dir = dirs::home_dir().expect("Failed to locate home directory");
    let envo_dir = home_dir.join(".envo");

    if !envo_dir.exists() {
        std::fs::create_dir_all(&envo_dir).expect("Failed to create the envo folder");
    }

    let key_dir = envo_dir.join("keys.json");

    if !std::path::Path::new(&key_dir).exists() {
        println!("Creating key file at {}", key_dir.display());
        std::fs::File::create(&key_dir).expect("Failed to create the key.json file");
    }
    return key_dir;
}
