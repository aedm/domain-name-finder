use crate::Database;
use anyhow::Result;
use flate2::read::GzDecoder;
use seq_macro::seq;
use std::collections::{BTreeSet, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use xz2::read::XzDecoder;

const INPUT_FILE_PATH: &str = "com.zone.filtered.txt.gz";
const CHANNEL_BATCH_SIZE: usize = 50_000;

async fn read_input_from_file(sender: Sender<Vec<String>>) -> Result<()> {
    println!("Reading from {}...", INPUT_FILE_PATH);
    let reader = BufReader::new(GzDecoder::new(File::open(INPUT_FILE_PATH)?));
    let mut batch = Vec::with_capacity(CHANNEL_BATCH_SIZE);
    for (counter, line) in reader.lines().enumerate() {
        batch.push(line?);
        if batch.len() == CHANNEL_BATCH_SIZE {
            sender.send(batch).await?;
            batch = Vec::with_capacity(CHANNEL_BATCH_SIZE);
        }
        if counter % 10_000_000 == 0 {
            println!("{} million entries loaded.", counter / 1_000_000);
        }
    }
    sender.send(batch).await?;
    Ok(())
}

fn read_input_from_file_blocking(sender: Sender<Vec<String>>) -> Result<()> {
    println!("Reading from {}...", INPUT_FILE_PATH);
    let reader = BufReader::new(GzDecoder::new(File::open(INPUT_FILE_PATH)?));
    let mut batch = Vec::with_capacity(CHANNEL_BATCH_SIZE);
    for (counter, line) in reader.lines().enumerate() {
        batch.push(line?);
        if batch.len() == CHANNEL_BATCH_SIZE {
            sender.blocking_send(batch)?;
            batch = Vec::with_capacity(CHANNEL_BATCH_SIZE);
        }
        if counter % 10_000_000 == 0 {
            println!("{} million entries loaded.", counter / 1_000_000);
        }
    }
    sender.blocking_send(batch)?;
    Ok(())
}

async fn distribute(
    mut recv: Receiver<Vec<String>>,
    senders: Vec<Sender<Vec<String>>>,
) -> Result<()> {
    while let Some(lines) = recv.recv().await {
        let mut batches = vec![Vec::with_capacity(CHANNEL_BATCH_SIZE); 64];
        for line in lines {
            batches[line.len()].push(line);
        }
        for (i, v) in batches.into_iter().enumerate() {
            if v.len() > 0 {
                senders[i - 1].send(v).await?;
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

pub async fn read_database() -> Result<Database> {
    println!("Reading database...");

    let now = Instant::now();

    let (input_to_distributor_sender, mut input_to_distributor_recv) =
        channel::<Vec<String>>(1_000);
    let mut distributor_to_builder_senders = vec![];
    seq!(N in 1..64 {
        let (sender, mut distributor_to_builder_receiver) = channel::<Vec<String>>(1_000);
        distributor_to_builder_senders.push(sender);
        let hashset_builder_task_~N = tokio::spawn(async move {
            build_hash_set::<N>(distributor_to_builder_receiver).await
        });
    });

    let file_reader_task =
        tokio::spawn(async move { read_input_from_file(input_to_distributor_sender).await });
    let distributor_task = tokio::spawn(async move {
        distribute(input_to_distributor_recv, distributor_to_builder_senders).await
    });

    // Await all tasks before asserting on their success
    seq!(N in 1..64 {
        let db = Database {
            #(
                words_~N: hashset_builder_task_~N.await?,
            )*
        };
    });
    let distributor_result = distributor_task.await?;
    let reader_result = file_reader_task.await?;

    // Assert tasks' success
    reader_result?;
    distributor_result?;

    println!("Input read in {:.2?} sec.", now.elapsed());

    Ok(db)
}
