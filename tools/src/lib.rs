use anyhow::{anyhow, Result};
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Method;
use std::cmp::min;
use std::fs::File;
use std::io::Write;
use std::str::Bytes;

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

pub async fn download_stream_to_file(
    mut stream: impl futures_core::Stream<Item = reqwest::Result<Bytes>>,
) -> Result<()> {
    let total_size = res
        .content_length()
        .context("Failed to get content length")?;

    // Indicatif setup
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("#>-"));
    pb.set_message(format!("Downloading {}", ZONE_FILE_URL));

    // download chunks
    let path = "com.zone.txt.gz";
    let mut file = File::create(path)?;
    let mut downloaded: u64 = 0;

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk)?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(format!("Downloaded {} to {}", ZONE_FILE_URL, path));
    Ok(())
}
