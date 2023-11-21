use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: u128,
    pub chat_id: u64,
    pub content: String,
    pub time: u64
}
impl Message {
    pub fn new() -> Message {
        Message {
            chat_id: 0,
            id: 0,
            content: "".to_string(),
            time: 0
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}
