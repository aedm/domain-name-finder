use crate::Database;
use anyhow::Result;
use flate2::read::GzDecoder;
use seq_macro::seq;
use std::collections::{BTreeSet, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use xz2::read::XzDecoder;

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

pub fn read_database() -> Result<Database> {
    println!("Reading database...");
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
