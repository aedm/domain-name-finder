use anyhow::{anyhow, Context, Result};
use aws_config::meta::region::RegionProviderChain;
use futures_util::TryStreamExt;
use futures_util::{pin_mut, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use reqwest::Method;
use std::cell::RefCell;
use std::cmp::min;
use std::fs::File;
use std::io::{Error, ErrorKind, Write};
use std::rc::Rc;
use std::str::Bytes;
use tokio_util::io::StreamReader;

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

fn type_of<T>(_: T) -> String {
    format!("{}", std::any::type_name::<T>())
}

pub async fn download_stream_to_file(mut res: reqwest::Response, path: &str) -> Result<()> {
    let total_size = res
        .content_length()
        .context("Failed to get content length")?;

    // Indicatif setup
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("#>-"));
    pb.set_message(format!("Downloading {}", path));
    let pb = Rc::new(RefCell::new(pb));
    let pb_clone = pb.clone();

    // download chunks
    // TODO: download speed is slow for some reason
    let mut stream = res.bytes_stream();
    let pb_stream = async_stream::stream! {
        while let Some(item) = stream.next().await {
            let chunk = item?;
            pb_clone.borrow_mut().inc(chunk.len() as u64);
            yield Ok(chunk);
        }
    };
    pin_mut!(pb_stream);

    let mut file = tokio::fs::File::create(path).await?;
    let mut stream_reader = StreamReader::new(
        pb_stream
            .map_err(|_: reqwest::Error| std::io::Error::new(ErrorKind::Other, "Download error")),
    );
    tokio::io::copy(&mut stream_reader, &mut file).await?;

    pb.borrow_mut()
        .finish_with_message(format!("Downloaded {}", path));
    Ok(())
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
