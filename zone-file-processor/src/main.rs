use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::time::Instant;

// returns output file name
fn process_lines(input_file_name: &str) -> Result<String> {
    let gz_file = File::open(input_file_name)?;
    let gz_decoder = GzDecoder::new(gz_file);
    let header = gz_decoder
        .header()
        .with_context(|| "Error reading gz header")?;
    let original_file_name_bytes = header
        .filename()
        .with_context(|| "gz header doesn't contain a file name")?;
    let original_file_name = String::from_utf8_lossy(original_file_name_bytes);
    let output_file_name = format!("{}.filtered.txt.gz", original_file_name);
    println!("Writing to '{}'...", output_file_name);

    let reader = BufReader::new(gz_decoder);
    let target_file = File::create(&output_file_name)?;
    let mut writer = GzEncoder::new(target_file, Compression::fast());

    let mut last_domain = String::new();
    let mut counter = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let domain = line.split('.').nth(0).expect("Error in input");
        if last_domain != domain {
            writeln!(writer, "{}", domain)?;
            last_domain = String::from(domain);
            counter += 1;
            if counter % 1_000_000 == 0 {
                println!("Processed {} million entries.", counter / 1_000_000);
                break;
            }
        }
    }
    println!("Total entries: {}", counter);
    Ok(output_file_name)
}

fn main() -> Result<()> {
    let now = Instant::now();
    let output_file_name = process_lines("com.txt.gz")?;
    let elapsed = now.elapsed().as_micros();
    println!("Runtime: {} sec", elapsed as f64 / 1000000.0);
    Ok(())
}
