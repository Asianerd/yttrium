struct Chat {
    pub id: u64,
    pub name: String
}
impl Chat {
    pub fn new() -> Chat {
        Chat {
            id: 0,
            name: "".to_string()
        }
    }
}

struct ChatData {
    pub id: u64,
    pub user_id: u64,
    pub chat_id: u64,
    pub last_interact: u64,
    pub seen: bool
}
impl ChatData {
    pub fn new() -> ChatData {
        ChatData {
            id: 0,
            user_id: 0,
            chat_id: 0,
            last_interact: 0,
            seen: true
        }
    }
}
