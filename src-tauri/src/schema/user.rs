use entity::user::ActiveModel;
use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};
use utils::hash_password;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

impl UserLogin {
    pub fn hash_password(mut self) -> Self {
        self.password = hash_password(&self.password);
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRegister {
    pub username: String,
    pub password: String,
    pub email: String,
    pub code: String,
}

impl UserRegister {
    pub fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            username: ActiveValue::Set(self.username),
            password: ActiveValue::Set(self.password),
            email: ActiveValue::Set(self.email),
            ..Default::default()
        }
    }
}
