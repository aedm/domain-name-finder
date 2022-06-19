use anyhow::{anyhow, Context, Result};
use crossbeam_channel::{unbounded, Receiver, Sender};
use flate2::read::GzDecoder;
use itertools::Itertools;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::time::Instant;
use std::{fs, mem, thread};

const MAX_LENGTH: usize = 64;

fn find_raw_zone_file_path() -> (String, String) {
    let pattern: Regex = Regex::new(r"/com-zone-raw.(\d+-\d+).txt.gz$").unwrap();
    let mut entries = std::fs::read_dir("./db/zone-file")
        .unwrap()
        .flatten()
        .map(|entry| entry.path().to_str().map(str::to_string))
        .flatten()
        .filter(|path| pattern.is_match(path))
        .collect_vec();
    assert_eq!(entries.len(), 1);
    let path = entries[0].clone();
    let date = pattern
        .captures(&path)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .to_string();
    (path, date)
}

fn read_zone_file(path: &str, senders: Vec<Sender<String>>) {
    let mut lines: Vec<Vec<String>> = vec![vec![]; MAX_LENGTH + 1];
    let batch_len = 1000;

    let reader = BufReader::new(GzDecoder::new(File::open(path).unwrap()));
    let mut counter = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let index = line.find('.').expect("Can't parse input line.");
        let domain = &line[0..index];
        let length = domain.len();
        let buf = &mut lines[length];
        if buf.last().map(String::as_str) != Some(domain) {
            if buf.len() == batch_len {
                let mut buf2 = mem::take(buf);
                buf2.push("".into());
                let message = buf2.join("\n");
                senders[length].send(message).unwrap();
            }
            buf.push(domain.to_string());
            counter += 1;
            if counter % 10_000_000 == 0 {
                println!("Read {} million entries.", counter / 1_000_000);
            }
        }
    }
    for i in 1..=MAX_LENGTH {
        if lines[i].len() > 0 {
            lines[i].push("".into());
            let message = lines[i].join("\n");
            senders[i].send(message).unwrap();
        }
    }
    println!("Finished reading {counter} entries.");
}

fn write_result_files(db: Vec<Vec<u8>>, date: &str) {
    let dir = format!("./db/processed/{date}");
    fs::create_dir_all(&dir).unwrap();
    for i in 1..=64 {
        let file_name = format!("{dir}/{i:0>2}.txt.zst");
        let part = &db[i];
        println!("Writing '{}', {} bytes...", file_name, part.len());
        let mut file = File::create(file_name).unwrap();
        file.write_all(part).unwrap();
    }
}

fn make_result_stream(length: usize, recv: Receiver<String>) -> Result<Vec<u8>> {
    let mut target = Vec::new();
    let mut encoder = zstd::stream::Encoder::new(&mut target, 20).unwrap();
    while let Ok(line) = recv.recv() {
        encoder.write_all(line.as_bytes())?;
    }
    encoder.finish()?;
    Ok(target)
}

fn main() {
    let (raw_zone_file_path, date) = find_raw_zone_file_path();
    println!("Processing raw zone file '{raw_zone_file_path}' ({date})");
    let now = Instant::now();

    let mut senders = vec![];
    let mut threads = vec![];
    for length in 0..=MAX_LENGTH {
        let (sender, recv) = unbounded();
        senders.push(sender);
        threads.push(thread::spawn(move || make_result_stream(length, recv)));
    }
    read_zone_file(&raw_zone_file_path, senders);
    println!(
        "Zone file was read in {:.2?} seconds, waiting for compression...",
        now.elapsed()
    );

    let db = threads
        .into_iter()
        .map(|t| t.join().unwrap().unwrap())
        .collect_vec();
    write_result_files(db, &date);

    println!("Done in {:.2?} seconds.", now.elapsed());
}
