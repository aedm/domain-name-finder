use crate::{for_each_domain_length, Database};
use anyhow::{anyhow, Context, Result};
use flate2::read::GzDecoder;
use regex::Regex;
use seq_macro::seq;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use tokio::sync::mpsc::{channel, Receiver, Sender};

const CHANNEL_BATCH_SIZE: usize = 50_000;

async fn read_input_from_file(sender: Sender<Vec<String>>, input_file_path: String) -> Result<()> {
    println!("Reading from {}...", input_file_path);
    let reader = BufReader::new(GzDecoder::new(File::open(input_file_path)?));
    let mut batch = Vec::with_capacity(CHANNEL_BATCH_SIZE);
    let mut counter = 0;
    for line in reader.lines() {
        batch.push(line?);
        if batch.len() == CHANNEL_BATCH_SIZE {
            sender.send(batch).await?;
            batch = Vec::with_capacity(CHANNEL_BATCH_SIZE);
        }
        counter += 1;
        if counter % 10_000_000 == 0 {
            println!("{} million entries loaded.", counter / 1_000_000);
        }
    }
    sender.send(batch).await?;
    println!("Total entry count: {counter}");
    Ok(())
}

async fn distribute(
    mut recv: Receiver<Vec<String>>,
    senders: Vec<Sender<Vec<String>>>,
) -> Result<()> {
    while let Some(lines) = recv.recv().await {
        let mut batches = vec![Vec::with_capacity(CHANNEL_BATCH_SIZE); 65];
        for line in lines {
            if line.len() > 64 {
                println!("Line too long: {line}");
                continue;
            }
            batches[line.len()].push(line);
        }
        for (i, v) in batches.into_iter().enumerate() {
            if v.len() > 0 {
                senders[i].send(v).await?;
            }
        }
    }
    Ok(())
}

async fn build_hash_set<const N: usize>(mut recv: Receiver<Vec<String>>) -> HashSet<[u8; N]> {
    let mut set = HashSet::<[u8; N]>::new();
    while let Some(lines) = recv.recv().await {
        lines.into_iter().for_each(|line| {
            set.insert(line.as_bytes()[0..N].try_into().unwrap());
        });
    }
    set
}

fn find_database_file_path() -> Result<String> {
    let pattern: Regex = Regex::new(r"/com.zone.\d+-\d+.txt.gz$")?;
    for entry in std::fs::read_dir("./db")? {
        let path = entry?.path();
        println!("Entry: {path:?}");
        if let Some(path) = path.to_str() {
            if pattern.is_match(path) {
                return Ok(path.to_string());
            }
        }
    }
    Err(anyhow!("Database file not found."))
}

pub async fn read_database() -> Result<Database> {
    println!("Reading database...");

    let db_file = find_database_file_path()?;
    println!("Database file found: {db_file}");

    let now = Instant::now();

    let (input_to_distributor_sender, input_to_distributor_recv) = channel::<Vec<String>>(1_000);
    let mut distributor_to_builder_senders = vec![];
    for_each_domain_length!({
        let (sender, distributor_to_builder_receiver) = channel::<Vec<String>>(1_000);
        distributor_to_builder_senders.push(sender);
        let hashset_builder_task_~N = tokio::spawn(async move {
            build_hash_set::<N>(distributor_to_builder_receiver).await
        });
    });

    let file_reader_task =
        tokio::spawn(
            async move { read_input_from_file(input_to_distributor_sender, db_file).await },
        );
    let distributor_task = tokio::spawn(async move {
        distribute(input_to_distributor_recv, distributor_to_builder_senders).await
    });

    // Await all tasks before asserting on their success
    let distributor_result = distributor_task.await?;
    let reader_result = file_reader_task.await?;

    for_each_domain_length!({
        let db = Database {
            #(
                words_~N: hashset_builder_task_~N.await?,
            )*
        };
    });

    // Assert tasks' success
    reader_result?;
    distributor_result?;

    println!("Input read in {:.2?} sec.", now.elapsed());

    Ok(db)
}
