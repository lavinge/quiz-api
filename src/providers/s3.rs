use crate::utils::error::Error;
use aws_config::Region;
use aws_credential_types::Credentials;
use aws_sdk_s3::Client;
use aws_smithy_types::byte_stream::ByteStream;
use rocket::fs::TempFile;
use std::env;
use uuid::Uuid;

pub struct Provider {
    client: Client,
    endpoint: String,
    bucket: String,
}

impl Provider {
    pub async fn build() -> Result<Provider, Error> {
        let access_key = env::var("S3_ACCESS_KEY").unwrap_or("".to_string());

        let secret_key = env::var("S3_SECRET_KEY").unwrap_or("".to_string());

        let endpoint = env::var("S3_ENDPOINT").unwrap_or("".to_string());

        let bucket = env::var("S3_BUCKET").unwrap_or("".to_string());

        let credentials = Credentials::from_keys(access_key, secret_key, None);

        let config = aws_config::from_env()
            .endpoint_url(&endpoint)
            .region(Region::new("ru-msk"))
            .credentials_provider(credentials)
            .load()
            .await;

        Ok(Provider {
            client: Client::new(&config),
            endpoint,
            bucket,
        })
    }

    pub async fn put(&self, temp_file: &TempFile<'_>, ext: &str) -> Result<String, Error> {
        let filename = format!("{}.{}", Uuid::new_v4(), ext);

        let byte_stream = ByteStream::from_path(temp_file.path().unwrap()).await?;

        let _res = self
            .client
            .put_object()
            .bucket(&self.bucket)
            .key(&filename)
            .body(byte_stream)
            .send()
            .await?;

        Ok(format!("{}/{}/{}", self.endpoint, self.bucket, filename))
    }
}
