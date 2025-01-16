use std::io;

use serde::Serialize;
use thiserror::Error;
use actix_web::{http::StatusCode, HttpResponse, ResponseError};


pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Serialize)]
pub struct Response<T: Serialize> {
    pub code: i32,
    pub data: Option<T>,
    pub message: String,
}

impl<T: Serialize> Response<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 0,
            data: Some(data),
            message: "success".to_string(),
        }
    }

    pub fn error(error: &Error) -> Self {
        Self {
            code: -1,
            data: None,
            message: error.to_string(),
        }
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Username or password is incorrect")]
    UsernameOrPasswordIncorrect,
    #[error("Authorization is incorrect")]
    AuthorizationIncorrect,
    #[error("Email code is incorrect")]
    EmailCodeIncorrect,
    #[error("Email already exists")]
    EmailAlreadyExists,
    #[error(transparent)]
    DatabaseError(#[from] sea_orm::DbErr),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    LettreError(#[from] lettre::error::Error),
    #[error(transparent)]
    ParseError(#[from] anyhow::Error),
    #[error(transparent)]
    TeraError(#[from] tera::Error),
    #[error(transparent)]
    RedisError(#[from] deadpool_redis::PoolError),
    #[error(transparent)]
    RedisCmdError(#[from] deadpool_redis::redis::RedisError),
    #[error(transparent)]
    ActixError(#[from] actix_web::Error),
    #[error("Session not found")]
    SessionNotFound,
    #[error("No authorization")]
    NoAuthorization,
    #[error("Error: {0}")]
    CustomError(String),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        log::error!("error: {}", self);
        let response: Response<Error> = Response::error(self);
        serializer.serialize_str(&serde_json::to_string(&response).unwrap())
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}