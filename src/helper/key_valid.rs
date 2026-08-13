use crate::helper::secret_file::{DIR_MODE, FILE_MODE, restrict};
use std::path::PathBuf;

/// Checks that both keys are present and look like bech32 Nostr keys.
pub fn is_valid_keypair(public_key: &str, private_key: &str) -> bool {
    !public_key.is_empty()
        && !private_key.is_empty()
        && public_key.len() == private_key.len()
        && public_key.starts_with("npub1")
        && private_key.starts_with("nsec1")
}

/// Returns the path to `~/.envo`, creating it if it is missing.
///
/// Restricting runs on every call, not just on creation, so a directory left
/// world-readable by an older version gets tightened the next time envo runs.
pub fn envo_dir() -> PathBuf {
    let home_dir = dirs::home_dir().expect("Failed to locate home directory");
    let envo_dir = home_dir.join(".envo");

    if !envo_dir.exists() {
        std::fs::create_dir_all(&envo_dir).expect("Failed to create the envo folder");
    }

    restrict(&envo_dir, DIR_MODE).expect("Failed to restrict permissions on the envo folder");

    envo_dir
}

/// Returns the path to `~/.envo/keys.json`, creating the directory and an
/// empty key file if they are missing.
pub fn gen_key_dir() -> PathBuf {
    let key_dir = envo_dir().join("keys.json");

    // Creating the empty file is bookkeeping, not something the user asked
    // for, so it stays silent: `key_gen` reports the identity it writes.
    if !std::path::Path::new(&key_dir).exists() {
        std::fs::File::create(&key_dir).expect("Failed to create the key.json file");
    }

    // Runs on every command, not just on creation, so an identity written by
    // an older version gets tightened the next time envo is used. A secret we
    // cannot restrict is a secret we should not go on to use.
    restrict(&key_dir, FILE_MODE).expect("Failed to restrict permissions on the key file");

    key_dir
}
