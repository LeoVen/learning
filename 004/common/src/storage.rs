use std::time::Duration;

use anyhow::Context;
use aws_sdk_s3::presigning::PresignedRequest;
use aws_sdk_s3::presigning::PresigningConfig;

#[derive(Clone)]
pub struct Storage {
    client: aws_sdk_s3::Client,
}

impl Storage {
    #[tracing::instrument]
    pub async fn new() -> Self {
        dotenvy::dotenv().ok();

        tracing::info!("Setting up S3 client");

        let config = aws_config::load_from_env().await;
        // Overrides
        let config = config
            .into_builder()
            .endpoint_url("http://localhost:9000") // TODO move to config
            .build();
        // https://docs.aws.amazon.com/sdk-for-rust/latest/dg/endpoints.html
        let s3_config = aws_sdk_s3::config::Builder::from(&config)
            .force_path_style(true) // This is needed because this is minio's default
            .build();
        let client = aws_sdk_s3::Client::from_conf(s3_config);

        tracing::info!("S3 Client setup");

        Self { client }
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_presigned(
        &self,
        bucket: &str,
        object: &str,
        expires_in: Duration,
    ) -> anyhow::Result<PresignedRequest> {
        tracing::trace!("Get Presigned");

        // https://stackoverflow.com/questions/59693471/how-to-setup-minio-server-to-use-virtual-hosted-style
        self.client
            .put_object()
            .bucket(bucket)
            .key(object)
            .presigned(PresigningConfig::builder().expires_in(expires_in).build()?)
            .await
            .context("Failed to generate presigned URL")
    }
}
