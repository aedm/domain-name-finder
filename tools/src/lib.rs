use anyhow::{anyhow, Result};
use reqwest::Method;

pub fn get_env(name: &str) -> Result<String> {
    if let Ok(var) = std::env::var(name) {
        if var.len() > 0 {
            return Ok(var);
        }
    }
    Err(anyhow!("Missing environment variable '{name}'."))
}

pub fn send_request<Req: serde::ser::Serialize>(
    url: &str,
    access_token: Option<&str>,
    method: reqwest::Method,
    payload: &Req,
) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let mut request = reqwest::blocking::Client::new()
        .request(method, url)
        .header("User-Agent", "utils/0.1.0");
    if let Some(token) = access_token {
        request = request.header("Authorization", format!("bearer {token}"));
    }
    request.json(payload).send()
}

pub fn fetch_json<Req: serde::ser::Serialize, Resp: serde::de::DeserializeOwned>(
    url: &str,
    access_token: Option<&str>,
    request_payload: &Req,
) -> Result<Resp> {
    let response = send_request(url, access_token, Method::POST, request_payload)?;
    Ok(response.json::<Resp>()?)
}
