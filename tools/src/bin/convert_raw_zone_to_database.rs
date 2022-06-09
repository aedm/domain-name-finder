use anyhow::{anyhow, Context, Result};
use flate2::read::GzDecoder;
use itertools::Itertools;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn find_raw_zone_file_path() -> Result<String> {
    let pattern: Regex = Regex::new(r"/com-zone-raw.\d+-\d+.txt.gz$")?;
    let mut entries = std::fs::read_dir("./db")?
        .flatten()
        .map(|entry| entry.path().to_str().map(str::to_string))
        .flatten()
        .filter(|path| pattern.is_match(path))
        .collect_vec();
    match entries.len() {
        0 => Err(anyhow!("Database file not found.")),
        1 => Ok(entries[0].clone()),
        _ => Err(anyhow!("Too many database files found.")),
    }
}

fn read_zone_file(path: &str) -> Result<Vec<Vec<String>>> {
    let mut result: Vec<Vec<String>> = vec![vec![]; 65];

    let reader = BufReader::new(GzDecoder::new(File::open(path)?));
    let mut counter = 0;
    for line in reader.lines() {
        let line = line?;
        let index = line
            .find('.')
            .with_context(|| anyhow!("Can't parse input line: '{line}'"))?;
        let domain = &line[0..index];
        let buf = &mut result[domain.len()];
        if buf.last().map(String::as_str) != Some(domain) {
            buf.push(domain.to_string());
            counter += 1;
            if counter % 10_000_000 == 0 {
                println!("Read {} million entries.", counter / 1_000_000);
            }
        }
    }
    println!("Finished reading {counter} entries.");
    for i in 1..result.len() {
        println!("Length {} -> count: {}", i, result[i].len());
    }

    Ok(result)
}

fn write_result_file(db: Vec<Vec<String>>) -> Result<()> {
    for i in 1..=64 {
        let file_name = format!("./db/part-{i}.zst");
        print!("Writing file {file_name}...");
        let mut compressed = db[i].join("\n");
        println!(" {} bytes", compressed.len());
        let file = File::create(file_name)?;
        let mut encoder = zstd::stream::Encoder::new(file, 21).unwrap();
        encoder.write_all(compressed.as_bytes())?;
        encoder.finish()?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let raw_zone_file = find_raw_zone_file_path()?;
    println!("Processing raw zone file '{raw_zone_file}'");
    let db = read_zone_file(&raw_zone_file)?;
    write_result_file(db)?;
    Ok(())
}
