use std::{collections::HashMap, time::{SystemTime, UNIX_EPOCH}};

use sqlx::{Pool, sqlite::Sqlite};

use rocket::State;

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

    pub fn date_from_epoch(t: i64) -> String {
        // let time = SystemTime::now()
        //     .duration_since(UNIX_EPOCH)
        //     .unwrap()
        //     .as_secs() as i64;
        
        let diff = (SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64) - t;

        // h 3600
        // d 86400
        // wk 604800

        // if diff > 604800 {
        //     return format!("{}wk", (diff / 604800) as i32);
        // }

        // if diff > 86400 {
        //     return format!("{}d", (diff / 86400) as i32);
        // }

        for (value, name) in [(604800, "wk"), (86400, "d"), (3600, "h"), (60, "m"), (1, "s")] {
            if diff > value {
                return format!("{}{name}", (diff / value) as i32);
            }
        }

        format!("{}s", diff as i32)
    }
}

#[get("/<username>")]
pub async fn sidebar(username: String, pool: &State<Pool<Sqlite>>) -> String {
    // list of all chats + first messsage in them
    let chat_query = sqlx::query!("SELECT * FROM chat_data WHERE user = ?", username).fetch_all(&**pool).await.unwrap();
//                                      title, content, seen, date
    let mut display_data: HashMap<i64, (String, String, bool, String)> = HashMap::new();
    for chat in chat_query {
        let c_id = chat.chat_id.unwrap();
        let title = sqlx::query!("SELECT * FROM chat WHERE id = ?", c_id).fetch_one(&**pool).await.unwrap().name.unwrap();
        let first_message_result = sqlx::query!("SELECT * FROM message WHERE chat_id = ? ORDER BY time DESC LIMIT 1", c_id).fetch_optional(&**pool).await.unwrap();
        let first_message = match first_message_result {
            Some(n) => n.content.unwrap(),
            None => "".to_string()
        };
        display_data.insert(c_id, (title, first_message, chat.seen.unwrap(), ChatData::date_from_epoch(chat.last_interact.unwrap())));
    }

    serde_json::to_string(&display_data).unwrap()
}
