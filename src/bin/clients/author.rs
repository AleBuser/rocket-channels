use reqwest;
use reqwest::Url;

use std::{thread, time};

use serde_json::Result;

pub struct Author {
    api_key: String,
}

impl Author {
    pub fn new() -> Self {
        Self {
            api_key: "API_AUT".to_string(),
        }
    }

    async fn write_pubic(&mut self, msg: String) -> Result<()> {
        let client = reqwest::Client::new();

        let url_par = "http://0.0.0.0:8000/write_public/".to_owned() + &msg;

        &client
            .post(Url::parse(&url_par).unwrap())
            .header("x-api-key", self.api_key.clone())
            .send()
            .await
            .unwrap();

        println!("Sent Message: {}", msg.clone());

        Ok(())
    }
}
#[tokio::main]
async fn main() {
    let mut author = Author::new();

    let mut c = 0u32;
    loop {
        let msg = format!("test msg nr. {}", &c);

        author.write_pubic(msg.to_string()).await.unwrap();
        thread::sleep(time::Duration::from_secs(15));

        c = &c + 1;
    }
}