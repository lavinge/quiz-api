use super::{Question, Quiz};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(ToSchema, Clone, Debug, Serialize, Deserialize)]
pub struct QuizModification {
    pub name: String,
    pub icon: String,
    pub questions: Vec<Question>,
}

impl QuizModification {
    pub fn new_quiz(self) -> Quiz {
        Quiz {
            _id: Uuid::new_v4().to_string(),
            name: self.name,
            icon: self.icon,
            questions: self.questions,
        }
    }

    pub fn update_quiz(self, id: &str) -> Quiz {
        Quiz {
            _id: id.to_string(),
            name: self.name,
            icon: self.icon,
            questions: self.questions,
        }
    }
}
