
use serde_derive::Serialize;
use serde_derive::Deserialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub status: &'static str,
    pub message: String
}
