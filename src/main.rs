use sqlx::sqlite::SqlitePool;
use sqlx::{Pool, sqlite::Sqlite};

// use tokio;
#[macro_use] extern crate rocket;
use rocket::{State, http::Header, Request, Response, fairing::{Fairing, Info, Kind}};

extern crate dotenvy;
use dotenvy::dotenv;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
// use std::time::Instant;
// use rand::prelude::*;

mod db;

mod user;
mod chat;
mod message;

use message::Message;

// region: CORS
pub struct CORS;
#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "cors headers",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
// endregion

// region: DataHolder
#[derive(Debug, Clone)]
struct DataHolder {
    // pool: Pool<Sqlite>
}
impl DataHolder {
    async fn new() -> DataHolder {
        DataHolder {
            //pool: SqlitePool::connect(dotenvy::var("DATABASE_URL").unwrap().as_str()).await.unwrap()
        }
    }
}
// endregion

// region: db
//#[tokio::main]
// async fn start_db() -> Pool<Sqlite> {
//     dotenv().unwrap();

//     let pool = SqlitePool::connect(dotenvy::var("DATABASE_URL").unwrap().as_str()).await.unwrap();

//     //let mut connection = pool.acquire().await.unwrap();

//     //println!("{connection:?}");

//     println!("{:?}", sqlx::query!(r#"INSERT INTO table1 VALUES ('test', 10);"#).execute(&pool).await.unwrap());

//     let recs = sqlx::query!(r#"SELECT * FROM table1"#)
//         .fetch_all(&pool)
//         .await.unwrap();



//     println!("{recs:?}");

//     pool
// }
// endregion

// region: placeholder
#[get("/")]
fn index() -> String {
    "".to_string()
}

#[get("/")]
async fn fetch_all(pool: &State<Pool<Sqlite>>) -> String {
    format!("{:?}", sqlx::query!("SELECT * FROM message").fetch_all(&**pool).await.unwrap())
}
// endregion

// region: message
#[get("/<id>")]
async fn fetch_message(id: i32, pool: &State<Pool<Sqlite>>) -> String {
    let r = sqlx::query!("SELECT * FROM message WHERE id = ?", id).fetch_one(&**pool).await.unwrap();
    let m = Message {
        id: r.id as u128,
        chat_id: r.chat_id.unwrap() as u64,
        content: r.content.unwrap(),
        time: r.time.unwrap() as u64
    };
    m.to_string()
}

#[get("/<chat_id>/<content>")]
async fn create_message(chat_id: i32, content: String, pool: &State<Pool<Sqlite>>) -> String {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
    sqlx::query!(r#"INSERT INTO message(chat_id, content, time) VALUES (?, ?, ?)"#, chat_id, content, time)
        .execute(&**pool)
        .await
        .unwrap()
        .last_insert_rowid()
        .to_string()
}
// endregion

// region: testing
// #[tokio::main]
// async fn main() { 
//     dotenv().unwrap();

//     let pool = SqlitePool::connect(dotenvy::var("DATABASE_URL").unwrap().as_str()).await.unwrap();
//     let user_id = 1;

//     let chat_query = sqlx::query!("SELECT * FROM chat_data WHERE user_id = ?", user_id).fetch_all(&pool).await.unwrap();
//     //println!("{chat_query:?}");
//     let mut display_data: HashMap<i64, (String, String)> = HashMap::new();
//     for chat in chat_query {
//         let c_id = chat.chat_id.unwrap();
//         let title = sqlx::query!("SELECT * FROM chat WHERE id = ?", c_id).fetch_one(&pool).await.unwrap().name.unwrap();

//         //println!("{:?}", sqlx::query!("SELECT * FROM message WHERE chat_id = ? ORDER BY time DESC LIMIT 1", c_id).fetch_one(&pool).await.unwrap());
//         // some chats dont have messages at all
//         let first_message_result = sqlx::query!("SELECT * FROM message WHERE chat_id = ? ORDER BY time DESC LIMIT 1", c_id).fetch_optional(&pool).await.unwrap();
//         //println!("Result : {:?}", first_message_result);
//         let first_message = match first_message_result {
//             Some(n) => n.content.unwrap(),
//             None => "".to_string()
//         };
//         display_data.insert(c_id, (title, first_message));
//     }

//     format!("{display_data:?}")
// }
// endregion

#[launch]
async fn rocket() -> _ {
//async fn rocket() {
    dotenv().unwrap();

    rocket::build()
        .manage(SqlitePool::connect(dotenvy::var("DATABASE_URL").unwrap().as_str()).await.unwrap())
        .mount("/", routes![index])
        .mount("/fetch_all", routes![fetch_all])
        .mount("/fetch_message", routes![fetch_message])
        .mount("/create_message", routes![create_message])

        .mount("/user/user_exists", routes![user::user_exists])
        .mount("/user/attempt_login", routes![user::attempt_login])
        .mount("/user/attempt_signup", routes![user::attempt_signup])

        .mount("/chat/sidebar", routes![chat::sidebar])

        .attach(CORS)
}
