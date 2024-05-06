use serde::Serialize;
use utoipa::ToSchema;

#[derive(ToSchema, Serialize)]
pub struct Logo {
    pub url: String,
}
