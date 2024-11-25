use async_trait::async_trait;
use pingora::lb::health_check::HealthCheck;
use pingora::lb::Backend;
use pingora::protocols::l4::socket::SocketAddr;
use pingora::BError;
use pingora::ErrorType;
use reqwest::Method;
use reqwest::Url;

pub struct CustomHealthCheck {
    method: Method,
    path: String,
    query: Vec<(String, String)>,
    client: reqwest::Client,
}

impl CustomHealthCheck {
    pub fn new(method: Method, path: impl ToString, query: Option<Vec<(String, String)>>) -> Self {
        Self {
            method,
            path: path.to_string(),
            query: query.unwrap_or_default(),
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl HealthCheck for CustomHealthCheck {
    /// Check the given backend.
    ///
    /// `Ok(())`` if the check passes, otherwise the check fails.
    async fn check(&self, target: &Backend) -> Result<(), BError> {
        // Skip unsupported schemas
        match target.addr {
            SocketAddr::Inet(socket_addr) => match socket_addr {
                std::net::SocketAddr::V4(_) => {
                    // OK
                }
                std::net::SocketAddr::V6(_) => return Ok(()),
            },
            SocketAddr::Unix(_) => return Ok(()),
        };

        let url = &format!("http://{}", target.addr);
        let mut url: Url = Url::parse(url).map_err(|err| {
            let error = err.to_string();

            tracing::error!(error, url, "health check url parse error");

            pingora::Error::new(ErrorType::UnknownError)
        })?;

        url.set_path(&self.path);

        let req = self
            .client
            .request(self.method.clone(), url)
            .query(&self.query)
            .send()
            .await;

        let resp = match req {
            Ok(req) => req,
            Err(err) => {
                if let Some(code) = err.status() {
                    return Err(pingora::Error::new(ErrorType::HTTPStatus(code.as_u16())));
                } else if err.is_connect() {
                    return Err(pingora::Error::new(ErrorType::ConnectError));
                }

                let error = err.to_string();
                tracing::error!(
                    error,
                    "health check unknown error by sending request to upstream"
                );

                return Err(pingora::Error::new(ErrorType::UnknownError));
            }
        };

        if resp.status() != 200 {
            return Err(pingora::Error::new(ErrorType::HTTPStatus(
                resp.status().as_u16(),
            )));
        }

        Ok(())
    }

    /// Called when the health changes for a [Backend].
    async fn health_status_change(&self, _target: &Backend, _healthy: bool) {
        // Empty
    }

    /// This function defines how many *consecutive* checks should flip the health of a backend.
    ///
    /// For example: with `success``: `true`: this function should return the
    /// number of check need to flip from unhealthy to healthy.
    fn health_threshold(&self, _success: bool) -> usize {
        1
    }
}
