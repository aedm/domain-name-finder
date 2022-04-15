use anyhow::{Context, Result};
use async_compression::tokio::bufread::GzipDecoder;
use async_compression::tokio::write::GzipEncoder;
use futures::stream::TryStreamExt;
use itertools::Itertools;
use std::fs::File;
use std::thread;
use std::time::Instant;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio_util::compat::FuturesAsyncReadCompatExt;
use tokio_util::io::StreamReader;

type Message = Vec<String>;

const CAP: usize = 10_000;

// returns output file name
async fn read_input<T: tokio::io::AsyncRead + Unpin>(
    tx: Sender<Message>,
    reader: BufReader<T>,
) -> Result<()> {
    // let reader = BufReader::new(gz_decoder);
    println!("Reader");
    let mut v = Vec::with_capacity(CAP);
    let mut lines = reader.lines();
    while let Some(line) = lines.next_line().await? {
        v.push(line);
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
            let msg = v.iter().join("\n");
            filter_to_output_sender.send(format!("{msg}\n")).await?;
        }
    }
    println!("Total entries: {counter}");
    Ok(())
}

async fn write_output(mut rx: Receiver<String>, output_file_name: &str) -> Result<()> {
    // let output_file_name = format!("{}.filtered.txt.gz", original_file_name);
    println!("Writing to '{}'...", output_file_name);
    let target_file = tokio::fs::File::create(output_file_name).await?;
    let mut writer = GzipEncoder::new(target_file);

    while let Some(lines) = rx.recv().await {
        writer.write(lines.as_bytes()).await?;
    }
    Ok(())
}

pub async fn process_zone_file(response: reqwest::Response, output_file_name: &str) -> Result<()> {
    let stream = response
        .bytes_stream()
        .map_err(|e| futures::io::Error::new(futures::io::ErrorKind::Other, e))
        .into_async_read()
        .compat();
    let gzip_decoder = GzipDecoder::new(stream);
    let buf_reader = tokio::io::BufReader::new(gzip_decoder);

    let (input_to_filter_sender, mut input_to_filter_recv) = channel::<Message>(1_000);
    let (filter_to_output_sender, mut filter_to_output_recv) = channel::<String>(1_000);

    let now = Instant::now();

    let reader_task = tokio::spawn(async { read_input(input_to_filter_sender, buf_reader).await });
    let filter_task =
        tokio::spawn(async { filter_lines(input_to_filter_recv, filter_to_output_sender).await });
    let output_file_name = output_file_name.to_string();
    let writer_task =
        tokio::spawn(async move { write_output(filter_to_output_recv, &output_file_name).await });

    reader_task.await?;
    filter_task.await?;
    writer_task.await?;

    let elapsed = now.elapsed().as_micros();
    println!("Runtime: {} sec", elapsed as f64 / 1000000.0);
    Ok(())
}
