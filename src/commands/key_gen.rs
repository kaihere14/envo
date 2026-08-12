use crate::helper::key_valid::*;
use nostr_sdk::prelude::*;

pub fn key_gen() {
    let key_dir = gen_key_dir();

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

    if public_key.len() != private_key.len() || public_key.is_empty() || private_key.is_empty() {
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

pub fn load_keys() -> Result<(String, String), Box<dyn std::error::Error>> {
    let key_dir = gen_key_dir();
    let mut contents = std::fs::read_to_string(&key_dir)?;

    if contents.is_empty() {
        println!("Keys dosent exist, generating new keys...");
        key_gen();
        contents = std::fs::read_to_string(&key_dir)?;
    }

    let keys: serde_json::Value = serde_json::from_str(&contents)?;
    let public_key = keys["npub"].as_str().ok_or("npub not found")?.to_string();
    let private_key = keys["nsec"].as_str().ok_or("nsec not found")?.to_string();

    let valid_pair = is_valid_keypair(&public_key, &private_key);
    if !valid_pair {
        panic!("Invalid keypair: public key and private key must be the same length and non-empty");
    }

    Ok((public_key, private_key))
}
