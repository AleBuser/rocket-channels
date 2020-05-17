use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Keystore {
    pub api_key_subscriber: u64,
    pub api_key_author: u64,
}

impl Keystore {
    pub fn new(new_key_sub: String, new_key_aut: String) -> Keystore {
        Keystore {
            api_key_subscriber: calculate_hash(&new_key_sub),
            api_key_author: calculate_hash(&new_key_aut),
        }
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
