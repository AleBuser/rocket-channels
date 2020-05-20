/*
use futures::{Future};
use hyper::{Body, Client, Request, Response};
use tokio_core::reactor::Core;
use serde::de;
*/

use reqwest;
use reqwest::Url;

use serde_json::{Result, Value};

use channels_lite::channels::channel_subscriber::Channel;

pub struct Subscriber {
    api_key:String,
    channel_subscriber: Channel,
}

impl Subscriber {

    pub fn new() -> Subscriber {

        let subscriber: Channel = Channel::new(
            "SOME9SUBSCRIBER9SEED",
            "https://nodes.devnet.iota.org:443",
            "QSUIACTIMIPJYBJ9GYAXKZGVSUGHUBFKPWKIPUKMSNKRPQZCGTUGJVGWZCKXBKRTCMJQQXZE9UYWDCTKF".to_string(),
            "NFAIQDKVSWUAXIYMJXJPRFN9ZDD".to_string()
        );
        Self {
            api_key: "API_SUB".to_string(),
            channel_subscriber: subscriber,
        }
    }


    async fn get_tagged_list(&mut self) -> Result<Vec<String>>  {

        let client = reqwest::Client::new();

        let body = &client.get("http://0.0.0.0:8000/get_tagged_list")
        .header("x-api-key", self.api_key.clone())
        .send()
        .await.unwrap()
        .text()
        .await.unwrap().clone();

        let ret: Value  = serde_json::from_str(body).unwrap();

        let mut tagged_list: Vec<String> = vec![];  
        
        let list = ret["list"].as_array().unwrap().clone();
        for t in &list{
            tagged_list.push(t.as_str().unwrap().to_string());  
        }
        
        Ok(tagged_list)

    }

    async fn get_public_list(&mut self) -> Result<Vec<String>>  {

        let client = reqwest::Client::new();

        let body = &client.get("http://0.0.0.0:8000/get_public_list")
        .header("x-api-key", self.api_key.clone())
        .send()
        .await.unwrap()
        .text()
        .await.unwrap().clone();

        let ret: Value  = serde_json::from_str(body).unwrap();

        let mut public_list: Vec<String> = vec![];  
        
        let list = ret["list"].as_array().unwrap().clone();
        for t in &list{
            public_list.push(t.as_str().unwrap().to_string());  
        }
        
        Ok(public_list)
    }

    async fn get_masked_list(&mut self) -> Result<Vec<String>>  {

        let client = reqwest::Client::new();

        let body = &client.get("http://0.0.0.0:8000/get_masked_list")
        .header("x-api-key", self.api_key.clone())
        .send()
        .await.unwrap()
        .text()
        .await.unwrap().clone();

        let ret: Value  = serde_json::from_str(body).unwrap();

        let mut masked_list: Vec<String> = vec![];  
        
        let list = ret["list"].as_array().unwrap().clone();
        for t in &list{
            masked_list.push(t.as_str().unwrap().to_string());  
        }
        
        Ok(masked_list)
    }

    async fn connect(&mut self, tag: String) -> Result<String>  {

        let client = reqwest::Client::new();

        let url_par = "http://0.0.0.0:8000/add_subscriber/".to_owned() + &tag;

        let body = &client.put( Url::parse(&url_par).unwrap())
        .header("x-api-key", self.api_key.clone())
        .send()
        .await.unwrap()
        .text()
        .await.unwrap().clone();

        println!("connection: OK");

        let ret: Value  = serde_json::from_str(body).unwrap();

        let tag = ret["message"].as_str().unwrap().to_string();
        
        println!("{:?}",&tag);

        self.channel_subscriber.update_keyload(tag.clone()).unwrap();
        Ok(tag)

    }

    async fn read_tagged(&mut self) -> Result<Vec<String>>  {

        let tag_list: Vec<String> = self.get_tagged_list().await.unwrap();

        let mut msg_list: Vec<String> = vec![];

        for tag in tag_list{
            let msgs = self.channel_subscriber.read_tagged(tag).unwrap();
            for (msg_p,_msg_m) in msgs{
                msg_list.push(msg_p);
            }
        }
        
        println!("messages: {:?}",&msg_list);
        Ok(msg_list)
    }

    async fn read_public(&mut self) -> Result<Vec<String>>  {

        let tag_list: Vec<String> = self.get_public_list().await.unwrap();

        let mut msg_list: Vec<String> = vec![];

        for tag in tag_list{
            let msgs = self.channel_subscriber.read_signed(tag).unwrap();
            for (msg_p,_msg_m) in msgs{
                msg_list.push(msg_p);
            }
        }
        
        println!("messages: {:?}",&msg_list);
        Ok(msg_list)
    }

    async fn read_masked(&mut self) -> Result<Vec<String>>  {

        let tag_list: Vec<String> = self.get_masked_list().await.unwrap();

        let mut msg_list: Vec<String> = vec![];

        for tag in tag_list{
            let msgs = self.channel_subscriber.read_signed(tag).unwrap();
            for (msg_p,_msg_m) in msgs{
                msg_list.push(msg_p);
            }
        }
        
        println!("messages: {:?}",&msg_list);
        Ok(msg_list)
    }
}
#[tokio::main]
async fn main() {

    let mut sub = Subscriber::new();

    let subscription_tag: String = sub.channel_subscriber.connect().unwrap();

    sub.connect(subscription_tag).await.unwrap();

    sub.read_public().await.unwrap();
    sub.read_masked().await.unwrap();
    sub.read_tagged().await.unwrap();
}
