use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::iter::Iterator;
use std::ops::Div;

fn main() -> Result<(), std::io::Error> {
    let path = Path::new("input_a");

    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);

    let mut buf = String::new();
    reader.read_line(&mut buf);

    {
        let mut positions: Vec<u64> = buf.trim().split(",").map(|n| n.parse::<u64>().unwrap()).collect();

        let avg = positions.iter().sum::<u64>().div(positions.len() as u64);
        println!("avg: {}", avg);
        let answers: Vec<u64> = (0..=avg + 1).map(|average| {
            positions.iter().map(|p| {
                if average > *p {
                    average - p
                } else {
                    p - average
                }
            }).sum()
        }).collect();

        println!("part a: {:?}", answers.iter().min());
    }

    {
        let mut positions: Vec<u64> = buf.trim().split(",").map(|n| n.parse::<u64>().unwrap()).collect();

        let avg = positions.iter().sum::<u64>().div(positions.len() as u64);
        println!("avg: {}", avg);
        let answers: Vec<u64> = (0..=avg + 1).map(|average| {
            positions.iter().map(|p| {
                let steps =
                    if average > *p {
                        average - p
                    } else {
                        p - average
                    };
                (steps * (steps + 1)).div(2)
            }).sum()
        }).collect();

        println!("part b: {:?}", answers.iter().min());
    }

    Ok(())
}