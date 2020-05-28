/*
use futures::{Future};
use hyper::{Body, Client, Request, Response};
use tokio_core::reactor::Core;
use serde::de;
*/

use reqwest;
use reqwest::Url;

use std::{thread, time};

use serde_json::{Result, Value};

use channels_lite::channels::channel_subscriber::Channel;
use channels_lite::utils::response_write_signed::ResponseSigned;

pub struct Subscriber {
    api_key: String,
    channel_subscriber: Channel,
}

impl Subscriber {
    pub fn new() -> Self {
        let subscriber: Channel = Channel::new(
            "https://nodes.devnet.iota.org:443",
            "ZOPJKISBGSGRJTTXDQLCTVVMDRNGOQGSZXSAXYUOYT9YUNG9IIMKZISNYTKGLSSBTIOS9MRVZAONRNZDB"
                .to_string(),
            "N9A9FQZLRMMIJMDDAMQGXCPOUBE".to_string(),
            None,
        );
        Self {
            api_key: "API_SUB".to_string(),
            channel_subscriber: subscriber,
        }
    }

    async fn get_tagged_list(&mut self) -> Result<Vec<String>> {
        let client = reqwest::Client::new();

        let body = &client
            .get("http://0.0.0.0:8000/get_tagged_list")
            .header("x-api-key", self.api_key.clone())
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
            .clone();

        let ret: Value = serde_json::from_str(body).unwrap();

        let mut tagged_list: Vec<String> = vec![];
        let list = ret["list"].as_array().unwrap().clone();
        for t in &list {
            tagged_list.push(t.as_str().unwrap().to_string());
        }

        Ok(tagged_list)
    }

    async fn get_public_list(&mut self) -> Result<Vec<(String, Option<String>)>> {
        let client = reqwest::Client::new();

        let body = &client
            .get("http://0.0.0.0:8000/get_public_list")
            .header("x-api-key", self.api_key.clone())
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
            .clone();

        let ret: Value = serde_json::from_str(body).unwrap();

        let mut public_list: Vec<(String, Option<String>)> = vec![];
        let list = ret["list"].as_array().unwrap().clone();
        for t in &list {
            let t: ResponseSigned = serde_json::from_str(t.as_str().unwrap()).unwrap();
            let signed_message_tag = t.signed_message_tag;
            let change_key_tag = t.change_key_tag;
            public_list.push((signed_message_tag, change_key_tag));
        }
        Ok(public_list)
    }

    async fn get_masked_list(&mut self) -> Result<Vec<(String, Option<String>)>> {
        let client = reqwest::Client::new();

        let body = &client
            .get("http://0.0.0.0:8000/get_masked_list")
            .header("x-api-key", self.api_key.clone())
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
            .clone();

        let ret: Value = serde_json::from_str(body).unwrap();

        let mut masked_list: Vec<(String, Option<String>)> = vec![];
        let list = ret["list"].as_array().unwrap().clone();
        for t in &list {
            let t: ResponseSigned = serde_json::from_str(t.as_str().unwrap()).unwrap();
            let signed_message_tag = t.signed_message_tag;
            let change_key_tag = t.change_key_tag;
            masked_list.push((signed_message_tag, change_key_tag));
        }
        Ok(masked_list)
    }

    async fn share_subscription(&mut self, tag: String) -> Result<String> {
        let client = reqwest::Client::new();

        let url_par = "http://0.0.0.0:8000/add_subscriber/".to_owned() + &tag;

        let body = &client
            .put(Url::parse(&url_par).unwrap())
            .header("x-api-key", self.api_key.clone())
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
            .clone();

        println!("connection: OK");

        let ret: Value = serde_json::from_str(body).unwrap();

        let tag = ret["message"].as_str().unwrap().to_string();
        self.channel_subscriber.update_keyload(tag.clone()).unwrap();

        println!("Updated keyload to {:?}", &tag);

        Ok(tag)
    }

    async fn read_tagged(&mut self) -> Result<Vec<String>> {
        let tag_list: Vec<String> = self.get_tagged_list().await.unwrap();

        let mut msg_list: Vec<String> = vec![];

        for tag in tag_list {
            let msgs: Vec<(Option<String>, Option<String>)> =
                self.channel_subscriber.read_tagged(tag).unwrap();
            for (msg_p, _msg_m) in msgs {
                match msg_p {
                    None => continue,
                    Some(message) => msg_list.push(message),
                }
            }
        }

        println!("messages tagged : {:?}", &msg_list);
        Ok(msg_list)
    }

    async fn read_all_public(&mut self) -> Result<Vec<String>> {
        let tag_list: Vec<(String, Option<String>)> = self.get_public_list().await.unwrap();

        let mut msg_list: Vec<String> = vec![];

        for (signed_message_tag, change_key_tag) in tag_list {
            let msgs: Vec<(Option<String>, Option<String>)> = self
                .channel_subscriber
                .read_signed(signed_message_tag, change_key_tag)
                .unwrap();
            for (msg_p, _msg_m) in msgs {
                match msg_p {
                    None => continue,
                    Some(message) => msg_list.push(message),
                }
            }
        }

        println!("messages public: {:?}", &msg_list);
        Ok(msg_list)
    }

    async fn read_all_masked(&mut self) -> Result<Vec<String>> {
        let tag_list: Vec<(String, Option<String>)> = self.get_masked_list().await.unwrap();

        let mut msg_list: Vec<String> = vec![];

        for (signed_message_tag, change_key_tag) in tag_list {
            let msgs = self
                .channel_subscriber
                .read_signed(signed_message_tag, change_key_tag)
                .unwrap();
            for (_msg_p, msg_m) in msgs {
                match msg_m {
                    Some(message) => msg_list.push(message),
                    None => continue,
                }
            }
        }

        println!("messages masked: {:?}", &msg_list);
        Ok(msg_list)
    }
}
#[tokio::main]
async fn main() {
    let mut sub = Subscriber::new();

    let subscription_tag: String = sub.channel_subscriber.connect().unwrap();

    thread::sleep(time::Duration::from_secs(10 * 2));

    sub.share_subscription(subscription_tag).await.unwrap();

    loop {
        //give author time to publish some msg
        thread::sleep(time::Duration::from_secs(5));

        sub.read_all_public().await.unwrap();
        sub.read_all_masked().await.unwrap();
    }
}
