use nostr_sdk::prelude::*;

pub fn key_gen() {
    let keys = Keys::generate();

    println!("Your generated key pairs are : ");

    match keys.public_key().to_bech32() {
        Ok(pubkey_str) => println!("Public key: {}", pubkey_str),
        Err(e) => println!("Failed to encode pubkey: {}", e),
    }

    match keys.secret_key().to_bech32() {
        Ok(pvtkey_str) => println!("Private key: {}", pvtkey_str),
        Err(e) => println!("Failed to encode pvtkey: {}", e),
    }
}
