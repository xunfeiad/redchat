pub mod handle_stream;

use std::collections::HashMap;
use actix_web::{rt, web, App, HttpRequest, HttpResponse, HttpServer};
use actix_ws::{AggregatedMessage, Session};
use error::{Error as CusError, Result};
use futures_util::StreamExt as _;
use handle_stream::handle_message;
use sea_orm::{EntityTrait, DatabaseConnection, QueryFilter, ColumnTrait, prelude::Expr};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use std::sync::Arc;
use entity::user::{Entity as User, Column as UserColumn};
use entity::user_friend::{Entity as UserFriend, Column as UserFriendColumn};
use anyhow::Context;
#[derive(Default)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub user_connections: Arc<UserConnections>,
}

#[derive(Default)]
pub struct UserConnections(RwLock<HashMap<UserId, Session>>);

type UserId = i32;

#[derive(Serialize, Deserialize, Debug, Clone)]
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
#[serde(rename_all = "camelCase")]
pub struct WebRTCMessage {
    pub receiver_id: Option<i32>,
    pub group_id: Option<i32>,
    pub sender_name: Option<String>,
    pub content: String,
    pub sdp_type: String,
    pub call_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DisconnectMessage {
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "content")]
#[serde(rename_all = "camelCase")]
pub enum Message {
    #[serde(rename = "auth")]
    Auth(AuthMessage),
    #[serde(rename = "text")]
    Text(TextMessage),
    #[serde(rename = "webrtc")]
    WebRTC(WebRTCMessage),
    #[serde(rename = "disconnect")]
    Disconnect(DisconnectMessage),
}

pub const AUTH_TYPE: &str = "auth";
pub const TEXT_TYPE: &str = "text";



impl UserConnections {
    pub fn new() -> Self {
        Self(RwLock::new(HashMap::new()))
    }

    pub async fn add_session(&self, user_id: UserId, session: Session) {
        let mut map = self.0.write().await;
        println!("add session: {:?}", user_id);
        (*map).insert(user_id, session);
    }

    pub async fn remove_session(&self, user_id: UserId) {
        let mut map = self.0.write().await;
        println!("remove_session: {:?}", user_id);
        (*map).remove(&user_id);
    }

    pub async fn get_session(&self, user_id: UserId) -> Result<Session> {
        let map = self.0.read().await;
        map.get(&user_id).cloned().ok_or(CusError::SessionNotFound)
    }

    pub async fn is_auth(&self, user_id: UserId) -> bool {
        let map = self.0.read().await;
        map.contains_key(&user_id)
    }

    pub async fn get_session_ids(&self) -> Vec<UserId> {
        let map = self.0.read().await;
        map.keys().cloned().collect()
    }

    // When user_id is Some, send message to the specific user
    // When user_id is None, send message to all users except the skip_user_id(self)
    pub async fn find_session_and_send_message(&self, user_id: Option<UserId>, message: &Message, skip_user_id: Option<UserId>, db: Option<DatabaseConnection>) -> Result<()> {
        let mut map = self.0.write().await;

        match user_id {
            Some(user_id) => {
                let session = map.get_mut(&user_id).ok_or(CusError::SessionNotFound)?;
                session.text(serde_json::to_string(message)?).await.context("send message error")?;

            }
            None => {
                match skip_user_id {
                    Some(skip_user_id) => {
                        let notify_user_ids = UserFriend::find().filter(UserFriendColumn::UserId.eq(skip_user_id)).filter(UserFriendColumn::Status.eq(2)).all(db.as_ref().unwrap()).await?.into_iter().map(|user| user.friend_id).collect::<Vec<i32>>();
                        println!("skip_user_id: {:?}, notify_user_ids: {:?}, message: {:?}", skip_user_id, notify_user_ids, message);
                        for (key, value) in map.iter_mut() {
                            if notify_user_ids.contains(key){
                                value.text(serde_json::to_string(message)?).await.context("send message error")?;
                            }
                        }
                    }
                    None => {}
                }
            }
        }
        Ok(())
    }
}

async fn echo(
    state: web::Data<AppState>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, CusError> {
    let path = req.query_string();
    let user_id = path.split("&").find(|s| s.starts_with("userId=")).ok_or(CusError::CustomError("userId is required"))?.split("=").last().ok_or(CusError::CustomError("userId is required"))?.parse::<i32>()?;
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream
        .aggregate_continuations()
        // aggregate continuation frames up to 1MiB
        .max_continuation_size(2_usize.pow(20));
    // start task but don't wait for it
    rt::spawn(async move {
        // receive messages from websocket
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Text(ref text)) => {
                    if let Err(e) = handle_message(text.clone(), &state, session.clone(), user_id).await {
                        session.text(serde_json::to_string(&e).unwrap()).await.unwrap();
                    }
                }

                Ok(AggregatedMessage::Binary(bin)) => {
                    // echo binary message
                    session.binary(bin).await.unwrap();
                }

                Ok(AggregatedMessage::Ping(msg)) => {
                    session.pong(&msg).await.unwrap();
                }
                Ok(AggregatedMessage::Close(Some(_))) => {
                    println!("user disconnected: user_id: {}", user_id);
                    state.user_connections.remove_session(user_id).await;
                    if let Err(e) =User::update_many().col_expr(UserColumn::Online, Expr::value(false)).filter(UserColumn::Id.eq(user_id)).exec(&state.db).await{
                        println!("update user online error: {}", e);
                    };
                    if let Err(e) = state.user_connections.find_session_and_send_message(None, &Message::Disconnect(DisconnectMessage { user_id: user_id }), Some(user_id), Some(state.db.clone())).await {
                        println!("send disconnect signal error: {}", e);
                    };
                }

                _ => {}
            }
        }
    });

    // respond immediately with response connected to WS session
    Ok(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let user_connections = Arc::new(UserConnections::new());
    let db = db::get_db().await.expect("error while connecting to database");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                user_connections: user_connections.clone(),
                db: db.clone(),
            }))
            .route("/ws", web::get().to(echo))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
