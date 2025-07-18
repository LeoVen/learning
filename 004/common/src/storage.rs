use std::time::Duration;

use anyhow::Context;
use aws_sdk_s3::presigning::PresignedRequest;
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::primitives::ByteStream;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct StorageConfig {
    pub endpoint_url: String,
    pub force_path_style: bool,
}

#[derive(Clone)]
pub struct Storage {
    client: aws_sdk_s3::Client,
}

impl Storage {
    #[tracing::instrument]
    pub async fn new(storage_config: &StorageConfig) -> Self {
        dotenvy::dotenv().ok();

        tracing::info!("Setting up S3 client");

        let config = aws_config::load_from_env().await;
        // Overrides
        let config = config
            .into_builder()
            .endpoint_url(storage_config.endpoint_url.clone())
            .build();

        // https://docs.aws.amazon.com/sdk-for-rust/latest/dg/endpoints.html
        let s3_config = aws_sdk_s3::config::Builder::from(&config)
            // This is needed because this is minio's default
            .force_path_style(storage_config.force_path_style)
            .build();
        let client = aws_sdk_s3::Client::from_conf(s3_config);

        tracing::info!("S3 Client setup");

        Self { client }
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_presigned_put(
        &self,
        bucket: &str,
        key: &str,
        expires_in: Duration,
    ) -> anyhow::Result<PresignedRequest> {
        tracing::trace!("Get Presigned");

        // https://stackoverflow.com/questions/59693471/how-to-setup-minio-server-to-use-virtual-hosted-style
        self.client
            .put_object()
            .bucket(bucket)
            .key(key)
            .presigned(PresigningConfig::builder().expires_in(expires_in).build()?)
            .await
            .context("Failed to generate presigned URL")
    }

    #[tracing::instrument(skip(self))]
    pub async fn stream_file(&self, bucket: &str, key: &str) -> anyhow::Result<ByteStream> {
        let get_obj = self
            .client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await?;

        Ok(get_obj.body)
    }
}
