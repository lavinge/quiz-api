use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, ToSchema, Serialize)]
pub struct HttpError<'a> {
    pub message: &'a str,
}
