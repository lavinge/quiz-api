use crate::data::logos::{Logo, Upload};
use crate::data::quizes::{Question, Quiz, QuizModification};
use crate::data::HttpError;
use crate::handlers;
use utoipa::{openapi, OpenApi};
use utoipa_redoc::{Redoc, Servable};

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::quizes::list::list,
        handlers::quizes::create::create,
        handlers::quizes::get::get,
        handlers::quizes::update::update,
        handlers::quizes::delete::delete,
        handlers::logos::upload::upload
    ),
    components(schemas(Quiz, Question, QuizModification, Upload, Logo, HttpError)),
    tags(
        (
            name = "quizes",
            description = "используя API модуля quizes можно вносить изменения в коллекцию опросов,
            а также получать актуальную информацию об опросах."
        ),
        (
            name = "logos",
            description = "API модуля logos предоставляет метод для загрузки изображений,
            которые затем могут быть использованы в качестве логотипа опроса."
        )
    )
)]
struct Specification;

pub fn specification() -> Redoc<'static, 'static, openapi::OpenApi> {
    Redoc::with_url("/specification", Specification::openapi())
}
