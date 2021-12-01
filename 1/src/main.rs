use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    {
        let path = Path::new("input_a");

        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);
        let mut prev_line = String::new();
        reader.read_line(&mut prev_line)?;

        let mut increases = 0;
        loop {
            let mut line = String::new();
            let i = reader.read_line(&mut line)?;
            if i == 0 { break; }
            if prev_line.trim().parse::<u64>().unwrap() < line.trim().parse::<u64>().unwrap() {
                increases += 1;
            }
            prev_line = line;
        }
        println!("part A: {}", increases);
    }


    {
        let path = Path::new("input_a");
        let file = File::open(&path)?;
        let reader = BufReader::new(file);

        let mut increases = 0;
        let mut prev_sum: Option<u64> = None;
        for window in reader.lines().map(|v| { v.unwrap().trim().parse::<u64>().unwrap() }).collect::<Vec<u64>>().windows(3) {
            match prev_sum {
                Some(prev_val) => {
                    let new_sum: u64 = window.iter().sum();
                    if prev_val < new_sum {
                        increases += 1;
                    }
                    prev_sum = Some(new_sum)
                }
                None => {
                    prev_sum = Some(window.iter().sum())
                }
            }
        }
        println!("part B: {}", increases);
    }

    Ok(())
}
