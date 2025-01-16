pub mod chat;
pub mod user;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response<T: Serialize> {
    pub code: u64,
    pub message: String,
    pub data: T,
}

impl<T: Serialize> Response<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 0,
            message: "success".to_string(),
            data,
        }
    }
}
