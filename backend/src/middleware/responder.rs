use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use serde::Serialize;
use rocket::{Request, Response as RocketResponse};
use rocket::serde::json::Json;
use api::response_models::Response;

impl<'r, T: Serialize> Responder<'r, 'static> for Response<T> {
    fn respond_to(self, req: &'r Request<'_>) -> Result<'static> {
        RocketResponse::build_from(Json(self).respond_to(req)?)
            .status(Status::Ok)
            .header(ContentType::JSON)
            .ok()
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
