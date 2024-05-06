#[macro_use]
extern crate rocket;

mod data;
mod handlers;
mod providers;
mod utils;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount(
            "/",
            routes![
                handlers::quizes::create::create,
                handlers::quizes::list::list,
                handlers::quizes::get::get,
                handlers::quizes::update::update,
                handlers::quizes::delete::delete,
                handlers::logos::upload::upload
            ],
        )
        .register(
            "/",
            catchers![
                utils::catchers::not_found,
                utils::catchers::unprocessable_entity,
                utils::catchers::bad_request,
                utils::catchers::payload_too_large,
                utils::catchers::default
            ],
        )
        .mount("/", utils::openapi::specification())
        .manage(providers::s3::Provider::build().await.unwrap())
        .manage(providers::mongodb::Provider::build().await.unwrap())
        .launch()
        .await?;

    Ok(())
}
