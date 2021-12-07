use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::iter::Iterator;
use std::ops::Div;

const FISH_PERIOD: usize = 8;
const FISH_LIFETIME: usize = 6;
const DAYS_PART_A: u64 = 80;
const DAYS: u64 = 256;

struct Fishies {
    timers: Vec<u64>,
}

impl Fishies {
    fn new(vec: Vec<u64>) -> Self {
        let mut timers = vec![0; 9];
        for i in vec {
            timers[i as usize] += 1;
        }
        Self { timers }
    }

    fn advance(&mut self) {
        self.timers.rotate_left(1);
        self.timers[FISH_LIFETIME] += self.timers[FISH_PERIOD];
    }

    fn count(&self) -> u64 {
        self.timers.iter().sum()
    }
}

fn main() -> Result<(), std::io::Error> {
    let path = Path::new("input_a");

    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);

    let mut buf = String::new();
    reader.read_line(&mut buf);

    {
        let mut fish_ages: Vec<u64> = buf.trim().split(",").map(|n| n.parse::<u64>().unwrap()).collect();
        (1..=DAYS_PART_A).for_each(|day| {
            let mut new_fishes = 0;
            fish_ages = fish_ages.iter().map(|age|
                if age == &0 {
                    new_fishes += 1;
                    6
                } else {
                    age - 1
                }
            ).collect();
            (0..new_fishes).for_each(|f| fish_ages.push(FISH_PERIOD as u64));
            println!("{} - {}", day, fish_ages.len());
        });
    }

    {
        let mut fish_ages: Vec<u64> = buf.trim().split(",").map(|n| n.parse::<u64>().unwrap()).collect();
        let mut fishies = Fishies::new(fish_ages);
        (1..=DAYS).for_each(|day| {
            fishies.advance();
            println!("{} - {}", day, fishies.count());
        })
    }

    Ok(())
}