use crate::data::logos::{Logo, Upload};
use crate::providers::s3;
use crate::utils::error::Error;
use rocket::form::Form;
use rocket::serde::json::Json;
use rocket::State;

#[allow(unused_imports)]
use crate::data::HttpError;

/// Загрузить лого опроса
///
/// Метод принимает изображения в форматах png и jpeg. Размер изображения не должен превышать 1мб. В случае
/// успешной обработки изображения ответ будет содержать ссылку на него.  
#[utoipa::path(
    tag = "logos",
    post,
    path = "/logos/upload",
    request_body(content = Upload, content_type = "multipart/form-data"),
    responses(
        (status = 200, description = "Изображение успешно сохранено", body = Logo),
        (status = 413, description = "Размер изображения превышает органичение (1мб)", body = HttpError),
        (status = 422, description = "Расширение изображения не входит в список допустимых (.png, .jpeg)", body = HttpError),
        (status = 500, description = "Произошла ошибка на сервере", body = HttpError),
    )
)]
#[post("/logos/upload", data = "<upload>")]
pub async fn upload(
    s3: &State<s3::Provider>,
    upload: Form<Upload<'_>>,
) -> Result<Json<Logo>, Error> {
    match upload.image.content_type() {
        Some(content_type) => {
            let ext = content_type.sub();

            if ext == "png" || ext == "jpeg" {
                let url = s3.put(&upload.image, ext.as_str()).await?;

                Ok(Json(Logo { url }))
            } else {
                Err(Error::BadRequest())
            }
        }
        None => Err(Error::BadRequest()),
    }
}
