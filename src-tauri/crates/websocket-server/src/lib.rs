use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TextMessage {
    pub receiver_id: Option<i32>,
    pub group_id: Option<i32>,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthMessage {
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
// #[serde(tag = "type", content = "content")]
#[serde(rename_all = "camelCase")]
pub enum Message {
    // #[serde(rename = "auth")]
    Auth(AuthMessage),
    // #[serde(rename = "text")]
    Text(TextMessage),
}
