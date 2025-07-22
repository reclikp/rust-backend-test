use rocket::http::{ContentType, Status};
use rocket::Request;
use rocket::response::{Responder, Result, Response as RocketResponse};
use rocket::serde::json::{serde_json, Json};
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
pub struct StatusCode {
    pub code: u16,
    pub reason: &'static str,
}

impl StatusCode {
    pub const OK: StatusCode = StatusCode { code: 200, reason: "OK" };
    pub const BAD_REQUEST: StatusCode = StatusCode { code: 400, reason: "Bad Request" };
    pub const UNAUTHORIZED: StatusCode = StatusCode { code: 401, reason: "Unauthorized" };
    pub const FORBIDDEN: StatusCode = StatusCode { code: 403, reason: "Forbidden" };
    pub const NOT_FOUND: StatusCode = StatusCode { code: 404, reason: "Not Found" };
    pub const INTERNAL_SERVER_ERROR: StatusCode = StatusCode { code: 500, reason: "Internal Server Error" };
}

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
    pub fn new(status: StatusCode, description: &str) -> Self {
        Self {
            error: ErrorDetail {
                code: status.code,
                reason: status.reason.to_string(),
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
