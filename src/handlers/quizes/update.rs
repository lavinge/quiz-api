use crate::data::quizes::{Quiz, QuizModification};
use crate::providers::mongodb;
use crate::utils::error::Error;
use rocket::serde::json::Json;
use rocket::State;

#[allow(unused_imports)]
use crate::data::HttpError;

/// Обновить опрос по id
#[utoipa::path(
    tag = "quizes",
    put,
    path = "/quizes/update/{id}",
    request_body = QuizModification,
    responses(
        (status = 200, description = "Опрос успешно обновлен", body = Quiz),
        (status = 404, description = "Опрос не найден", body = HttpError),
        (status = 422, description = "Информация об опросе заполнена некорректно", body = HttpError),
        (status = 500, description = "Внутренняя ошибка сервера", body = HttpError),
    )
)]
#[put("/quizes/update/<id>", data = "<quiz_update>")]
pub async fn update(
    mongodb: &State<mongodb::Provider>,
    id: &str,
    quiz_update: Json<QuizModification>,
) -> Result<Json<Quiz>, Error> {
    let updated_quiz = mongodb.update(id, quiz_update.0).await?;

    match updated_quiz {
        Some(quiz) => Ok(Json(quiz)),
        None => Err(Error::NotFound),
    }
}
