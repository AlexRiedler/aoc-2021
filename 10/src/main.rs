use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::iter::Iterator;
use std::ops::Div;

fn main() -> Result<(), std::io::Error> {
    let path = Path::new("input_a");

    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let lines: Vec<String> =
        reader
            .lines()
            .map(|line| {
                line.unwrap().trim().to_string()
            })
            .collect();

    let mut score = HashMap::new();
    score.insert(')', 3);
    score.insert(']', 57);
    score.insert('}', 1197);
    score.insert('>', 25137);

    let mut total_score = 0;

    let invalid_lines: Vec<&String> = lines.iter()
        .filter (|line| {
            let mut stack: Vec<char> = vec![];
            let chars: Vec<char> = line.chars().collect();

            let mut invalid_char = None;

            chars.iter()
                .for_each(|c| {
                    if invalid_char.is_none() {
                        match c {
                            '(' => stack.push(')'),
                            '{' => stack.push('}'),
                            '<' => stack.push('>'),
                            '[' => stack.push(']'),
                            ')' => {
                                if let Some(ch) = stack.pop() {
                                    if ch != *c {
                                        invalid_char = Some(c)
                                    }
                                }
                            },
                            '}' => {
                                if let Some(ch) = stack.pop() {
                                    if ch != *c {
                                        invalid_char = Some(c)
                                    }
                                }
                            },
                            '>' => {
                                if let Some(ch) = stack.pop() {
                                    if ch != *c {
                                        invalid_char = Some(c)
                                    }
                                }
                            },
                            ']' => {
                                if let Some(ch) = stack.pop() {
                                    if ch != *c {
                                        invalid_char = Some(c)
                                    }
                                }
                            },
                            _ => { eprintln!("Received unknown character: {}", c) }
                        }
                    }
                });

            if let Some(ch) = invalid_char {
                total_score += score.get(ch).unwrap();
            }

            invalid_char.is_none()
        }).collect();

    println!("part a: {}", total_score);

    let mut correction_score = HashMap::new();
    correction_score.insert(')', 1);
    correction_score.insert(']', 2);
    correction_score.insert('}', 3);
    correction_score.insert('>', 4);

    let mut scores: Vec<u64> = invalid_lines
        .iter()
        .map(|line| {
            let mut stack: Vec<char> = vec![];
            let chars: Vec<char> = line.chars().collect();

            chars.iter()
                .for_each(|c| {
                    match c {
                        '(' => stack.push(')'),
                        '{' => stack.push('}'),
                        '<' => stack.push('>'),
                        '[' => stack.push(']'),
                        ')' => { stack.pop(); },
                        '}' => { stack.pop(); },
                        '>' => { stack.pop(); },
                        ']' => { stack.pop(); },
                        _ => { eprintln!("Received unknown character: {}", c) }
                    }
                });

            let mut invalid_sum: u64 = 0;
            stack.reverse();
            stack
                .iter()
                .for_each(|c| {
                    invalid_sum = invalid_sum * 5 + correction_score[c];
                });

            println!("{:?} : {}", stack, invalid_sum);

            invalid_sum
        })
        .collect();
    scores.sort();

    let answer = scores.get(scores.len().div(2)).unwrap();
    println!("part b: {}", answer);

    Ok(())
}
