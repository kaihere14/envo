use crate::helper::key_valid::is_valid_keypair;
use nostr_sdk::prelude::*;
use std::path::PathBuf;

pub fn key_gen() {
    let home: PathBuf;

    match dirs::home_dir() {
        Some(home_dir) => home = home_dir,
        None => panic!("Failed to get home directory"),
    };

    let envo_dir = home.join(".envo");

    if !envo_dir.exists() {
        match std::fs::create_dir(&envo_dir) {
            Ok(_) => {
                println!("File didnt exist, created .envo directory")
            }
            Err(e) => panic!("Failed to create .envo directory: {}", e),
        }
    }

    let key_dir = envo_dir.join("keys.json");

    if !key_dir.exists() {
        match std::fs::File::create(&key_dir) {
            Ok(_) => {
                println!("File didnt exist, created keys.json")
            }
            Err(e) => panic!("Failed to create keys.json: {}", e),
        }
    }

    let has_contents;
    match std::fs::read_to_string(&key_dir) {
        Ok(contents) => has_contents = contents,
        Err(_) => has_contents = String::new(),
    }

    if !has_contents.is_empty() {
        println!("keys.json already exists at {}", key_dir.display());
        let contents: serde_json::Value = serde_json::from_str(&has_contents).unwrap();

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
            println!("The keys were incorrect or corupter regenerating");
        } else {
            println!("Your public key : {}", public_key);
            return;
        }
    }

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

    if public_key.len() != private_key.len() && public_key.len() == 0 && private_key.len() == 0 {
        panic!("There was no public key or private key generated");
    }

    let contents = format!(r#"{{"npub": "{}", "nsec": "{}"}}"#, public_key, private_key);

    let result = std::fs::write(&key_dir, contents);
    match result {
        Ok(_) => {
            println!("Keys saved to {}", key_dir.display());
            println!("Your public key for sharing is : {}", public_key);
        }
        Err(e) => panic!("Failed to write keys to file: {}", e),
    }
}
