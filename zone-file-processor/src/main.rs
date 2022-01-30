use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use anyhow::{anyhow, Result};

fn process_lines(input_file_name: &str, output_file_name: &str) -> Result<()> {
    let reader = BufReader::new(File::open(input_file_name)?);
    let mut writer = BufWriter::new(File::create(output_file_name).unwrap());
    // reader.lines().map(|x| x.unwrap()).collect::<Vec<String>>();
    let mut last = String::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let domain = line.split(' ').nth(0).expect(&format!("Error in line {}", line));
        let s = String::from(domain);
        if last != s {
            writeln!(writer, "{}", s);
            last = s;
        }
    }
    Ok(())
}

fn main() {
    process_lines("input.txt", "output.txt");
}
