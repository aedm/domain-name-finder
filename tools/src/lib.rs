use anyhow::{anyhow, Result};

pub fn get_env(name: &str) -> Result<String> {
    if let Ok(var) = std::env::var(name) {
        if var.len() > 0 {
            return Ok(var);
        }
    }
    Err(anyhow!("Missing environment variable '{name}'."))
}

pub async fn fetch_json<Req: serde::ser::Serialize, Resp: serde::de::DeserializeOwned>(
    url: &str,
    request_payload: &Req,
) -> Result<Resp> {
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .json(&request_payload)
        .header("User-Agent", "utils/0.1.0")
        .send()
        .await?;
    Ok(res.json::<Resp>().await?)
}

pub async fn send_request<Req: serde::ser::Serialize>(
    url: &str,
    access_token: &str,
    method: reqwest::Method,
    payload: &Req,
) -> Result<reqwest::Response, reqwest::Error> {
    reqwest::Client::new()
        .request(method, url)
        .header("User-Agent", "utils/0.1.0")
        .header("Authorization", format!("bearer {access_token}"))
        .json(payload)
        .send()
        .await
}
