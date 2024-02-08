// use std::{io::{BufRead, BufReader, Write}, net::{
//     TcpListener,
//     TcpStream
// }};
#[macro_use] extern crate rocket;

use rand::prelude::*;
use std::sync::Mutex;
use std::time::Instant;

mod cors;
mod chat;
mod message;
mod user;

mod web_handler;

fn _old_tcp() {
    // old tcp approach
    // let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // for stream in listener.incoming() {
    //     let mut stream = stream.unwrap();
    //     let i = BufReader::new(&mut stream);
    //     let h: Vec<_> = i
    //         .lines()
    //         .map(|result| result.unwrap())
    //         .take_while(|line| !line.is_empty())
    //         .collect();
    //     println!("{h:?}");
    //     let r = "HTTP/1.1 200 OK";
    //     let f = stream.write_all(r.as_bytes()).unwrap();
    //     println!("{f:?}");
    // }
}

#[get("/")]
fn index() -> String {
    "can you understand me?".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Mutex::new(user::UserBank::new()))

        .mount("/", routes![index])

        .mount("/sign_up", routes![web_handler::signup])
        .mount("/login", routes![web_handler::login])
        .mount("/debug", routes![web_handler::debug])

        .attach(cors::CORS)
}

