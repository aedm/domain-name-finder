extern crate dotenv;

use anyhow::{Context, Result};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;
use chrono::DateTime;
use dotenv::dotenv;
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use reqwest::{Method, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::cmp::min;
use std::fs::File;
use std::io::Write;
use tools::{download_stream_to_file, fetch_json, get_env, send_request};

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

async fn fetch_headers(access_token: &str) -> Result<()> {
    let response =
        send_request(ZONE_FILE_URL, Some(access_token), Method::HEAD, &json!({})).await?;
    println!("{response:#?}");

    let headers = response.headers();
    let last_modified = headers
        .get("last-modified")
        .context("Missing header: 'last-modified'")?
        .to_str()?;
    println!("last mod {last_modified:?}");

    let last_mod = DateTime::parse_from_rfc2822(last_modified)?
        .format(DATE_FORMAT)
        .to_string();
    println!("last mod2 {last_mod:?}");

    Ok(())
}

async fn get_all_files_from_s3_bucket(bucket_name: &str) -> Result<Vec<String>> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let objects = client.list_objects_v2().bucket(bucket_name).send().await?;
    let contents = objects.contents.unwrap_or_default();
    Ok(contents.into_iter().map(|i| i.key).flatten().collect_vec())
}

async fn get_latest_s3_zone_date(bucket_name: &str) -> Result<Option<String>> {
    let files = get_all_files_from_s3_bucket(bucket_name).await?;

    // TODO: use a regex match instead of ends_with
    let last_file = files
        .into_iter()
        .filter(|s| s.ends_with(S3_FILE_EX))
        .sorted()
        .last();
    if let Some(s) = last_file {
        let date_str = &s[0..(s.len() - S3_FILE_EX.len())];
        return Ok(Some(date_str.into()));
        // if let Ok(date) = DateTime::parse_from_str(date_str, DATE_FORMAT) {
        //     return Ok(Some(date))
        // }
    }
    Ok(None)
}

async fn download_zone_file(access_token: &str) -> Result<()> {
    let response = send_request(ZONE_FILE_URL, Some(access_token), Method::GET, &json!({})).await?;
    // stream.next();
    let path = "com.zone.txt.gz";
    download_stream_to_file(response, path).await
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;

    // let aws = get_env("AWS_C")?;
    // let icann_username = get_env("ICANN_USERNAME")?;
    // let icann_password = get_env("ICANN_PASSWORD")?;
    // let token = fetch_access_token(&icann_username, &icann_password).await?;
    // println!("Access token: {token}");

    // let token = "eyJraWQiOiJFczQ4dzhadTZISjhxd2F1M3M1bjJhMUUtSFN1Tk5PbW00Tl9oU0JwYW1RIiwiYWxnIjoiUlMyNTYifQ.eyJ2ZXIiOjEsImp0aSI6IkFULmlNUlJIOXVFcVJLelZJUGdvc0NkbjVUdEdzb09OQ2xRbEdNbkV5REc2Tk0iLCJpc3MiOiJodHRwczovL2ljYW5uLWFjY291bnQub2t0YS5jb20vb2F1dGgyL2F1czJwMDFjMnJvSkFlQ2dZMnA3IiwiYXVkIjoiaHR0cDovL2FwaV9hdXRoZW5yaXphdGlvbl9zZXJ2ZXIuaWNhbm4ub3JnIiwiaWF0IjoxNjQ5NDA2MDEyLCJleHAiOjE2NDk0OTI0MTIsImNpZCI6IjBvYTFyY2prcWtPbGlNUHVMMnA3IiwidWlkIjoiMDB1ZWhrNGw4a21oRzZVN2EycDciLCJzY3AiOlsib3BlbmlkIiwiaWNhbm4tY3VzdG9tIl0sImF1dGhfdGltZSI6MTY0OTQwNjAxMiwic3ViIjoia29ydGV1ckBnbWFpbC5jb20iLCJnaXZlbl9uYW1lIjoiR8OhYm9yIiwiZmFtaWx5X25hbWUiOiJHeWVibsOhciIsImVtYWlsIjoia29ydGV1ckBnbWFpbC5jb20ifQ.BTizqQ98RkT2F29mgsK_oWNaBcrOmR0oWYeTmcLegeiK2i3cBEV92Ld9J3JZI9dc_otcht4uxx5AMjvfT55H1zkIe1yyxd5mr6ZqLZe-okkFIKYawCVu_UExP35DVKT_0c1qGT9BAbz4ec5myCZtpbiVVCC10DvjGfpTLbvbQOWUovek8q6eKPX7_B0utPRWxb8RWYuJCcDRgxUJTyvpOaI0KMKcat2vzBdyslpG1652ePJKjqOOrqgbQWnYyAqXLwvfU5jIOmjUZXPTnyEitFbP6n0gtBlAklWR_3MKD1bE5ci9djTSEmlfyPWNs4tp-cQgr1Lk3CAIGt53hErBCg";
    // fetch_headers(token).await?;
    // download_zone_file(&token).await?;

    let s3_bucket = get_env("S3_BUCKET_NAME")?;

    let latest = get_latest_s3_zone_date(&s3_bucket).await?;
    println!("{latest:?}");
    Ok(())
}
