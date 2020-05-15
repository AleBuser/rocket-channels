#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate channels_lite;

use rocket_contrib::{json::Json};
use channels_lite::channels::channel_author::Channel;

mod lib;
use self::lib::{response::Response};


#[get("/add_subscriber/<subscribe_tag>")]
fn index(subscribe_tag: String) -> Json<Response> {
    let mut author: Channel = Channel::new("SOME9AUTHOR9SEED9SECRTE9A", "https://nodes.devnet.iota.org:443");
    author.add_subscriber(subscribe_tag.clone());
    Json(Response {
        status: "OK",
        message: subscribe_tag.clone(),
    })
}

fn main() {
   

    let delay_time: u64 = 40;

    //Open Channel
    let mut author: Channel = Channel::new("SOME9AUTHOR9SEED9SECRTE9A", "https://nodes.devnet.iota.org:443");
    let (channel_address, announcement_tag) = author.open().unwrap();
    println!("Author: Announced channel");
    println!("channel_address: {}", channel_address);
    println!("announcement_tag: {}", announcement_tag);


    rocket::ignite()
    .mount("/", routes![index])
    .launch();
}