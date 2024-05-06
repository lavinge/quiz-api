use crate::data::quizes::{Quiz, QuizModification};
use crate::providers::mongodb;
use crate::utils::error::Error;
use rocket::serde::json::Json;
use rocket::State;

#[allow(unused_imports)]
use crate::data::HttpError;

/// Создать опрос
#[utoipa::path(
    tag = "quizes",
    post,
    path = "/quizes/create",
    request_body = QuizModification,
    responses(
        (status = 200, description = "Опрос успешно создан", body = Quiz),
        (status = 422, description = "Получена некорректная информация об опросе", body = HttpError),
        (status = 500, description = "Произошла ошибка на сервере", body = HttpError),
    )
)]
#[post("/quizes/create", data = "<new_quiz>")]
pub async fn create(
    mongodb: &State<mongodb::Provider>,
    new_quiz: Json<QuizModification>,
) -> Result<Json<Quiz>, Error> {
    let created_quiz = mongodb.create(new_quiz.0).await?;

    Ok(Json(created_quiz))
}
