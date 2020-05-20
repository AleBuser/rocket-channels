
use serde_derive::Serialize;
use serde_derive::Deserialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnedList{
    pub status: &'static str,
    pub list: Vec<String>
}