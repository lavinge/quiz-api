use crate::data::quizes::Quiz;
use crate::providers::mongodb;
use crate::utils::error::Error;
use rocket::serde::json::Json;
use rocket::State;

#[allow(unused_imports)]
use crate::data::HttpError;

/// Получить список опросов
#[utoipa::path(
    tag = "quizes",
    get,
    path = "/quizes/list",
    responses(
        (status = 200, description = "Список опросов", body = [Quiz]),
        (status = 500, description = "Внутренняя ошибка сервера", body = HttpError),
    )
)]
#[get("/quizes/list")]
pub async fn list(mongodb: &State<mongodb::Provider>) -> Result<Json<Vec<Quiz>>, Error> {
    let quizes = mongodb.list().await?;

    Ok(Json(quizes))
}
