#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate channels_lite;

use rocket_contrib::{json::Json};
use channels_lite::channels::channel_author::Channel;

use local::types::response::Response;


#[get("/add_subscriber/<subscribe_tag>")]
fn add_subscriber(subscribe_tag: String) -> Json<Response> {
    let mut author: Channel = Channel::new("SOME9AUTHOR9SEED9SECRTE9A", "https://nodes.devnet.iota.org:443");
    author.add_subscriber(subscribe_tag.clone());
    Json(Response {
        status: "OK",
        message: subscribe_tag.clone(),
    })
}

#[get("/remove_subscriber/<unsubscribe_tag>")]
fn remove_subscriber(unsubscribe_tag: String) -> Json<Response> {
    let mut author: Channel = Channel::new("SOME9AUTHOR9SEED9SECRTE9A", "https://nodes.devnet.iota.org:443");
    author.remove_subscriber(unsubscribe_tag.clone());
    Json(Response {
        status: "OK",
        message: unsubscribe_tag.clone(),
    })
}

#[get("/write_public/<auth_key>/<public_message>")]
fn write_public(auth_key:String, public_message: String) -> Json<Response> {
    let mut author: Channel = Channel::new("SOME9AUTHOR9SEED9SECRTE9A", "https://nodes.devnet.iota.org:443");

    author.write_signed(false, &public_message.clone(), "");
    Json(Response {
        status: "OK",
        message: public_message.clone(),
    })
}

#[get("/write_masked/<auth_key>/<masked_message>")]
fn write_masked(auth_key:String, masked_message: String) -> Json<Response> {
    let mut author: Channel = Channel::new("SOME9AUTHOR9SEED9SECRTE9A", "https://nodes.devnet.iota.org:443");
    author.write_signed(true, "", &masked_message.clone());
    Json(Response {
        status: "OK",
        message: masked_message.clone(),
    })
}

#[get("/write_tagged/<auth_key>/<tagged_message>")]
fn write_tagged(auth_key:String, tagged_message: String) -> Json<Response> {
    let mut author: Channel = Channel::new("SOME9AUTHOR9SEED9SECRTE9A", "https://nodes.devnet.iota.org:443");
    author.write_tagged("", &tagged_message.clone());
    Json(Response {
        status: "OK",
        message: tagged_message.clone(),
    })
}


fn main() {

    //Open Channel
    let mut author: Channel = Channel::new("SOME9AUTHOR9SEED9SECRTE9A", "https://nodes.devnet.iota.org:443");
    let (channel_address, announcement_tag) = author.open().unwrap();
    println!("Author: Announced channel");
    println!("channel_address: {}", channel_address);
    println!("announcement_tag: {}", announcement_tag);


    rocket::ignite()
    .mount("/", routes![add_subscriber, remove_subscriber, write_public, write_masked, write_tagged])
    .launch();
}