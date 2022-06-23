extern crate dotenv;

use anyhow::{Context, Result};
use chrono::DateTime;
use dotenv::dotenv;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{env, fs};
use tools::{fetch_json, get_env, send_request};

const AUTH_URL: &str = &"https://account-api.icann.org/api/authenticate";
const ZONE_FILE_URL: &str = &"https://czds-api.icann.org/czds/downloads/com.zone";
const DATE_FORMAT: &str = "%Y%m%d-%H%M%S";

fn fetch_access_token(username: &str, password: &str) -> Result<String> {
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

    let response: AuthResponse = fetch_json(AUTH_URL, None, &AuthRequest { username, password })?;
    Ok(response.access_token)
}

fn get_file_date_from_header(response: &reqwest::blocking::Response) -> Result<String> {
    let headers = response.headers();
    let last_modified_orig = headers
        .get("last-modified")
        .context("Missing header: 'last-modified'")?
        .to_str()?;
    let last_icann_date = DateTime::parse_from_rfc2822(last_modified_orig)?
        .format(DATE_FORMAT)
        .to_string();
    Ok(last_icann_date)
}

fn download_zone_file(access_token: &str, date_only: bool) -> Result<()> {
    let response = send_request(ZONE_FILE_URL, Some(access_token), Method::GET, &json!({}))?;
    let length = response.content_length().context("Response length error")?;
    let last_icann_date = get_file_date_from_header(&response)?;

    if date_only {
        println!("{last_icann_date}");
        return Ok(());
    }
    println!("Zone file date {last_icann_date}");

    let pb = ProgressBar::new(length);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("#>-"));
    pb.set_message(format!("Downloading {}", ZONE_FILE_URL));

    let dir = "./db/zone-file";
    fs::create_dir_all(&dir).unwrap();
    let path = format!("{dir}/com-zone-raw.{last_icann_date}.txt.gz");
    let mut target_file = std::fs::File::create(&path)?;
    std::io::copy(&mut pb.wrap_read(response), &mut target_file)?;
    pb.finish_with_message(format!("Downloaded to {}", path));
    Ok(())
}

fn get_token() -> Result<String> {
    let icann_username = get_env("ICANN_USERNAME")?;
    let icann_password = get_env("ICANN_PASSWORD")?;
    fetch_access_token(&icann_username, &icann_password)
}

fn main() -> Result<()> {
    let _ = dotenv();

    if env::args().any(|a| a == "--only-date") {
        download_zone_file(&get_token()?, true)?;
        return Ok(());
    }

    println!("Starting zone file downloader.");
    download_zone_file(&get_token()?, false)?;
    println!("Done.");
    Ok(())
}
