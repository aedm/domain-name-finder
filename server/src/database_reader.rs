use anyhow::Result;
use flate2::read::GzDecoder;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::time::Instant;

pub type Database = HashSet<String>;

pub fn read_lines(input_file_name: &str) -> Result<Database> {
    let gz_file = File::open(input_file_name)?;
    let gz_decoder = GzDecoder::new(gz_file);
    let reader = BufReader::new(gz_decoder);

    let mut set = HashSet::new();
    let mut counter = 0;
    for line in reader.lines() {
        set.insert(line.unwrap());
        counter += 1;
        if counter % 1_000_000 == 0 {
            println!("{} million entries loaded.", counter / 1_000_000);
        }
    }
    println!("Total entries: {}", counter);
    Ok(set)
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
