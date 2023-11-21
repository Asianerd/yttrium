use sqlx::{Pool, sqlite::Sqlite};

use rocket::State;

pub struct User {
    id: u32,
    name: String,
    password: String, // possible security problem? :)
}
impl User {
    pub fn new() -> User {
        User {
            id: 0,
            name: "".to_string(),
            password: "".to_string()
        }
    }
}

#[get("/<username>")]
pub async fn user_exists(username: String, pool: &State<Pool<Sqlite>>) -> String {
    println!("{username:?}");
    match sqlx::query!("SELECT username FROM user WHERE username = ?", username).fetch_one(&**pool).await {
        Err(_) => "false".to_string(),
        _ => "true".to_string()
    }
}

#[get("/<username>/<password>")]
pub async fn attempt_login(username: String, password: String, pool: &State<Pool<Sqlite>>) -> String {
    match sqlx::query!("SELECT username FROM user WHERE username = ?", username).fetch_one(&**pool).await {
        Err(_) => {
            return serde_json::to_string(&(false, "username doesnt exist")).unwrap();
        },
        _ => {}
    };

    match sqlx::query!("SELECT username FROM user WHERE username = ? AND password = ?", username, password).fetch_one(&**pool).await {
        Err(_) => {
            return serde_json::to_string(&(false, "password is incorrect")).unwrap();
        },
        _ => {
            return serde_json::to_string(&(true, "")).unwrap();
        }
    }
}

#[get("/<username>/<password>")]
pub async fn attempt_signup(username: String, password :String, pool: &State<Pool<Sqlite>>) -> String {
    match sqlx::query!("SELECT username FROM user WHERE username = ?", username).fetch_optional(&**pool).await.unwrap() {
        Some(_) => { return serde_json::to_string(&(false, "username taken")).unwrap(); }
        None => {
            let r = sqlx::query!("INSERT INTO user(username, password) VALUES (?, ?)", username, password).execute(&**pool).await.unwrap();
            serde_json::to_string(&(true, format!("{r:?}"))).unwrap()
        }
    }
}

// CREATE TABLE chat_data (id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, user varchar, chat_id int, last_interact int, seen bool, foreign key(user) references user(username), foreign key(chat_id) references chat(id))

// CREATE TABLE chat (id integer not null primary key autoincrement, name varchar)

// CREATE TABLE chat (id integer not null primary key autoincrement, name varchar)
// CREATE TABLE chat_data (id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, user_id int, chat_id int, last_interact int, seen bool, foreign key(user_id) references user(id), foreign key(chat_id) references chat(id))
// CREATE TABLE message (id integer not null primary key autoincrement, chat_id int, author int, content text, time int, foreign key(chat_id) references chat(id), foreign key(author) references user(id))
// CREATE TABLE user (id integer NOT NULL PRIMARY KEY AUTOINCREMENT, username varchar, password varchar)
