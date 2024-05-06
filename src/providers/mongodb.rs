use crate::data::quizes::{Quiz, QuizModification};
use crate::utils::error::Error;
use core::time::Duration;
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, to_bson},
    options::{ClientOptions, FindOneAndUpdateOptions, ReturnDocument},
    Client, Collection,
};
use std::env;

/// Постав
pub struct Provider {
    client: Client,
}

impl Provider {
    pub async fn build() -> Result<Provider, Error> {
        let username = env::var("MONGODB_USERNAME").unwrap_or("".to_string());

        let password = env::var("MONGODB_PASSWORD").unwrap_or("".to_string());

        let hostname = env::var("MONGODB_HOSTNAME").unwrap_or("".to_string());

        let port = env::var("MONGODB_PORT").unwrap_or("".to_string());

        let url = format!("mongodb://{username}:{password}@{hostname}:{port}");

        let mut options = ClientOptions::parse(url).await?;

        options.server_selection_timeout = Some(Duration::from_millis(5000));

        options.connect_timeout = Some(Duration::from_millis(5000));

        let client = mongodb::Client::with_options(options)?;

        client
            .database("admin")
            .run_command(doc! { "ping": 1 }, None)
            .await?;

        Ok(Provider { client })
    }

    fn collection(&self) -> Collection<Quiz> {
        self.client.database("quizes").collection("quizes")
    }

    pub async fn create(&self, new_modification: QuizModification) -> Result<Quiz, Error> {
        let quiz = new_modification.new_quiz();

        self.collection().insert_one(&quiz, None).await?;

        Ok(quiz)
    }

    pub async fn list(&self) -> Result<Vec<Quiz>, Error> {
        let mut cursor = self.collection().find(None, None).await?;

        let mut quizes: Vec<Quiz> = vec![];

        while let Some(quiz) = cursor.try_next().await? {
            quizes.push(quiz);
        }

        Ok(quizes)
    }

    pub async fn get(&self, id: &str) -> Result<Option<Quiz>, Error> {
        let quiz = self
            .collection()
            .find_one(
                doc! {
                    "_id": id
                },
                None,
            )
            .await?;

        Ok(quiz)
    }

    pub async fn update(
        &self,
        id: &str,
        new_modification: QuizModification,
    ) -> Result<Option<Quiz>, Error> {
        let options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        let quiz = new_modification.update_quiz(id);

        let updated_quiz = self
            .collection()
            .find_one_and_update(
                doc! { "_id": &quiz._id },
                doc! { "$set": to_bson(&quiz)?  },
                options,
            )
            .await?;

        Ok(updated_quiz)
    }

    pub async fn delete(&self, id: &str) -> Result<Option<Quiz>, Error> {
        let deleted_quiz = self
            .collection()
            .find_one_and_delete(
                doc! {
                    "_id": id
                },
                None,
            )
            .await?;

        Ok(deleted_quiz)
    }
}
