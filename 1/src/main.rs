use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    let path = Path::new("input_a");

    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);
    let mut prev_line = String::new();
    reader.read_line(&mut prev_line)?;

    let mut increases = 0;
    while (true) {
        let mut line = String::new();
        let i = reader.read_line(&mut line)?;
        if i == 0 { break; }
        if prev_line.trim().parse::<u64>().unwrap() < line.trim().parse::<u64>().unwrap() {
            increases += 1;
        }
        prev_line = line;
    }
    println!("part A: {}", increases);

    Ok(())
}
