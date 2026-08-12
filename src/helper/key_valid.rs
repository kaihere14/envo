pub fn is_valid_keypair(public_key: &str, private_key: &str) -> bool {
    !public_key.is_empty()
        && !private_key.is_empty()
        && public_key.len() == private_key.len()
        && public_key.starts_with("npub1")
        && private_key.starts_with("nsec1")
}
