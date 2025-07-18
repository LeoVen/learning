use std::str::FromStr;
use std::time::Duration;

use common::storage::Storage;

use crate::models::PresignedRequestResult;

pub struct UploadService {
    storage: Storage,
    bucket: String,
}

impl UploadService {
    #[tracing::instrument(skip(storage))]
    pub fn new(storage: Storage, bucket: String) -> Self {
        tracing::info!("Setup upload service");
        Self { storage, bucket }
    }

    // https://docs.aws.amazon.com/AmazonS3/latest/API/s3_example_s3_Scenario_PresignedUrl_section.html
    #[tracing::instrument(skip_all)]
    pub async fn get_presigned(&self, file_name: &str) -> anyhow::Result<PresignedRequestResult> {
        let expires_in = Duration::from_secs(100);

        let presigned = self
            .storage
            .get_presigned_put(&self.bucket, file_name, expires_in)
            .await?;

        Ok(PresignedRequestResult {
            uri: http::uri::Uri::from_str(presigned.uri())?,
            method: http::Method::from_bytes(presigned.method().as_bytes())?,
            headers: presigned
                .headers()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        })
    }
}
