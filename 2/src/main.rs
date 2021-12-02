use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    let path = Path::new("input_a");

    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let commands: Vec<(String, u64)> = reader
        .lines()
        .map(|l| {
            l.unwrap().split_once(" ").map( |cmd| { (cmd.0.to_string(), cmd.1.trim().parse::<u64>().unwrap()) }).unwrap()
        })
        .collect();

    {
        let mut depth = 0u64;
        let mut position = 0u64;

        for (command, magnitude) in commands.iter() {
            match command.as_str() {
                "forward" => {
                    position += magnitude;
                }
                "down" => {
                    depth += magnitude;
                }
                "up" => {
                    depth -= magnitude;
                }
                _ => {
                    eprintln!("Invalid Command: {}", command)
                }
            }
        }

        println!("part a: position={} depth={} answer={}", position, depth, position * depth);
    }

    {
        let mut depth = 0u64;
        let mut position = 0u64;
        let mut aim = 0u64;

        for (command, magnitude) in commands.iter() {
            match command.as_str() {
                "forward" => {
                    position += magnitude;
                    depth += aim * magnitude;
                }
                "down" => {
                    aim += magnitude;
                }
                "up" => {
                    aim -= magnitude;
                }
                _ => {
                    eprintln!("Invalid Command: {}", command)
                }
            }
        }

        println!("part b: position={} depth={} aim={} answer={}", position, depth, aim, position * depth);
    }

    Ok(())
}
