use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(ToSchema, Clone, Debug, Serialize, Deserialize)]
pub struct Question {
    pub text: String,
    pub answer: i32,
    pub options: Vec<String>,
}
