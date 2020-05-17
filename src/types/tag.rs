use rocket::request::FromParam;
use rocket::http::RawStr;

pub struct Tag<'r>{
    pub val: &'r str
}

#[derive(Debug)]
pub enum TagError {
    Invalid,
    Missing
}

impl<'r> FromParam<'r> for Tag<'r> {
    type Error = TagError;

    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
        let tag = param;

        if !tag.chars().all(|c| (c == '9') || (c >= 'A' && c <= 'Z')) {
            return Err(TagError::Invalid);
        }

        if !(tag.chars().count() == 5usize){
            return Err(TagError::Invalid);
        }

        Ok(Tag{val:tag})     
    }
}