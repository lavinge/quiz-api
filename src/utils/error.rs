use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Responder, Result};

#[derive(Debug)]
pub enum Error {
    Internal(String),
    NotFound,
    BadRequest(),
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _: &Request) -> Result<'static> {
        match self {
            Error::NotFound => Err(Status::NotFound),
            Error::BadRequest() => Err(Status::BadRequest),
            Error::Internal(_) => Err(Status::InternalServerError),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotFound => write!(f, "{}", "not_found"),
            Error::BadRequest() => write!(f, "{}", "bad_request"),
            Error::Internal(message) => write!(f, "{}", message),
        }
    }
}

impl<T: std::error::Error> From<T> for Error {
    fn from(value: T) -> Self {
        Self::Internal(value.to_string())
    }
}
