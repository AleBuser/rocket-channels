
use serde_derive::Serialize;

#[derive(Serialize, Debug)]
pub struct Response {
    pub status: &'static str,
    pub message: String
}