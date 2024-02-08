// use rand::prelude::*;

#[derive(Clone, Debug)]
pub struct User {
    pub user_id: u64,
    pub username: String,

    password: String, // private as security measure
}
impl User {
    fn verify_password(&self, password: &String) -> bool {
        password.clone() == self.password
    }
}

#[derive(Debug)]
pub struct UserBank {
    // pub rng : rand::prelude::ThreadRng,

    pub collection : Vec<User>
}
impl UserBank {
    pub fn new() -> UserBank {
        UserBank {
            collection: vec![],
            // rng: rand::thread_rng()
        }
    }

    fn register_user(&mut self, username: &String, password: &String) -> u64 {
        self.collection.push(User {
            user_id: self.collection.len() as u64,
            username: username.clone(),
            password: password.clone()
        });

        self.collection.len() as u64
    }

    pub fn signup(&mut self, username: &String, password: &String) -> (Option<Error>, u64) {
        if self.username_exists(username) {
            return (Some(Error::UsernameExists), 0)
        }

        (None, self.register_user(username, password))
    }

    fn user_id_exists(&self, user_id: &u64) -> bool {
        self.collection.clone().iter().map(|x| x.user_id).collect::<Vec<u64>>().contains(user_id)
    }

    fn username_exists(&self, username: &String) -> bool {
        self.collection.clone().iter().map(|x| x.username.clone()).collect::<Vec<String>>().contains(username)
    }

    pub fn login(&self, username: &String, password: &String) -> (Option<Error>, u64) {
        if !self.username_exists(username) {
            return (Some(Error::UsernameDoesntExist), 0);
        }
        for i in &self.collection {
            if i.verify_password(&password) {
                return (None, i.user_id);
            }
        }
        (Some(Error::WrongPassword), 0)
    }
}

#[derive(Debug)]
pub enum Error {
    UsernameExists,
    UsernameDoesntExist,

    WrongPassword
}
