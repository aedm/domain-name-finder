use crate::{for_each_domain_length, Database};
use anyhow::{anyhow, Context, Result};
use flate2::read::GzDecoder;
use regex::Regex;
use seq_macro::seq;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::Instant;
use tokio::sync::mpsc::{channel, Receiver, Sender};

const DATABASE_DIR: &'static str = "./db/processed";

fn find_database_dir() -> String {
    let pattern: Regex = Regex::new(r"/\d+-\d+$").unwrap();
    for entry in std::fs::read_dir(DATABASE_DIR).unwrap() {
        let path = entry.unwrap().path();
        println!("Entry: {path:?}");
        if let Some(path) = path.to_str() {
            if pattern.is_match(path) {
                return path.to_string();
            }
        }
    }
    panic!("Database not found");
}

fn read_input_file<const N: usize>(dir: String) -> HashSet<[u8; N]> {
    let mut set = HashSet::<[u8; N]>::new();
    if N == 0 {
        return set;
    }
    let path = format!("{}/{:0>2}.txt.zst", dir, N);
    println!("Reading '{path}'");
    let reader = BufReader::new(zstd::stream::Decoder::new(File::open(&path).unwrap()).unwrap());
    for line in reader.lines() {
        set.insert(line.unwrap().as_bytes()[0..N].try_into().unwrap());
    }
    println!("Finished reading '{path}'");
    set
}

pub fn read_database() -> Database {
    println!("Reading database...");

    let db_dir = find_database_dir();
    println!("Database directory found: {db_dir}");

    let now = Instant::now();

    for_each_domain_length!({
        #(
            let hashset_builder_task_~N = {
                let db_dir = db_dir.clone();
                thread::spawn(|| read_input_file::<N>(db_dir))
            };
        )*
    });

    for_each_domain_length!({
        let db = Database {
            #(
                words_~N: hashset_builder_task_~N.join().unwrap(),
            )*
        };
    });

    println!("Input read in {:.2?} sec.", now.elapsed());
    db
}
