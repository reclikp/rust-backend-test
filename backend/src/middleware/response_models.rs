use std::io::Cursor;
use rocket::http::Status;
use rocket::Request;
use rocket::serde::Serialize;
use rocket::response::{Responder, Response as RocketResponse};

#[derive(Debug)]
pub enum NetworkResponse {
    // #[response(status = 201)]
    Created(String),
    // #[response(status = 400)]
    BadRequest(String),
    // #[response(status = 401)]
    Unauthorized(String),
    // #[response(status = 403)]
    Forbidden(String),
    // #[response(status = 404)]
    NotFound(String),
    // #[response(status = 409)]
    Conflict(String),
    // #[response(status = 500)]
    InternalError(Option<String>),
}

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
    AuthenticationToken(String),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}

impl NetworkResponse {
    pub fn status(&self) -> Status {
        match self {
            NetworkResponse::Created(_) => Status::Created,
            NetworkResponse::BadRequest(_) => Status::BadRequest,
            NetworkResponse::Unauthorized(_) => Status::Unauthorized,
            NetworkResponse::Forbidden(_) => Status::Forbidden,
            NetworkResponse::NotFound(_) => Status::NotFound,
            NetworkResponse::Conflict(_) => Status::Conflict,
            NetworkResponse::InternalError(_) => Status::InternalServerError,
        }
    }

    pub fn message(&self) -> String {
        match self {
            NetworkResponse::Created(msg)
            | NetworkResponse::BadRequest(msg)
            | NetworkResponse::Unauthorized(msg)
            | NetworkResponse::Forbidden(msg)
            | NetworkResponse::NotFound(msg)
            | NetworkResponse::Conflict(msg) => msg.clone(),
            NetworkResponse::InternalError(Some(msg)) => msg.clone(),
            NetworkResponse::InternalError(None) => "Internal server error".to_string(),
        }
    }
}

impl<'r> Responder<'r, 'static> for NetworkResponse {
    fn respond_to(self, _request: &'r Request<'_>) -> rocket::response::Result<'static> {
        let status = self.status();
        let message = self.message();

        RocketResponse::build()
        .status(status)
        .sized_body(message.len(), Cursor::new(message))
        .ok()
    }
}
