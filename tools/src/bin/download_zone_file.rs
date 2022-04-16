extern crate dotenv;

use anyhow::{anyhow, Context, Result};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;
use chrono::DateTime;
use dotenv::dotenv;
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::cmp::min;
use std::fs::File;
use std::io::Write;
use tools::process_zone_file::process_zone_file;
use tools::{fetch_json, get_all_files_from_s3_bucket, get_env, make_aws_s3_client, send_request};

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

async fn fetch_latest_s3_zone_date(
    client: &aws_sdk_s3::Client,
    bucket_name: &str,
) -> Result<Option<String>> {
    println!("fetch_latest_s3_zone_date");
    let files = get_all_files_from_s3_bucket(client, bucket_name).await?;

    // TODO: use a regex match instead of ends_with
    let last_file = files
        .into_iter()
        .filter(|s| s.ends_with(S3_FILE_EX))
        .sorted()
        .last();
    if let Some(s) = last_file {
        let date_str = &s[0..(s.len() - S3_FILE_EX.len())];
        return Ok(Some(date_str.into()));
    }
    Ok(None)
}

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

async fn download_zone_file(access_token: &str, latest: Option<&str>) -> Result<()> {
    let response = send_request(ZONE_FILE_URL, Some(access_token), Method::GET, &json!({})).await?;
    // let response = send_request(
    //     "http://localhost:8000/com.txt.gz",
    //     Some(access_token),
    //     Method::GET,
    //     &json!({}),
    // )
    // .await?;
    let last_icann_date = get_file_date_from_header(&response)?;

    if let Some(last_processed_date) = latest {
        if last_processed_date >= last_icann_date.as_str() {
            return Err(anyhow!("ICANN zone file ({last_icann_date}) older than last processed ({last_processed_date})."));
        }
    }

    let path = format!("com.zone.{last_icann_date}.txt.gz");

    // download_stream_to_file(response, &path).await?;
    process_zone_file(response, &path).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv();

    let icann_username = get_env("ICANN_USERNAME")?;
    let icann_password = get_env("ICANN_PASSWORD")?;
    let token = fetch_access_token(&icann_username, &icann_password).await?;
    // println!("Access token: {token}");

    // let token = "eyJraWQiOiJFczQ4dzhadTZISjhxd2F1M3M1bjJhMUUtSFN1Tk5PbW00Tl9oU0JwYW1RIiwiYWxnIjoiUlMyNTYifQ.eyJ2ZXIiOjEsImp0aSI6IkFULmxtdkt3MlVJS0FtOHV0eERjbjVEZWpBN1JfTFRvcW1UUWRpdzM1djU3aHMiLCJpc3MiOiJodHRwczovL2ljYW5uLWFjY291bnQub2t0YS5jb20vb2F1dGgyL2F1czJwMDFjMnJvSkFlQ2dZMnA3IiwiYXVkIjoiaHR0cDovL2FwaV9hdXRoZW5yaXphdGlvbl9zZXJ2ZXIuaWNhbm4ub3JnIiwiaWF0IjoxNjQ5NTA1NDQ1LCJleHAiOjE2NDk1OTE4NDUsImNpZCI6IjBvYTFyY2prcWtPbGlNUHVMMnA3IiwidWlkIjoiMDB1ZWhrNGw4a21oRzZVN2EycDciLCJzY3AiOlsiaWNhbm4tY3VzdG9tIiwib3BlbmlkIl0sImF1dGhfdGltZSI6MTY0OTUwNTQ0NSwic3ViIjoia29ydGV1ckBnbWFpbC5jb20iLCJnaXZlbl9uYW1lIjoiR8OhYm9yIiwiZmFtaWx5X25hbWUiOiJHeWVibsOhciIsImVtYWlsIjoia29ydGV1ckBnbWFpbC5jb20ifQ.X4L5cJ93JvFTbYREmV10or30xnhMchKXsgxZsUN7RwPneUGd3KCrKIO8K4v1S1KjJh8YHGDN9ii1_4Pm--qL211bCU1BsmYOblX-pXxoxNxIxLx3Y3X5Zn6RsH8SZnCG8GjJjey5b8vK4CND6805QrjGDRilE0fSSESMX3YFOzi1NsCi3Bed-aLFGsVMxJqe87Pqh28OookE6Av1L9mE3EoAs5tNjRpwrA-wbKnjsM9GmtODgY5r3_spXXYLGme3vId6tSUVjm8Z_MWzLMWyEyOlMGQpnuf6BVEUEMfbjZILQOdd89NSJNDP23oe64oA9zlVvsKrDxpIdtiEYuIN1w";
    // fetch_headers(token).await?;

    let client = make_aws_s3_client().await;
    let s3_bucket = get_env("S3_BUCKET_NAME")?;

    let latest = fetch_latest_s3_zone_date(&client, &s3_bucket).await?;
    let latest_ref = latest.as_ref().and_then(|s| Some(s.as_str()));

    download_zone_file(&token, latest_ref).await?;
    println!("{latest:?}");
    Ok(())
}
