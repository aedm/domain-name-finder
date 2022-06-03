extern crate dotenv;

use anyhow::{anyhow, Context, Result};
use chrono::DateTime;
use dotenv::dotenv;
use futures::stream::TryStreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::ErrorKind;
use tools::process_zone_file::process_zone_file;
use tools::{fetch_json, get_all_files_from_s3_bucket, get_env, make_aws_s3_client, response_to_reader, send_request, ObservedReader, send_request_blocking};

const AUTH_URL: &str = &"https://account-api.icann.org/api/authenticate";
const ZONE_FILE_URL: &str = &"https://czds-api.icann.org/czds/downloads/com.zone";
const DATE_FORMAT: &str = "%Y%m%d-%H%M%S";
const S3_FILE_EX: &str = ".txt.gz";

async fn fetch_access_token(username: &str, password: &str) -> Result<String> {
    #[derive(Deserialize, Debug)]
    struct AuthResponse {
        #[serde(rename = "accessToken")]
        access_token: String,
    }

    #[derive(Serialize, Debug)]
    struct AuthRequest<'a> {
        username: &'a str,
        password: &'a str,
    }

    let response: AuthResponse =
        fetch_json(AUTH_URL, None, &AuthRequest { username, password }).await?;
    Ok(response.access_token)
}

// async fn fetch_latest_s3_zone_date(
//     client: &aws_sdk_s3::Client,
//     bucket_name: &str,
// ) -> Result<Option<String>> {
//     let files = get_all_files_from_s3_bucket(client, bucket_name).await?;
//
//     // TODO: use a regex match instead of ends_with
//     let last_file = files
//         .into_iter()
//         .filter(|s| s.ends_with(S3_FILE_EX))
//         .sorted()
//         .last();
//     if let Some(s) = last_file {
//         let date_str = &s[0..(s.len() - S3_FILE_EX.len())];
//         return Ok(Some(date_str.into()));
//     }
//     Ok(None)
// }

fn get_file_date_from_header(response: &reqwest::Response) -> Result<String> {
    let headers = response.headers();
    let last_modified_orig = headers
        .get("last-modified")
        .context("Missing header: 'last-modified'")?
        .to_str()?;
    let last_icann_date = DateTime::parse_from_rfc2822(last_modified_orig)?
        .format(DATE_FORMAT)
        .to_string();
    println!("Zone file date {last_icann_date:?}");
    Ok(last_icann_date)
}

async fn download_zone_file(access_token: &str) -> Result<()> {
    let mut response =
        send_request(ZONE_FILE_URL, Some(access_token), Method::GET, &json!({})).await?;
    let last_icann_date = get_file_date_from_header(&response)?;
    //
    // if let Some(last_processed_date) = latest {
    //     if last_processed_date >= last_icann_date.as_str() {
    //         return Err(anyhow!("ICANN zone file ({last_icann_date}) older than last processed ({last_processed_date})."));
    //     }
    // }
    //
    // if let Err(err) = std::fs::create_dir("db") {
    //     if err.kind() != ErrorKind::AlreadyExists {
    //         return Err(anyhow!("Can't create db directory: {err:?}"));
    //     }
    // }
    //
    let path = format!("./db/com.zone.{last_icann_date}.txt.gz");
    // process_zone_file(response, &path).await?;
    let total_size = response.content_length().context(format!(
        "Failed to get content length from '{}'",
        ZONE_FILE_URL
    ))?;

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("#>-"));
    pb.set_message(format!("Downloading {}", path));

    let mut target_file = tokio::fs::File::create(format!("{path}")).await?;
    let mut reader = response_to_reader(response);
    let mut buf_reader = tokio::io::BufReader::new(reader);
    let mut sum = 0;
    let mut observer = ObservedReader::new(buf_reader, |buf| {
        // println!("Buf size: {}", buf.filled().len());
        let last = sum / 10_000_000;
        sum += buf.filled().len() as u64;
        if sum / 10_000_000 != last {
            pb.set_position(sum);
        }
    });
    tokio::io::copy(&mut observer, &mut target_file).await?;
    pb.finish_with_message(format!("Downloaded {}", path));

    Ok(())
}

fn download_zone_file2(access_token: &str) -> Result<()> {
    let mut response =
        send_request_blocking(ZONE_FILE_URL, Some(access_token), Method::GET, &json!({}))?;
    // let total_size = response.content_length().context(format!(
    //     "Failed to get content length from '{}'",
    //     ZONE_FILE_URL
    // ))?;
    let length = response.content_length().context("Response length error")?;
    let last_icann_date = get_file_date_from_header(&response)?;


    let pb = ProgressBar::new(length);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("#>-"));
    pb.set_message(format!("Downloading {}", path));

    let path = format!("./db/com.zone.{last_icann_date}.txt.gz");
    let mut target_file = std::fs::File::create(path)?;

    // let mut target_file = tokio::fs::File::create(format!("{path}")).await?;
    // let mut reader = response_to_reader(response);
    // let mut buf_reader = tokio::io::BufReader::new(reader);
    // let mut sum = 0;
    // let mut observer = ObservedReader::new(buf_reader, |buf| {
    //     // println!("Buf size: {}", buf.filled().len());
    //     let last = sum / 10_000_000;
    //     sum += buf.filled().len() as u64;
    //     if sum / 10_000_000 != last {
    //         pb.set_position(sum);
    //     }
    // });
    std::io::copy(&mut pb.wrap_read(response), &mut target_file)?;
    // tokio::io::copy(&mut observer, &mut target_file).await?;
    pb.finish_with_message(format!("Downloaded {}", path));
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting zone file downloader.");
    let _ = dotenv();

    let icann_username = get_env("ICANN_USERNAME")?;
    let icann_password = get_env("ICANN_PASSWORD")?;
    let token = fetch_access_token(&icann_username, &icann_password).await?;
    println!("TOKEN: {token}");
    download_zone_file2(&token)?;
    println!("DOne.");
    Ok(())
}
