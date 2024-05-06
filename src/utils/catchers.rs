use crate::data::HttpError;
use rocket::serde::json::Json;
use rocket::Request;

#[catch(413)]
pub fn payload_too_large(_: &Request) -> Json<HttpError<'static>> {
    Json(HttpError {
        message: "payload_too_large",
    })
}

#[catch(404)]
pub fn not_found(_: &Request) -> Json<HttpError<'static>> {
    Json(HttpError {
        message: "not_found",
    })
}

#[catch(422)]
pub fn unprocessable_entity(_: &Request) -> Json<HttpError<'static>> {
    Json(HttpError {
        message: "bad_request",
    })
}

#[catch(400)]
pub fn bad_request(_: &Request) -> Json<HttpError<'static>> {
    Json(HttpError {
        message: "bad_request",
    })
}

#[catch(default)]
pub fn default(_: &Request) -> Json<HttpError<'static>> {
    Json(HttpError {
        message: "internal_error",
    })
}
