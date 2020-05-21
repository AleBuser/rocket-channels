use crypto::digest::Digest;
use crypto::sha1::Sha1;

#[derive(Debug)]
pub struct Keystore {
    pub api_key_subscriber: String,
    pub api_key_author: String,
}

impl Keystore {
    pub fn new(new_key_sub: String, new_key_aut: String) -> Keystore {
        Keystore {
            api_key_subscriber: calculate_hash(new_key_sub),
            api_key_author: calculate_hash(new_key_aut),
        }
    }
}

pub fn calculate_hash(t: String) -> String {
    let mut hasher = Sha1::new();
    hasher.input_str(&t);
    let hex = hasher.result_str();
    hex
}
