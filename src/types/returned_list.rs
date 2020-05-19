
use serde_derive::Serialize;


#[derive(Serialize, Debug)]
pub struct ReturnedList {
    pub status: &'static str,
    pub list: Vec<String>
}