use anyhow::{Context, Result};
use chrono::DateTime;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{Method, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::cmp::min;
use std::fs::File;
use std::io::Write;
use tools::{download_stream_to_file, fetch_json, get_env, send_request};

const AUTH_URL: &str = &"https://account-api.icann.org/api/authenticate";
const ZONE_FILE_URL: &str = &"https://czds-api.icann.org/czds/downloads/com.zone";

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

    let last_modified = response
        .headers()
        .get("last-modified")
        .context("Missing header: 'last-modified'")?
        .to_str()?;
    println!("last mod {last_modified:?}");

    let last_mod = DateTime::parse_from_rfc2822(last_modified)?
        .format("%Y%m%d-%H%M%S")
        .to_string();
    println!("last mod2 {last_mod:?}");

    Ok(())
    // reqwest::Client::new()
    //     .post(ZONE_FILE_URL)
    //     .header("User-Agent", "utils/0.1.0")
    //     .header("Authorization", format!("bearer {access_token}"));
}

async fn download_zone_file(access_token: &str) -> Result<()> {
    let res = send_request(ZONE_FILE_URL, Some(access_token), Method::GET, &json!({})).await?;
    let mut stream = res.bytes_stream();
    stream.next();
    download_stream_to_file(stream).await
}

#[tokio::main]
async fn main() -> Result<()> {
    // let aws = get_env("AWS_C")?;
    let icann_username = get_env("ICANN_USERNAME")?;
    let icann_password = get_env("ICANN_PASSWORD")?;
    let token = fetch_access_token(&icann_username, &icann_password).await?;
    println!("Access token: {token}");

    // let token = "eyJraWQiOiJFczQ4dzhadTZISjhxd2F1M3M1bjJhMUUtSFN1Tk5PbW00Tl9oU0JwYW1RIiwiYWxnIjoiUlMyNTYifQ.eyJ2ZXIiOjEsImp0aSI6IkFULmw2OU1ZZlhsSW5HbDVFNzJibTIxZmx3cThsRXlsbkRuQ2Zqck1YaXFSc28iLCJpc3MiOiJodHRwczovL2ljYW5uLWFjY291bnQub2t0YS5jb20vb2F1dGgyL2F1czJwMDFjMnJvSkFlQ2dZMnA3IiwiYXVkIjoiaHR0cDovL2FwaV9hdXRoZW5yaXphdGlvbl9zZXJ2ZXIuaWNhbm4ub3JnIiwiaWF0IjoxNjQ5MzM4MjMyLCJleHAiOjE2NDk0MjQ2MzIsImNpZCI6IjBvYTFyY2prcWtPbGlNUHVMMnA3IiwidWlkIjoiMDB1ZWhrNGw4a21oRzZVN2EycDciLCJzY3AiOlsiaWNhbm4tY3VzdG9tIiwib3BlbmlkIl0sImF1dGhfdGltZSI6MTY0OTMzODIzMiwic3ViIjoia29ydGV1ckBnbWFpbC5jb20iLCJnaXZlbl9uYW1lIjoiR8OhYm9yIiwiZmFtaWx5X25hbWUiOiJHeWVibsOhciIsImVtYWlsIjoia29ydGV1ckBnbWFpbC5jb20ifQ.fVxH1PbOw1ls880eJJCpLOIxVdOTsz1u4xUgUUFLc3rK6TEWO-RCFNm2rissRqIoFmcVvQ9fU1D7mTELGlYsc6YxqsqoJlvtUQR1SsavelnmlzwE0oaxYVvVn9iHmjbeVCHd6MuEksoA_-W886hEvJ8kAounbG81KehObM1cq82jwTUzEGn2uJN-bTPLeLZAcA9J6O71YqhJwrgOzS_O9bBprmnctXq65zlfwtBY468TvtF7Efpt3_XmP1D9lmNcKTTso3rhfd1xnc92F3aIb35Zc2vBuGBQppATHCKp00NqAIoC_pfNv6v25McikEpp6gPY3gc1w-uZK2KybeIBfg";
    // fetch_headers(token).await?;
    download_zone_file(&token).await?;
    Ok(())
}
