use std::collections::HashMap;

pub struct Message {
    pub content: String,
    pub author: u64
}

// implement data caching later
pub struct CacheParent {
    pub usernames: HashMap<u64, String>
}

pub struct CachePackage {
    pub username: String,
}
