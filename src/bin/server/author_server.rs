#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate handlebars;
use rocket::State;
extern crate channels_lite;
extern crate rocket_contrib;
extern crate serde_derive;

use std::sync::Mutex;

use channels_lite::channels::channel_author::Channel;
use channels_lite::channels::Network;
use channels_lite::utils::payload::json::PayloadBuilder;
use rocket_contrib::json::Json;

use local::types::tag::Tag;

use local::security::{
    api_key_author::ApiKeyAuthor, api_key_subscriber::ApiKeySubscriber, keystore::KeyManager,
    random_key,
};

use local::responses::{
    response_announce::ResponseAnnounce, response_list::ResponseList,
    response_message::ResponseMessage,
};

struct TagLists {
    signed_public: Mutex<Vec<String>>,
    signed_masked: Mutex<Vec<String>>,
    tagged: Mutex<Vec<String>>,
}

struct ChannelAdress {
    channel_address: Mutex<String>,
}

struct AnnouncementTag {
    announcement_tag: Mutex<String>,
}

#[put("/add_subscriber/<subscribe_tag>")]
fn add_subscriber(
    subscribe_tag: Tag,
    author: State<Mutex<Channel>>,
    _key: ApiKeySubscriber,
) -> Json<ResponseMessage> {
    let mut author = author.lock().expect("lock author");

    match author.add_subscriber(subscribe_tag.val.to_string()) {
        Ok(keyload) => {
            println!("added subscriber and generated keyload: {}", keyload);
            Json(ResponseMessage {
                status: "OK",
                message: keyload,
            })
        }
        Err(_e) => Json(ResponseMessage {
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
) -> Json<ResponseMessage> {
    let mut author = author.lock().expect("lock author");

    match author.remove_subscriber(unsubscribe_tag.val.to_string()) {
        Ok(_) => Json(ResponseMessage {
            status: "OK",
            message: unsubscribe_tag.val.to_string().clone(),
        }),
        Err(_e) => Json(ResponseMessage {
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
) -> Json<ResponseMessage> {
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
            Json(ResponseMessage {
                status: "OK",
                message: "Message sent to Tangle".to_string(),
            })
        }
        Err(_e) => Json(ResponseMessage {
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
) -> Json<ResponseMessage> {
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
            Json(ResponseMessage {
                status: "OK",
                message: "Message sent to Tangle".to_string(),
            })
        }
        Err(_e) => Json(ResponseMessage {
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
) -> Json<ResponseMessage> {
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
            Json(ResponseMessage {
                status: "OK",
                message: "Message sent to Tangle".to_string(),
            })
        }
        Err(_e) => Json(ResponseMessage {
            status: "Error",
            message: "Failed to write message".to_string(),
        }),
    }
}

#[get("/get_tagged_list")]
fn get_tagged_list(list: State<TagLists>, _key: ApiKeySubscriber) -> Json<ResponseList> {
    let tagged_list = list.tagged.lock().expect("lock list data").clone();
    Json(ResponseList {
        status: "OK",
        list: tagged_list,
    })
}

#[get("/get_public_list")]
fn get_public_list(list: State<TagLists>, _key: ApiKeySubscriber) -> Json<ResponseList> {
    let signed_public_list = list.signed_public.lock().expect("lock list data").clone();
    Json(ResponseList {
        status: "OK",
        list: signed_public_list,
    })
}

#[get("/get_masked_list")]
fn get_masked_list(list: State<TagLists>, _key: ApiKeySubscriber) -> Json<ResponseList> {
    let signed_masked_list = list.signed_masked.lock().expect("lock list data").clone();
    Json(ResponseList {
        status: "OK",
        list: signed_masked_list,
    })
}

#[get("/new_subscriber_key")]
fn new_subscriber_key(
    keystore: State<Mutex<KeyManager>>,
    _key: ApiKeyAuthor,
) -> Json<ResponseMessage> {
    let key = random_key::new();
    let mut keystore = keystore.lock().expect("lock keystore data");
    keystore.add_subscriber(key.to_string());
    Json(ResponseMessage {
        status: "OK",
        message: key.to_string(),
    })
}

#[get("/get_announcement")]
fn get_announcement(
    channel_address: State<ChannelAdress>,
    announcement_tag: State<AnnouncementTag>,
    _key: ApiKeySubscriber,
) -> Json<ResponseAnnounce> {
    Json(ResponseAnnounce {
        status: "OK",
        channel_address: channel_address
            .channel_address
            .lock()
            .expect("lock list data")
            .clone(),
        announcement_tag: announcement_tag
            .announcement_tag
            .lock()
            .expect("lock list data")
            .clone(),
    })
}

fn main() {
    //Open Channel
    let author: Mutex<Channel> = Mutex::new(Channel::new(Network::Devnet, None));
    let (x, y) = author.lock().expect("").open().unwrap();
    println!("Author: Announced channel");
    println!("channel_address: {}", x);
    println!("announcement_tag: {}", y);

    let channel_address = ChannelAdress {
        channel_address: Mutex::new(x),
    };

    let announcement_tag = AnnouncementTag {
        announcement_tag: Mutex::new(y),
    };

    //let mut keystore = KeyManager::new(calculate_hash("API_AUT".to_string()), vec![]);
    let key_manager = Mutex::new(KeyManager::restore());

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
                get_masked_list,
                get_announcement,
                new_subscriber_key,
            ],
        )
        .manage(author)
        .manage(key_manager)
        .manage(tagstore)
        .manage(channel_address)
        .manage(announcement_tag)
        .launch();
}
