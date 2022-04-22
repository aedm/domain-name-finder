use anyhow::Result;
use async_compression::tokio::bufread::GzipDecoder;
use async_compression::tokio::write::GzipEncoder;
use futures::stream::TryStreamExt;
use itertools::Itertools;
use std::thread;
use std::time::Instant;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::mpsc::{channel, Receiver, Sender};

type Message = Vec<String>;

const CAP: usize = 10_000;

// returns output file name
async fn read_input(tx: Sender<Message>, input: impl tokio::io::AsyncRead + Unpin) -> Result<()> {
    println!("Reader task started.");
    let reader = BufReader::new(input);
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
    println!("Filter task started.");
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
            v.push(domain);
            counter += 1;
            if counter % 1_000_000 == 0 {
                println!("Processed {} million entries.", counter / 1_000_000);
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
    println!("Writing to '{}'...", output_file_name);
    let target_file = tokio::fs::File::create(output_file_name).await?;
    let mut writer = GzipEncoder::new(target_file);

    while let Some(lines) = rx.recv().await {
        writer.write(lines.as_bytes()).await?;
    }
    writer.shutdown().await?;
    Ok(())
}

pub async fn process_zone_file(response: reqwest::Response, output_file_name: &str) -> Result<()> {
    let stream = response
        .bytes_stream()
        .map_err(|e| futures::io::Error::new(futures::io::ErrorKind::Other, e));
    let stream_reader = tokio_util::io::StreamReader::new(stream);
    let gzip_decoder = GzipDecoder::new(stream_reader);

    let (input_to_filter_sender, input_to_filter_recv) = channel::<Message>(100);
    let (filter_to_output_sender, filter_to_output_recv) = channel::<String>(100);

    let now = Instant::now();

    let reader_task =
        tokio::spawn(async { read_input(input_to_filter_sender, gzip_decoder).await });
    let filter_task =
        tokio::spawn(async { filter_lines(input_to_filter_recv, filter_to_output_sender).await });
    let writer_task = {
        let path_string = output_file_name.to_string();
        tokio::spawn(async move { write_output(filter_to_output_recv, &path_string).await })
    };

    let reader_result = reader_task.await?;
    let filter_result = filter_task.await?;
    let writer_result = writer_task.await?;

    reader_result?;
    filter_result?;
    writer_result?;

    let elapsed = now.elapsed().as_micros();
    println!("Runtime: {} sec", elapsed as f64 / 1000000.0);
    println!("Download finished: {output_file_name}");
    Ok(())
}
