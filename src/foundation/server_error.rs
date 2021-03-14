use crate::foundation::server_object::{MessageContent, ResBody};
use actix_web::http::{header, StatusCode};
use actix_web::{dev::HttpResponseBuilder, error, HttpResponse};
use derive_more::{Display, Error};
use log;
use serde::Serialize;

#[derive(Debug, Display, Error, Serialize, Clone)]
pub enum ServerErrorType {
    InternalError,
    BadRequest,
    Timeout,
}
#[derive(Serialize, Debug, Clone)]
pub struct ServerError {
    pub error_type: ServerErrorType,
    pub message_list: Vec<MessageContent>,
}

impl<T> From<std::sync::PoisonError<T>> for ServerError {
    fn from(_e: std::sync::PoisonError<T>) -> Self {
        Self::new_internal_error("Could not access data.")
    }
}
impl ServerError {
    pub fn new(error_type: ServerErrorType, message: impl Into<String>) -> Self {
        Self {
            error_type: error_type,
            message_list: vec![MessageContent::new("", message)],
        }
    }
    pub fn new_internal_error(message: impl Into<String>) -> Self {
        Self {
            error_type: ServerErrorType::InternalError,
            message_list: vec![MessageContent::new("", message)],
        }
    }
    pub fn new_bad_request(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            error_type: ServerErrorType::BadRequest,
            message_list: vec![MessageContent::new(field, message)],
        }
    }
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.error_type)
    }
}
impl error::ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        log::debug!("error: {}", self);
        let res = ResBody {
            message_list: self.message_list.clone(),
            content: vec![String::from("not content")],
        };
        HttpResponseBuilder::new(self.status_code())
            .header(header::CONTENT_TYPE, "application/json")
            .json(res)
    }

    fn status_code(&self) -> StatusCode {
        match self.error_type {
            ServerErrorType::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ServerErrorType::BadRequest => StatusCode::BAD_REQUEST,
            ServerErrorType::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}
