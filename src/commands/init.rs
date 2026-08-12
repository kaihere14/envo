//complete work is left for now this is just a testing code for my understanding
pub fn init(tag: String, env: &Vec<String>, trusted_content: &Vec<String>) {
    let keys = match crate::commands::key_gen::require_keys() {
        Ok(keys) => keys,
        Err(e) => {
            eprintln!("error: could not load keys: {}", e);
            return;
        }
    };

    println!(
        "Successfully loaded keys {:?}, using tag: {} ",
        keys.public_key, tag
    );

    println!("Your current env content is : {:?}", env);

    for s in trusted_content {
        println!("{}", s);
    }
}
