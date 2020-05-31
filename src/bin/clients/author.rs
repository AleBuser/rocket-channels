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

    async fn write_masked(&mut self, msg: String) -> Result<()> {
        let client = reqwest::Client::new();

        let url_par = "http://0.0.0.0:8000/write_masked/".to_owned() + &msg;

        &client
            .post(Url::parse(&url_par).unwrap())
            .header("x-api-key", self.api_key.clone())
            .send()
            .await
            .unwrap();

        println!("Sent Message: {}", msg.clone());

        Ok(())
    }

    async fn write_tagged(&mut self, msg: String) -> Result<()> {
        let client = reqwest::Client::new();

        let url_par = "http://0.0.0.0:8000/write_tagged/".to_owned() + &msg;

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
        let msg = format!("test public msg nr. {}", &c);
        author.write_pubic(msg.to_string()).await.unwrap();
        thread::sleep(time::Duration::from_secs(20));

        /*
        let msg = format!("test masked msg nr. {}", &c);
        author.write_masked(msg.to_string()).await.unwrap();
        thread::sleep(time::Duration::from_secs(10));

        let msg = format!("test tagged msg nr. {}", &c);
        author.write_masked(msg.to_string()).await.unwrap();
        thread::sleep(time::Duration::from_secs(10));
        */
        c = &c + 1;
    }
}
