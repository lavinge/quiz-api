use super::Question;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Опрос
#[derive(ToSchema, Clone, Debug, Serialize, Deserialize)]
pub struct Quiz {
    pub _id: String,
    pub name: String,
    pub logo: String,
    pub questions: Vec<Question>,
}
