use rocket::fs::TempFile;
use utoipa::ToSchema;

#[derive(ToSchema, FromForm)]
pub struct Upload<'r> {
    pub image: TempFile<'r>,
}
