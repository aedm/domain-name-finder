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

async fn read_input_from_file(sender: Sender<Vec<String>>) -> Result<()> {
    let path = "com.zone.filtered.txt.gz";
    println!("Reading from {}...", path);

    let gz_file = File::open(path)?;
    let gz_decoder = GzDecoder::new(gz_file);
    let reader = BufReader::new(gz_decoder);

    let mut counter = 0;

    const CAP: usize = 10_000;
    let mut v = Vec::with_capacity(CAP);
    for line in reader.lines() {
        v.push(line?);
        if v.len() == CAP {
            sender.send(v).await?;
            v = Vec::with_capacity(CAP);
        }
        counter += 1;
        if counter % 10_000_000 == 0 {
            println!("{} million entries loaded.", counter / 1_000_000);
        }
    }
    sender.send(v).await?;
    Ok(())
}

async fn distribute(
    mut recv: Receiver<Vec<String>>,
    senders: Vec<Sender<Vec<String>>>,
) -> Result<()> {
    const CAP: usize = 1_000;
    let mut batches = vec![Vec::new(); 64];
    while let Some(lines) = recv.recv().await {
        for line in lines {
            batches[line.len()].push(line);
        }
        for i in 1..64 {
            if batches[i].len() >= CAP {
                senders[i - 1]
                    .send(std::mem::replace(&mut batches[i], Vec::new()))
                    .await?;
            }
        }
    }
    for i in 1..64 {
        if batches[i].len() > 0 {
            senders[i - 1]
                .send(std::mem::replace(&mut batches[i], Vec::new()))
                .await?;
        }
    }
    Ok(())
}

async fn build_hash_set<const N: usize>(mut recv: Receiver<Vec<String>>) -> HashSet<[u8; N]> {
    println!("Hashset builder {}", N);
    let mut set = HashSet::<[u8; N]>::new();
    while let Some(lines) = recv.recv().await {
        for line in lines {
            let array: &[u8; N] = &line.as_bytes()[0..N].try_into().unwrap();
            set.insert(*array);
        }
    }
    set
}

fn read_file_single_step() -> Result<Database> {
    let input_file_name = "com.zone.filtered.txt.gz";
    let gz_file = File::open(input_file_name)?;
    let gz_decoder = GzDecoder::new(gz_file);
    let reader = BufReader::new(gz_decoder);

    let mut db = Database::new();

    let mut counter = 0;
    for line in reader.lines() {
        let line = line?;
        seq!(N in 1..64 {
            match line.len() {
                #(
                     N => {
                        let array: &[u8; N] = &line.as_bytes()[0..N].try_into().unwrap();
                        db.words_~N.insert(*array);
                     },
                )*
                _ => panic!("Invalid length"),
            }
        });

        counter += 1;
        if counter % 10_000_000 == 0 {
            println!("{} million entries loaded.", counter / 1_000_000);
        }
    }
    println!("Total entries: {}", counter);
    Ok(db)
}

pub async fn read_database() -> Result<Database> {
    println!("Reading database...");

    let now = Instant::now();

    let (input_to_filter_sender, mut input_to_filter_recv) = channel::<Vec<String>>(1_000);
    let mut length_senders = vec![];
    seq!(N in 1..64 {
        let (sender, mut length_receiver_~N) = channel::<Vec<String>>(1_000);
        length_senders.push(sender);
    });

    let reader_task =
        tokio::spawn(async move { read_input_from_file(input_to_filter_sender).await });
    let distribute_task =
        tokio::spawn(async move { distribute(input_to_filter_recv, length_senders).await });
    seq!(N in 1..64 {
       let hashset_builder_task_~N = tokio::spawn(async move { build_hash_set::<N>(length_receiver_~N).await });
    });

    reader_task.await?;
    distribute_task.await?;

    seq!(N in 1..64 {
       let words_~N = hashset_builder_task_~N.await?;
    });

    // let db = read_file_single_step();

    let elapsed = now.elapsed().as_micros();
    println!("Input read in {} sec.", elapsed as f64 / 1000000.0);

    println!("Press enter");
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut buffer)?;

    Ok(Database::new())
}
