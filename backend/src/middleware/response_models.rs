use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Result};
use rocket::{Request, Response as RocketResponse, Rocket, State};
use rocket::serde::json::Json;
use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct Response<T> {
    pub data: T,
}

impl<T: Serialize> Response<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl<'r, T: Serialize> Responder<'r, 'static> for Response<T> {
    fn respond_to(self, req: &'r Request<'_>) -> Result<'static> {
        RocketResponse::build_from(Json(self).respond_to(req)?)
        .status(Status::Ok)
        .header(ContentType::JSON)
        .ok()
    }
}

#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
}

#[derive(Serialize, Debug)]
pub struct ErrorDetail {
    pub code: u16,
    pub reason: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl ErrorResponse {
    pub fn new(status: Status, description: &str) -> Self {
        Self {
            error: ErrorDetail {
                code: status.code,
                reason: status.reason().unwrap_or("Unknown error").to_string(),
                description: description.to_string(),
                details: None,
            }
        }
    }
}

impl<'r> Responder<'r, 'static> for ErrorResponse {
    fn respond_to(self, req: &'r Request<'_>) -> Result<'static> {
        let status = Status::from_code(self.error.code).unwrap_or(Status::InternalServerError);

        RocketResponse::build_from(Json(self).respond_to(req)?)
            .status(status)
            .header(ContentType::JSON)
            .ok()
    }
}








































// impl NetworkResponse {
//     pub fn status(&self) -> Status {
//         match self {
//             NetworkResponse::Created(_) => Status::Created,
//             NetworkResponse::BadRequest(_) => Status::BadRequest,
//             NetworkResponse::Unauthorized(_) => Status::Unauthorized,
//             NetworkResponse::Forbidden(_) => Status::Forbidden,
//             NetworkResponse::NotFound(_) => Status::NotFound,
//             NetworkResponse::Conflict(_) => Status::Conflict,
//             NetworkResponse::InternalError(_) => Status::InternalServerError,
//         }
//     }
//
//     pub fn message(&self) -> String {
//         match self {
//             NetworkResponse::Created(msg)
//             | NetworkResponse::BadRequest(msg)
//             | NetworkResponse::Unauthorized(msg)
//             | NetworkResponse::Forbidden(msg)
//             | NetworkResponse::NotFound(msg)
//             | NetworkResponse::Conflict(msg) => msg.clone(),
//             NetworkResponse::InternalError(Some(msg)) => msg.clone(),
//             NetworkResponse::InternalError(None) => "Internal server error".to_string(),
//         }
//     }
// }

// impl<'r> Responder<'r, 'static> for NetworkResponse {
//     fn respond_to(self, _request: &'r Request<'_>) -> rocket::response::Result<'static> {
//         let status = self.status();
//         let message = self.message();
//
//         RocketResponse::build()
//         .status(status)
//         .sized_body(message.len(), Cursor::new(message))
//         .ok()
//     }
// }
