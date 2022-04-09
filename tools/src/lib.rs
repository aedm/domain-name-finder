use anyhow::{anyhow, Context, Result};
use futures_util::TryStreamExt;
use futures_util::{pin_mut, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
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
    let mut stream = res.bytes_stream();
    // let mut rrr = StreamReader::new(stream.map(|data| -> Result<_, std::io::Error> {
    //     // let chunk = item?;
    //     if let Ok(chunk) = data {
    //         pb_clone.borrow_mut().inc(chunk.len() as u64);
    //         return Ok(data);
    //     }
    //     Err(std::io::Error::new(ErrorKind::Other, "oh no!"))
    // }));

    // TODO: download speed is slow for some reason
    let pb_stream = async_stream::stream! {
        while let Some(item) = stream.next().await {
            let chunk = item?;
            // file.write_all(&chunk)?;
            // let new = min(downloaded + (chunk.len() as u64), total_size);
            // downloaded += chunk.len() as u64;
            // pb_clone.borrow_mut().set_position(downloaded);
            pb_clone.borrow_mut().inc(chunk.len() as u64);
            yield Ok(chunk);
        }
    };
    pin_mut!(pb_stream);

    let mut file = tokio::fs::File::create(path).await?;
    fn convert_err(err: reqwest::Error) -> std::io::Error {
        todo!()
    }
    let mut reader = StreamReader::new(pb_stream.map_err(convert_err));
    tokio::io::copy(&mut reader, &mut file).await?;

    pb.borrow_mut()
        .finish_with_message(format!("Downloaded {}", path));
    Ok(())
}
