use reqwest;
use reqwest::Url;

use handlebars::Handlebars;
#[macro_use]
use serde_json::Result;
use serde_json::json;
use std::{thread, time};

pub struct Author {
    api_key: String,
    base_url: Handlebars<'static>,
}

impl Author {
    pub fn new(base_url: &str) -> Self {
        let mut reg = Handlebars::new();
        reg.register_template_string("base_url", base_url).unwrap();
        Self {
            api_key: "API_AUT".to_string(),
            base_url: reg,
        }
    }

    async fn write_pubic(&mut self, msg: String) -> Result<()> {
        let client = reqwest::Client::new();

        let url_par = self
            .base_url
            .render("base_url", &json!({"method": "write_public", "msg":&msg}))
            .unwrap();

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

        let url_par = self
            .base_url
            .render("base_url", &json!({"method": "write_masked", "msg":&msg}))
            .unwrap();

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

        let url_par = self
            .base_url
            .render("base_url", &json!({"method": "write_tagged", "msg":&msg}))
            .unwrap();

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
    let mut author = Author::new("http://0.0.0.0:8000/{{method}}/{{msg}}");

    let mut c = 0u32;
    loop {
        let msg = format!("test public msg nr. {}", &c);
        author.write_pubic(msg.to_string()).await.unwrap();
        thread::sleep(time::Duration::from_secs(20));

        let msg = format!("test masked msg nr. {}", &c);
        author.write_masked(msg.to_string()).await.unwrap();
        thread::sleep(time::Duration::from_secs(10));

        let msg = format!("test tagged msg nr. {}", &c);
        author.write_tagged(msg.to_string()).await.unwrap();
        thread::sleep(time::Duration::from_secs(10));
        c = &c + 1;
    }
}
