use crate::helper::key_valid::*;
use crate::helper::log;
use crate::helper::secret_file::write_secret;
use nostr_sdk::prelude::*;
use std::io::Write;
use std::path::Path;

/// Generates the local Nostr keypair, or reports the existing one.
/// Safe to re-run: valid keys already on disk are kept as they are.
pub fn key_gen() {
    let key_dir = gen_key_dir();

    if let Some(keys) = read_existing_keys(&key_dir) {
        log::success(&format!(
            "Found an existing identity at {}",
            key_dir.display()
        ));

        match keys.public_key().to_bech32() {
            Ok(public_key) => log::step(&format!("Public key: {}", public_key)),
            Err(_) => panic!("Failed to display public key"),
        };

        return;
    }

    if !confirm("No valid keys found. Generate a new identity?") {
        log::warn("Aborted, no keys were generated");
        return;
    }

    create_new_keys(&key_dir);
}

/// Prompts the user with a yes/no question, defaulting to no.
fn confirm(prompt: &str) -> bool {
    print!("{} [y/N]: ", prompt);
    std::io::stdout().flush().ok();

    let mut input = String::new();
    if std::io::stdin().read_line(&mut input).is_err() {
        return false;
    }

    let answer = input.trim().to_lowercase();
    answer == "y" || answer == "yes"
}

/// Reads the key file and returns the keypair when it holds a valid one.
/// `None` means the caller should generate a fresh pair: either the file is
/// empty or the keys in it did not pass validation.
fn read_existing_keys(key_dir: &Path) -> Option<Keys> {
    // An unreadable file is treated the same as an empty one: we fall
    // through and generate a fresh keypair below.
    let has_contents = std::fs::read_to_string(key_dir).unwrap_or_default();

    if has_contents.is_empty() {
        return None;
    }

    // A damaged key file is recoverable by regenerating, so warn and fall
    // through rather than crashing the whole command.
    let contents: serde_json::Value = match serde_json::from_str(&has_contents) {
        Ok(contents) => contents,
        Err(e) => {
            log::warn(&format!("Could not parse {}: {}", key_dir.display(), e));
            return None;
        }
    };

    let public_key = contents["npub"].as_str().unwrap_or_default();
    let private_key = contents["nsec"].as_str().unwrap_or_default();

    let valid_keypair = is_valid_keypair(public_key, private_key);

    if !valid_keypair {
        log::warn("Existing keys are invalid or corrupted");
        return None;
    }

    // Hand back a real `Keys` so callers can sign and do ECDH straight away
    // instead of re-parsing the bech32 text themselves.
    match Keys::parse(private_key) {
        Ok(keys) => Some(keys),
        Err(_) => {
            log::warn("Stored secret key could not be parsed");
            None
        }
    }
}

/// Generates a fresh keypair, writes it to the key file and returns it.
fn create_new_keys(key_dir: &Path) -> Keys {
    let keys = Keys::generate();

    let public_key: String = match keys.public_key().to_bech32() {
        Ok(key) => key,
        Err(_) => panic!("Failed to generate public key"),
    };

    let private_key: String = match keys.secret_key().to_bech32() {
        Ok(key) => key,
        Err(_) => panic!("Failed to generate private key"),
    };

    if public_key.len() != private_key.len() || public_key.is_empty() || private_key.is_empty() {
        panic!("There was no public key or private key generated");
    }

    let contents = format!(r#"{{"npub": "{}", "nsec": "{}"}}"#, public_key, private_key);

    // Owner-only permissions: the secret key is the entire access model, so
    // it must not be readable by other accounts on the machine.
    match write_secret(key_dir, &contents) {
        Ok(_) => {
            log::success(&format!("Created a new identity at {}", key_dir.display()));
            log::step(&format!("Public key (safe to share): {}", public_key));
        }
        Err(e) => panic!("Failed to write keys to file: {}", e),
    }

    keys
}

/// Loads the stored identity as a `Keys`, failing when there is none.
/// Callers get something they can sign and encrypt with directly.
///
/// Generating a keypair here would let a command publish under a brand new
/// identity without the user noticing, so creating keys stays the job of
/// `key_gen` alone.
pub fn require_keys() -> Result<Keys, Box<dyn std::error::Error>> {
    let key_dir = gen_key_dir();

    match read_existing_keys(&key_dir) {
        Some(keys) => Ok(keys),
        None => Err("No identity found. Run `envo keygen` first.".into()),
    }
}
