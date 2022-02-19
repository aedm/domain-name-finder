use crate::database::{LETTER_INDEX, VALID_LETTERS_COUNT};
use crate::Database;
use anyhow::Result;
use flate2::read::GzDecoder;
use smol_str::SmolStr;
use std::collections::{BTreeSet, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use xz2::read::XzDecoder;

// pub type DbEntry = SmolStr;
// pub type Database = HashSet<DbEntry>;

fn read_lines(input_file_name: &str) -> Result<Database> {
    let gz_file = File::open(input_file_name)?;
    let gz_decoder = GzDecoder::new(gz_file);
    let reader = BufReader::new(gz_decoder);

    let mut words =
        vec![
            HashSet::<SmolStr>::new();
            VALID_LETTERS_COUNT * VALID_LETTERS_COUNT * VALID_LETTERS_COUNT * VALID_LETTERS_COUNT
        ];

    let mut counter = 0;
    for line in reader.lines() {
        let line = line?;
        if line.len() < 3 {
            continue;
        }

        let b = line.as_bytes();
        let index = (LETTER_INDEX[b[0] as usize] * VALID_LETTERS_COUNT
            + LETTER_INDEX[b[1] as usize])
            * VALID_LETTERS_COUNT
            + LETTER_INDEX[b[2] as usize];
        words[index].insert(SmolStr::from(&line[3..]));

        counter += 1;
        if counter % 10_000_000 == 0 {
            println!("{} million entries loaded.", counter / 1_000_000);
        }
    }
    println!("Total entries: {}", counter);

    let db = Database { words };
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

// fn main() -> Result<()> {
//     let now = Instant::now();
//     let mut set = read_lines("com.zone.46792.filtered.txt.gz")?;
//     let elapsed = now.elapsed().as_micros();
//     println!("Input read in {} sec.", elapsed as f64 / 1000000.0);
//
//     println!("Press enter");
//     let mut buffer = String::new();
//     let stdin = io::stdin(); // We get `Stdin` here.
//     stdin.read_line(&mut buffer)?;
//
//     set.clear();
//
//     println!("Press enter 2");
//     let mut buffer = String::new();
//     let stdin = io::stdin(); // We get `Stdin` here.
//     stdin.read_line(&mut buffer)?;
//
//     Ok(())
// }
