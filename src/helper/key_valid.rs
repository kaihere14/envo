use std::path::PathBuf;

pub fn is_valid_keypair(public_key: &str, private_key: &str) -> bool {
    !public_key.is_empty()
        && !private_key.is_empty()
        && public_key.len() == private_key.len()
        && public_key.starts_with("npub1")
        && private_key.starts_with("nsec1")
}

pub fn gen_key_dir() -> PathBuf {
    let home_dir = dirs::home_dir().unwrap();
    let envo_dir = home_dir.join(".envo");
    if !envo_dir.exists() {
        std::fs::create_dir_all(&envo_dir).unwrap();
    }
    let key_dir = envo_dir.join("keys.json");
    if !std::path::Path::new(&key_dir).exists() {
        println!("key file dosent exist");
        std::fs::File::create(&key_dir).unwrap();
    }
    return key_dir;
}
