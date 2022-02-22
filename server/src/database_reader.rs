use crate::Database;
use anyhow::Result;
use flate2::read::GzDecoder;
use seq_macro::seq;
use std::collections::{BTreeSet, HashSet};
use std::fs::File;
use std::futures;
use std::io;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use xz2::read::XzDecoder;

async fn read_input_from_file(sender: mpsc::Sender<Vec<String>>) -> Result<()> {
    let path = "com.zone.filtered.txt.gz";
    println!("Reading from {}...", path);

    let gz_file = File::open(path)?;
    let gz_decoder = GzDecoder::new(gz_file);
    let reader = BufReader::new(gz_decoder);

    const CAP: usize = 10_000;
    let mut v = Vec::with_capacity(CAP);
    for line in reader.lines() {
        v.push(line?);
        if v.len() == CAP {
            sender.send(v)?;
            v = Vec::with_capacity(CAP);
        }
    }
    sender.send(v)?;
    Ok(())
}

async fn distribute(
    recv: mpsc::Receiver<Vec<String>>,
    senders: Vec<mpsc::Sender<Vec<String>>>,
) -> Result<()> {
    while let Some(lines) = recv.recv().await {}
    Ok(())
}

async fn build_hash_set<const N: usize>(recv: mpsc::Receiver<Vec<String>>) -> HashSet<[u8; N]> {
    let mut set = HashSet::new();

    set
}

fn read_lines(input_file_name: &str) -> Result<Database> {
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

    let (input_to_filter_sender, mut input_to_filter_recv) = mpsc::channel::<Vec<String>>();

    actix_web::rt::spawn(async move {
        read_input_from_file(input_to_filter_sender).await;
    });

    let now = Instant::now();
    let mut db = read_lines("com.zone.filtered.txt.gz");
    // let mut set = read_lines("com.zone.46792.filtered.txt.gz")?;
    let elapsed = now.elapsed().as_micros();
    println!("Input read in {} sec.", elapsed as f64 / 1000000.0);

    println!("Press enter");
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut buffer)?;

    db
}
