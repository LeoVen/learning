use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::header::CONTENT_TYPE;
use serde_json::Value;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_str("application/x-www-form-urlencoded").unwrap(),
    );

    let response = client
        .post("http://localhost:8080/realms/example/protocol/openid-connect/token")
        .headers(headers)
        .body("username=username&password=password&client_id=admin-cli&grant_type=password")
        .send()
        .await?;

    let value = response.json::<Value>().await?;

    let token = value
        .get("access_token")
        .expect("could not get access token from resulting body");

    let text = client
        .get("http://localhost:4000/admin/Testing")
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    println!("Result: {}", text);

    Ok(())
}
