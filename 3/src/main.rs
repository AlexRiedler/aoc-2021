use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Div};
use std::path::Path;
use std::iter::Iterator;

fn main() -> Result<(), std::io::Error> {
    let path = Path::new("input_a");

    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let diagnostic_strs: Vec<_> = reader
        .lines()
        .map(|l| {
            l.unwrap().trim().to_string()
        })
        .collect();

    {
        let mut gamma_str = String::new();
        let mut diagnostic: Vec<&String> = diagnostic_strs.iter().collect();
        for x in 0..diagnostic[0].len() {
            sort_by_nth_char(&mut diagnostic, x);
            gamma_str.insert(gamma_str.len(), diagnostic[diagnostic.len().div(2)].chars().nth(x).unwrap());
        }
        let epsilon_str: String = gamma_str.chars().map(|c| { if c == '0' { '1' } else { '0' } }).collect();

        let gamma = isize::from_str_radix(gamma_str.as_str(), 2).unwrap();
        let epsilon = isize::from_str_radix(epsilon_str.as_str(), 2).unwrap();
        println!("part a: gamma={:b} epsilon={:b} answer={}", gamma, epsilon, gamma * epsilon);
    }

    {
        let oxygen_rating_str = find_oxygen_rating(&diagnostic_strs).unwrap();
        let co2_rating_str = find_co2_rating(&diagnostic_strs).unwrap();

        let oxygen_rating = isize::from_str_radix(oxygen_rating_str.as_str(), 2).unwrap();
        let co2_rating= isize::from_str_radix(co2_rating_str.as_str(), 2).unwrap();
        println!("part b: oxygen={:b} co2_rating={:b} answer={}", oxygen_rating, co2_rating, oxygen_rating * co2_rating);
    }

    Ok(())
}

fn find_oxygen_rating(diagnostic: &Vec<String>) -> Option<String> {
    let mut remaining: Vec<&String> = diagnostic.iter().collect();
    for x in 0..remaining[0].len() {
        sort_by_nth_char(&mut remaining, x);
        let filter_char = remaining[remaining.len().div(2)].chars().nth(x).unwrap();
        remaining = remaining.iter().filter(|s| { s.chars().nth(x).unwrap() == filter_char }).cloned().collect();
        if remaining.len() == 1 {
            return Some(remaining.first().unwrap().to_string());
        }
    }
    None
}

fn find_co2_rating(diagnostic: &Vec<String>) -> Option<String> {
    let mut remaining: Vec<&String> = diagnostic.iter().collect();
    for x in 0..remaining[0].len() {
        sort_by_nth_char(&mut remaining, x);
        let filter_char = remaining[remaining.len().div(2)].chars().nth(x).unwrap();
        remaining = remaining.iter().filter(|s| { s.chars().nth(x).unwrap() != filter_char }).cloned().collect();
        if remaining.len() == 1 {
            return Some(remaining.first().unwrap().to_string());
        }
    }
    None
}

fn sort_by_nth_char(remaining: &mut Vec<&String>, x: usize) {
    remaining.sort_by(|a, b| { a.chars().nth(x).unwrap().partial_cmp(&b.chars().nth(x).unwrap()).unwrap() });
}
