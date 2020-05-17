#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate channels_lite;

use channels_lite::channels::channel_author::Channel;
use rocket_contrib::json::Json;

use local::types::{
    api_key_author::ApiKeyAuthor, api_key_subscriber::ApiKeySubscriber, response::Response,
    tag::Tag,
};

use local::security::keystore::Keystore;

#[get("/add_subscriber/<subscribe_tag>")]
fn add_subscriber(subscribe_tag: Tag, _key: ApiKeySubscriber) -> Json<Response> {
    let mut author: Channel = Channel::new(
        "SOME9AUTHOR9SEED9SECRTE9A",
        "https://nodes.devnet.iota.org:443",
    );
    match author.add_subscriber(subscribe_tag.val.to_string()) {
        Ok(_) => Json(Response {
            status: "OK",
            message: subscribe_tag.val.to_string().clone(),
        }),
        Err(_e) => Json(Response {
            status: "Error",
            message: "Not a valid Tag".to_string(),
        }),
    }
}

#[get("/remove_subscriber/<unsubscribe_tag>")]
fn remove_subscriber(unsubscribe_tag: Tag, _key: ApiKeySubscriber) -> Json<Response> {
    let mut author: Channel = Channel::new(
        "SOME9AUTHOR9SEED9SECRTE9A",
        "https://nodes.devnet.iota.org:443",
    );
    match author.remove_subscriber(unsubscribe_tag.val.to_string()) {
        Ok(_) => Json(Response {
            status: "OK",
            message: unsubscribe_tag.val.to_string().clone(),
        }),
        Err(_e) => Json(Response {
            status: "Error",
            message: "Not a valid Tag".to_string(),
        }),
    }
}

#[get("/write_public/<public_message>")]
fn write_public(public_message: Tag, _key: ApiKeyAuthor) -> Json<Response> {
    let mut author: Channel = Channel::new(
        "SOME9AUTHOR9SEED9SECRTE9A",
        "https://nodes.devnet.iota.org:443",
    );

    match author.write_signed(false, &public_message.val, "") {
        Ok(_) => Json(Response {
            status: "OK",
            message: public_message.val.to_string().clone(),
        }),
        Err(_e) => Json(Response {
            status: "Error",
            message: "Not a valid Tag".to_string(),
        }),
    }
}

#[get("/write_masked/<masked_message>")]
fn write_masked(masked_message: Tag, _key: ApiKeyAuthor) -> Json<Response> {
    let mut author: Channel = Channel::new(
        "SOME9AUTHOR9SEED9SECRTE9A",
        "https://nodes.devnet.iota.org:443",
    );
    match author.write_signed(true, "", &masked_message.val) {
        Ok(_) => Json(Response {
            status: "OK",
            message: masked_message.val.to_string().clone(),
        }),
        Err(_e) => Json(Response {
            status: "Error",
            message: "Not a valid Tag".to_string(),
        }),
    }
}

#[get("/write_tagged/<tagged_message>")]
fn write_tagged(tagged_message: Tag, _key: ApiKeyAuthor) -> Json<Response> {
    let mut author: Channel = Channel::new(
        "SOME9AUTHOR9SEED9SECRTE9A",
        "https://nodes.devnet.iota.org:443",
    );
    match author.write_tagged("", &tagged_message.val) {
        Ok(_) => Json(Response {
            status: "OK",
            message: tagged_message.val.to_string().clone(),
        }),
        Err(_e) => Json(Response {
            status: "Error",
            message: "Not a valid Tag".to_string(),
        }),
    }
}

fn main() {
    //Open Channel
    let mut author: Channel = Channel::new(
        "SOME9AUTHOR9SEED9SECRTE9A",
        "https://nodes.devnet.iota.org:443",
    );
    let (channel_address, announcement_tag) = author.open().unwrap();
    println!("Author: Announced channel");
    println!("channel_address: {}", channel_address);
    println!("announcement_tag: {}", announcement_tag);

    let keystore = Keystore::new("API_SUB".to_string(), "API_AUT".to_string());

    rocket::ignite()
        .mount(
            "/",
            routes![
                add_subscriber,
                remove_subscriber,
                write_public,
                write_masked,
                write_tagged
            ],
        )
        .manage(keystore)
        .launch();
}
