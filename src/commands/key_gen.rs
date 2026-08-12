use crate::helper::key_valid::*;
use nostr_sdk::prelude::*;
use std::path::Path;

/// Generates the local Nostr keypair, or reports the existing one.
/// Safe to re-run: valid keys already on disk are kept as they are.
pub fn key_gen() {
    let key_dir = gen_key_dir();

    if let Some(keys) = read_existing_keys(&key_dir) {
        println!("Found existing keys at {}", key_dir.display());

        match keys.public_key().to_bech32() {
            Ok(public_key) => println!("Your public key: {}", public_key),
            Err(_) => panic!("Failed to display public key"),
        };

        return;
    }

    create_new_keys(&key_dir);
}

/// Reads the key file and returns the keypair when it holds a valid one.
/// `None` means the caller should generate a fresh pair: either the file is
/// empty or the keys in it did not pass validation.
fn read_existing_keys(key_dir: &Path) -> Option<Keys> {
    // An unreadable file is treated the same as an empty one: we fall
    // through and generate a fresh keypair below.
    let has_contents;

    match std::fs::read_to_string(key_dir) {
        Ok(contents) => has_contents = contents,
        Err(_) => has_contents = String::new(),
    }

    if has_contents.is_empty() {
        return None;
    }

    // A damaged key file is recoverable by regenerating, so warn and fall
    // through rather than crashing the whole command.
    let contents: serde_json::Value = match serde_json::from_str(&has_contents) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("warning: could not parse {}: {}", key_dir.display(), e);
            return None;
        }
    };

    let public_key;
    match contents["npub"].as_str() {
        Some(key) => public_key = key,
        None => public_key = "",
    };

    let private_key;
    match contents["nsec"].as_str() {
        Some(key) => private_key = key,
        None => private_key = "",
    };

    let valid_keypair = is_valid_keypair(&public_key, &private_key);

    if !valid_keypair {
        eprintln!("warning: existing keys are invalid or corrupted, regenerating");
        return None;
    }

    // Hand back a real `Keys` so callers can sign and do ECDH straight away
    // instead of re-parsing the bech32 text themselves.
    match Keys::parse(private_key) {
        Ok(keys) => Some(keys),
        Err(_) => {
            eprintln!("warning: stored secret key could not be parsed, regenerating");
            None
        }
    }
}

/// Generates a fresh keypair, writes it to the key file and returns it.
fn create_new_keys(key_dir: &Path) -> Keys {
    let keys = Keys::generate();

    let public_key: String;
    let private_key: String;

    match keys.public_key().to_bech32() {
        Ok(key) => public_key = key,
        Err(_) => panic!("Failed to generate public key"),
    };

    match keys.secret_key().to_bech32() {
        Ok(key) => private_key = key,
        Err(_) => panic!("Failed to generate private key"),
    };

    if public_key.len() != private_key.len() || public_key.is_empty() || private_key.is_empty() {
        panic!("There was no public key or private key generated");
    }

    let contents = format!(r#"{{"npub": "{}", "nsec": "{}"}}"#, public_key, private_key);

    let result = std::fs::write(key_dir, contents);
    match result {
        Ok(_) => {
            println!("Keys saved to {}", key_dir.display());
            println!("Your public key (safe to share): {}", public_key);
        }
        Err(e) => panic!("Failed to write keys to file: {}", e),
    }

    keys
}

/// Loads the stored identity as a `Keys`, generating one on first run.
/// Callers get something they can sign and encrypt with directly.
pub fn load_keys() -> Result<Keys, Box<dyn std::error::Error>> {
    let key_dir = gen_key_dir();

    // Both branches reuse the helpers above, so reading, parsing and
    // validating the key file only ever happens in one place.
    if let Some(keys) = read_existing_keys(&key_dir) {
        return Ok(keys);
    }

    println!("No keys found, generating new keys...");

    Ok(create_new_keys(&key_dir))
}
