use crate::data::quizes::Quiz;
use crate::providers::mongodb;
use crate::utils::error::Error;
use rocket::serde::json::Json;
use rocket::State;

#[allow(unused_imports)]
use crate::data::HttpError;

/// Удалить опрос по id
#[utoipa::path(
    tag = "quizes",
    delete,
    path = "/quizes/delete/{id}",
    responses(
        (status = 200, description = "Опрос успешно удален", body = Quiz),
        (status = 404, description = "Опрос не найден", body = HttpError),
        (status = 500, description = "Внутренняя ошибка сервера", body = HttpError),
    )
)]
#[delete("/quizes/delete/<id>")]
pub async fn delete(mongodb: &State<mongodb::Provider>, id: &str) -> Result<Json<Quiz>, Error> {
    let deleted_quiz = mongodb.delete(id).await?;

    match deleted_quiz {
        Some(quiz) => Ok(Json(quiz)),
        None => Err(Error::NotFound),
    }
}
