use crate::data::quizes::Quiz;
use crate::providers::mongodb;
use crate::utils::error::Error;
use rocket::serde::json::Json;
use rocket::State;

#[allow(unused_imports)]
use crate::data::HttpError;

/// Получить опрос по id
#[utoipa::path(
    tag = "quizes",
    get,
    path = "/quizes/get/{id}",
    responses(
        (status = 200, description = "Опрос", body = Quiz),
        (status = 404, description = "Опрос не найден", body = HttpError),
        (status = 500, description = "Внутренняя ошибка сервера", body = HttpError),
    )
)]
#[get("/quizes/get/<id>")]
pub async fn get(mongodb: &State<mongodb::Provider>, id: &str) -> Result<Json<Quiz>, Error> {
    let quiz = mongodb.get(id).await?;

    match quiz {
        Some(quiz) => Ok(Json(quiz)),
        None => Err(Error::NotFound),
    }
}
