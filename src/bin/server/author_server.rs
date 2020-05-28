#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::State;
extern crate channels_lite;

use std::sync::Mutex;

use channels_lite::channels::channel_author::Channel;
use channels_lite::utils::payload::json::PayloadBuilder;
use rocket_contrib::json::Json;

use local::types::{
    api_key_author::ApiKeyAuthor, api_key_subscriber::ApiKeySubscriber, response::Response,
    returned_list::ReturnedList, tag::Tag,
};

use local::security::keystore::Keystore;

struct TagLists {
    signed_public: Mutex<Vec<String>>,
    signed_masked: Mutex<Vec<String>>,
    tagged: Mutex<Vec<String>>,
}

#[put("/add_subscriber/<subscribe_tag>")]
fn add_subscriber(
    subscribe_tag: Tag,
    author: State<Mutex<Channel>>,
    _key: ApiKeySubscriber,
) -> Json<Response> {
    let mut author = author.lock().expect("lock author");

    match author.add_subscriber(subscribe_tag.val.to_string()) {
        Ok(keyload) => {
            println!("added subscriber and generated keyload: {}", keyload);
            Json(Response {
                status: "OK",
                message: keyload,
            })
        }
        Err(_e) => Json(Response {
            status: "Error",
            message: "Not a valid Tag".to_string(),
        }),
    }
}

#[delete("/remove_subscriber/<unsubscribe_tag>")]
fn remove_subscriber(
    unsubscribe_tag: Tag,
    author: State<Mutex<Channel>>,
    _key: ApiKeySubscriber,
) -> Json<Response> {
    let mut author = author.lock().expect("lock author");

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

#[post("/write_public/<public_message>")]
fn write_public(
    public_message: String,
    author: State<Mutex<Channel>>,
    list: State<TagLists>,
    _key: ApiKeyAuthor,
) -> Json<Response> {
    let mut author = author.lock().expect("lock author");

    match author.write_signed(
        false,
        PayloadBuilder::new()
            .public(&public_message)
            .unwrap()
            .build(),
    ) {
        Ok(public_message_tag) => {
            let response = serde_json::to_string(&public_message_tag).unwrap();
            list.signed_public
                .lock()
                .expect("lock list data")
                .push(response.clone());
            println!("sent public message with tag: {}", response);
            Json(Response {
                status: "OK",
                message: "Message sent to Tangle".to_string(),
            })
        }
        Err(_e) => Json(Response {
            status: "Error",
            message: "Not a valid Tag".to_string(),
        }),
    }
}

#[post("/write_masked/<masked_message>")]
fn write_masked(
    masked_message: String,
    author: State<Mutex<Channel>>,
    list: State<TagLists>,
    _key: ApiKeyAuthor,
) -> Json<Response> {
    let mut author = author.lock().expect("lock author");

    match author.write_signed(
        true,
        PayloadBuilder::new()
            .masked(&masked_message)
            .unwrap()
            .build(),
    ) {
        Ok(masked_message_tag) => {
            let response = serde_json::to_string(&masked_message_tag).unwrap();
            list.signed_masked
                .lock()
                .expect("lock list data")
                .push(response.clone());
            println!("sent masked message with tag: {}", response);
            Json(Response {
                status: "OK",
                message: "Message sent to Tangle".to_string(),
            })
        }
        Err(_e) => Json(Response {
            status: "Error",
            message: "Not a valid Tag".to_string(),
        }),
    }
}

#[post("/write_tagged/<tagged_message>")]
fn write_tagged(
    tagged_message: String,
    author: State<Mutex<Channel>>,
    list: State<TagLists>,
    _key: ApiKeyAuthor,
) -> Json<Response> {
    let mut author = author.lock().expect("lock author");

    match author.write_tagged(
        PayloadBuilder::new()
            .public(&tagged_message)
            .unwrap()
            .build(),
    ) {
        Ok(tagget_packet_tag) => {
            list.tagged
                .lock()
                .expect("lock list data")
                .push(tagget_packet_tag.clone());
            println!("sent tagged message with tag: {}", tagget_packet_tag);
            Json(Response {
                status: "OK",
                message: "Message sent to Tangle".to_string(),
            })
        }
        Err(_e) => Json(Response {
            status: "Error",
            message: "Not a valid Tag".to_string(),
        }),
    }
}

#[get("/get_tagged_list")]
fn get_tagged_list(list: State<TagLists>, _key: ApiKeySubscriber) -> Json<ReturnedList> {
    let tagged_list = list.tagged.lock().expect("lock list data").clone();
    Json(ReturnedList {
        status: "OK",
        list: tagged_list,
    })
}

#[get("/get_public_list")]
fn get_public_list(list: State<TagLists>, _key: ApiKeySubscriber) -> Json<ReturnedList> {
    let signed_public_list = list.signed_public.lock().expect("lock list data").clone();
    Json(ReturnedList {
        status: "OK",
        list: signed_public_list,
    })
}

#[get("/get_masked_list")]
fn get_masked_list(list: State<TagLists>, _key: ApiKeySubscriber) -> Json<ReturnedList> {
    let signed_masked_list = list.signed_masked.lock().expect("lock list data").clone();
    Json(ReturnedList {
        status: "OK",
        list: signed_masked_list,
    })
}

fn main() {
    //Open Channel
    let author: Mutex<Channel> = Mutex::new(Channel::new(
        "https://nodes.devnet.iota.org:443",
        Some("EVERVERVERVERAWQQQ".to_string()),
    ));
    let (channel_address, announcement_tag) = author.lock().expect("").open().unwrap();
    println!("Author: Announced channel");
    println!("channel_address: {}", channel_address);
    println!("announcement_tag: {}", announcement_tag);

    let keystore = Keystore::new("API_SUB".to_string(), "API_AUT".to_string());

    let tagstore = TagLists {
        signed_public: Mutex::new(vec![]),
        signed_masked: Mutex::new(vec![]),
        tagged: Mutex::new(vec![]),
    };

    rocket::ignite()
        .mount(
            "/",
            routes![
                add_subscriber,
                remove_subscriber,
                write_public,
                write_masked,
                write_tagged,
                get_tagged_list,
                get_public_list,
                get_masked_list
            ],
        )
        .manage(author)
        .manage(keystore)
        .manage(tagstore)
        .launch();
}