use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use rocket::State;
use std::sync::Mutex;

extern crate serde_json;

use crate::security::keystore::calculate_hash;
use crate::security::keystore::KeyManager;

pub struct ApiKeySubscriber(String);

/// Returns true if `key` is a valid API key string.
fn is_valid(key: &str, hashes: Vec<String>) -> bool {
    for hash in hashes {
        if calculate_hash(key.to_string()) == hash {
            return true;
        }
    }
    false
}

#[derive(Debug)]
pub enum ApiKeyError {
    BadCount,
    Missing,
    Invalid,
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKeySubscriber {
    type Error = ApiKeyError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("x-api-key").collect();
        let store = request.guard::<State<Mutex<KeyManager>>>().unwrap();
        let hashes = store.lock().expect("").keystore.api_key_subscribers.clone();
        match keys.len() {
            0 => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            1 if is_valid(keys[0], hashes) => {
                Outcome::Success(ApiKeySubscriber(keys[0].to_string()))
            }
            1 => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
            _ => Outcome::Failure((Status::BadRequest, ApiKeyError::BadCount)),
        }
    }
}
