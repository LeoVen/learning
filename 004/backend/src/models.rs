use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize)]
pub struct PresignedRequestResult {
    #[serde(with = "http_serde::method")]
    pub method: http::Method,
    #[serde(with = "http_serde::uri")]
    pub uri: http::uri::Uri,
    pub headers: HashMap<String, String>,
}
