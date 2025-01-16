pub mod handle_stream;

use std::collections::HashMap;
use actix_web::{rt, web, App, HttpRequest, HttpResponse, HttpServer};
use actix_ws::{AggregatedMessage, Session};
use error::{Error as CusError, Response, Result};
use futures_util::StreamExt as _;
use handle_stream::handle_message;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[derive(Default)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub user_connections: UserConnections,
}

#[derive(Default)]
pub struct UserConnections(RwLock<HashMap<UserId, Session>>);

type UserId = i32;

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
#[serde(rename_all = "camelCase")]
pub struct WebRTCMessage {
    pub receiver_id: Option<i32>,
    pub group_id: Option<i32>,
    pub sdp: String,
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
}

pub const AUTH_TYPE: &str = "auth";
pub const TEXT_TYPE: &str = "text";


impl UserConnections {
    pub async fn new() -> Self {
        Self(RwLock::new(HashMap::new()))
    }

    pub async fn add_session(&self, user_id: UserId, session: Session) {
        let mut map = self.0.write().await;
        (*map).insert(user_id, session);
    }

    pub async fn remove_session(&self, user_id: UserId) {
        let mut map = self.0.write().await;
        (*map).remove(&user_id);
    }

    pub async fn get_session(&self, user_id: UserId) -> Option<Session> {
        let map = self.0.read().await;
        map.get(&user_id).cloned()
    }

    pub async fn is_auth(&self, user_id: UserId) -> bool {
        let map = self.0.read().await;
        map.contains_key(&user_id)
    }
    
}

async fn echo(
    state: web::Data<AppState>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, CusError> {
    let path = req.query_string();
    let user_id = path.split("&").find(|s| s.starts_with("userId=")).ok_or(CusError::CustomError("userId is required".to_string()))?.split("=").last().ok_or(CusError::CustomError("userId is required".to_string()))?.parse::<i32>()?;
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
                Ok(AggregatedMessage::Close(None)) => {
                    println!("close");
                    state.user_connections.remove_session(user_id).await;
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
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState::default()))
            .route("/ws", web::get().to(echo))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
