use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    pub id: i32,
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub last_message_time: DateTime<Utc>,
    pub last_message: Option<String>,
    pub unread: u64,
    pub online: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub id: i32,
    pub content: String,
    #[serde(rename = "type")]
    pub category: String,
    pub is_self: bool,
    pub timestamp: DateTime<FixedOffset>,
    pub status: String,
}
