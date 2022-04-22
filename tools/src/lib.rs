pub mod process_zone_file;

use anyhow::{anyhow, Result};
use aws_config::meta::region::RegionProviderChain;
use itertools::Itertools;
use reqwest::Method;

pub fn get_env(name: &str) -> Result<String> {
    if let Ok(var) = std::env::var(name) {
        if var.len() > 0 {
            return Ok(var);
        }
    }
    Err(anyhow!("Missing environment variable '{name}'."))
}

pub async fn send_request<Req: serde::ser::Serialize>(
    url: &str,
    access_token: Option<&str>,
    method: reqwest::Method,
    payload: &Req,
) -> Result<reqwest::Response, reqwest::Error> {
    let mut request = reqwest::Client::new()
        .request(method, url)
        .header("User-Agent", "utils/0.1.0");
    if let Some(token) = access_token {
        request = request.header("Authorization", format!("bearer {token}"));
    }
    request.json(payload).send().await
}

pub async fn fetch_json<Req: serde::ser::Serialize, Resp: serde::de::DeserializeOwned>(
    url: &str,
    access_token: Option<&str>,
    request_payload: &Req,
) -> Result<Resp> {
    let response = send_request(url, access_token, Method::POST, request_payload).await?;
    Ok(response.json::<Resp>().await?)
}

pub async fn make_aws_s3_client() -> aws_sdk_s3::Client {
    let region_provider = RegionProviderChain::default_provider();
    let config = aws_config::from_env().region(region_provider).load().await;
    aws_sdk_s3::Client::new(&config)
}

pub async fn get_all_files_from_s3_bucket(
    client: &aws_sdk_s3::Client,
    bucket_name: &str,
) -> Result<Vec<String>> {
    let objects = client.list_objects_v2().bucket(bucket_name).send().await?;
    let contents = objects.contents.unwrap_or_default();
    Ok(contents.into_iter().map(|i| i.key).flatten().collect_vec())
}
