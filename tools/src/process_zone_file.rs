use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::thread;
use std::time::Instant;
use tokio::sync::mpsc::{channel, Receiver, Sender};

type Message = Vec<String>;

const CAP: usize = 10_000;

// returns output file name
async fn read_input(tx: Sender<Message>, gz_decoder: GzDecoder<File>) -> Result<()> {
    let reader = BufReader::new(gz_decoder);
    println!("Reader");
    let mut v = Vec::with_capacity(CAP);
    for line in reader.lines() {
        v.push(line?);
        if v.len() == CAP {
            tx.send(v).await?;
            v = Vec::with_capacity(CAP);
        }
    }
    tx.send(v).await?;
    Ok(())
}

async fn filter_lines(
    mut input_to_filter_recv: Receiver<Message>,
    filter_to_output_sender: Sender<String>,
) -> Result<()> {
    println!("Filter {:?}", thread::current().id());
    let mut last_domain = String::new();
    let mut counter = 0;
    while let Some(lines) = input_to_filter_recv.recv().await {
        let mut v = Vec::with_capacity(CAP);
        for line in lines.iter() {
            let index = line.find('.').expect("Error in input");
            let domain = &line[0..index];
            if (v.len() > 0 && v[v.len() - 1] == domain) || (v.len() == 0 && last_domain == domain)
            {
                continue;
            }
            // let domain = line.split('.').nth(0).expect("Error in input");
            // filter_to_output_sender.send(last_domain.clone()).await?;
            v.push(domain);
            counter += 1;
            if counter % 1_000_000 == 0 {
                println!(
                    "Processed {} million entries. {:?}",
                    counter / 1_000_000,
                    thread::current().id()
                );
            }
        }
        if v.len() > 0 {
            last_domain = String::from(v[v.len() - 1]);
            filter_to_output_sender.send(v.iter().join("\n")).await?;
        }
    }
    println!("Total entries: {}", counter);
    Ok(())
}

async fn write_output(mut rx: Receiver<String>, output_file_name: &str) -> Result<()> {
    // let output_file_name = format!("{}.filtered.txt.gz", original_file_name);
    println!("Writing to '{}'...", output_file_name);
    let target_file = File::create(&output_file_name)?;
    let mut writer = GzEncoder::new(target_file, Compression::new(7));

    while let Some(lines) = rx.recv().await {
        writeln!(writer, "{}", lines)?
    }
    Ok(())
}

pub async fn process_zone_file(response: reqwest::Response, output_file_name: &str) -> Result<()> {
    println!("main");

    // let gz_file = File::open("com.txt.gz")?;
    // let gz_decoder = GzDecoder::new(gz_file);
    let gz_decoder = GzDecoder::new(response.bytes_stream());
    // let header = gz_decoder
    //     .header()
    //     .with_context(|| "Error reading gz header")?;
    // let original_file_name_bytes = header
    //     .filename()
    //     .with_context(|| "gz header doesn't contain a file name")?;
    // let original_file_name = String::from_utf8_lossy(original_file_name_bytes).to_string();

    let (input_to_filter_sender, mut input_to_filter_recv) = channel::<Message>(1_000);
    let (filter_to_output_sender, mut filter_to_output_recv) = channel::<String>(1_000);

    let now = Instant::now();

    let reader_task = tokio::spawn(async { read_input(input_to_filter_sender, gz_decoder).await });
    let filter_task =
        tokio::spawn(async { filter_lines(input_to_filter_recv, filter_to_output_sender).await });
    let writer_task =
        tokio::spawn(async { write_output(filter_to_output_recv, output_file_name).await });

    reader_task.await?;
    filter_task.await?;
    writer_task.await?;

    let elapsed = now.elapsed().as_micros();
    println!("Runtime: {} sec", elapsed as f64 / 1000000.0);
    Ok(())
}
