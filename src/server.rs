#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate channels_lite;

use rocket_contrib::{json::Json};
use channels_lite::channels::channel_author;

mod lib;
use self::lib::{response::Response};


#[get("/add_subscriber/<subscribe_tag>")]
fn index(subscribe_tag: String) -> Json<Response> {
    Author.add_subscriber(subscribe_tag);
    Json(Response {
        status: "OK",
        message: subscribe_tag,
    })
}

fn main() {

    let seed_subscriber = "SOME9SUBSCRIBER9SEED";

    let delay_time: u64 = 40;

    //Open Channel
    let (channel_address, announcement_tag) = Author.open().unwrap();
    println!("Author: Announced channel");
    println!("channel_address: {}", channel_address);
    println!("announcement_tag: {}", announcement_tag);


    rocket::ignite()
    .mount("/", routes![index])
    .launch();
}