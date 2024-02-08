use std::sync::Mutex;
use crate::user;
use rocket::State;

#[get("/<username>/<password>")]
pub fn signup(username: String, password: String, data_holder: &State<Mutex<user::UserBank>>) -> String {
    let mut i = data_holder.lock().unwrap();
    format!("{:?}", i.signup(&username, &password))
}

#[get("/")]
pub fn debug(data_holder: &State<Mutex<user::UserBank>>) -> String {
    format!("{:?}", data_holder.lock().unwrap())
}

#[get("/<username>/<password>")]
pub fn login(username: String, password: String, data_holder: &State<Mutex<user::UserBank>>) -> String {
    let i = data_holder.lock().unwrap();
    format!("{:?}", i.login(&username, &password))
}
