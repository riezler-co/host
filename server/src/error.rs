use std::io::Cursor;

use rocket::{
    http::{ContentType, Status},
    response::{self, Responder},
    Request, Response,
};
use serde::Serialize;
use sqlx;
use thiserror::Error;

#[derive(Debug, Serialize)]
pub struct Message {
    pub code: String,
}

#[derive(Error, Debug, PartialEq, Copy, Clone)]
pub enum ApiError {
    #[error("internal_error")]
    InternalServerError,

    #[error("not_found")]
    NotFound,
}

impl ApiError {
    fn get_status(&self) -> Status {
        match self {
            ApiError::InternalServerError => Status::InternalServerError,
            ApiError::NotFound => Status::NotFound,
        }
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            _ => ApiError::InternalServerError,
        }
    }
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _req: &'r Request<'_>) -> response::Result<'static> {
        let body = Message {
            code: self.to_string(),
        };

        let body = match serde_json::to_string(&body) {
            Ok(json) => json,
            Err(_) => return Response::build().status(Status::InternalServerError).ok(),
        };

        let status = self.get_status();
        Response::build()
            .status(status)
            .sized_body(body.len(), Cursor::new(body))
            .header(ContentType::new("text", "json"))
            .ok()
    }
}
