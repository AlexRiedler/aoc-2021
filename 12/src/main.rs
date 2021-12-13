use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::iter::Iterator;
use std::ops::Add;

#[derive(Debug)]
struct Edge {
    start: String,
    end: String,
}

fn main() -> Result<(), std::io::Error> {
    let path = Path::new("input_a");

    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let edges: Vec<Edge> =
        reader
            .lines()
            .map(|line| {
                line.unwrap().trim().to_string().split_once('-').map(|(start, end)|
                    Edge { start: start.to_string(), end: end.to_string() }
                ).unwrap()
            })
            .collect();

    let paths_a = number_of_paths_part_a("start".to_string(), &edges, &mut HashSet::new(), "".to_string());
    println!("part a: {}", paths_a);

    let paths_b = number_of_paths_part_b("start".to_string(), &edges, &mut HashMap::new(), "".to_string(), false);
    println!("part b: {}", paths_b);

    Ok(())
}

fn number_of_paths_part_a(loc: String, edges: &Vec<Edge>, visited_small_caves: &mut HashSet<String>, path: String) -> u64 {
    if loc.to_ascii_lowercase() == loc {
        visited_small_caves.insert(loc.clone());
    }

    if loc.as_str() == "end" {
        //println!("{},{}", path, loc);
        return 1
    }

    let outward_edges: Vec<&Edge> = edges
        .iter()
        .filter(|e| { e.start == loc && !visited_small_caves.contains(&e.end) })
        .collect();

    let backward_edges: Vec<&Edge> = edges
        .iter()
        .filter(|e| { e.end == loc && !visited_small_caves.contains(&e.start) })
        .collect();

    let new_path = path.clone().add(",").add(loc.as_str());

    outward_edges.iter().map(|e| number_of_paths_part_a(e.end.clone(), edges, &mut visited_small_caves.clone(), new_path.clone())).sum::<u64>()
        + backward_edges.iter().map(|e| number_of_paths_part_a(e.start.clone(), edges, &mut visited_small_caves.clone(), new_path.clone())).sum::<u64>()
}

fn number_of_paths_part_b(loc: String, edges: &Vec<Edge>, visited_small_caves: &mut HashMap<String, u64>, path: String, visited_smallcave_twice: bool) -> u64 {
    let mut visited_twice = false;
    visited_small_caves.insert("start".to_string(), 2);
    if loc.to_ascii_lowercase() == loc {
        if visited_small_caves.contains_key(loc.as_str()) {
            visited_small_caves.insert(loc.clone(), visited_small_caves.get(&loc).unwrap() + 1);
            visited_twice = true;
            if visited_small_caves.iter().filter(|(_key, val)| { val >= &&2u64 }).count() > 2 {
                return 0
            }
        } else {
            visited_small_caves.insert(loc.clone(), 1);
        }
    }

    if loc.as_str() == "end" {
        return if visited_small_caves.iter().filter(|(_key, val)| { val >= &&2u64 }).count() <= 2 {
            1
        } else {
            0
        }
    }

    let outward_edges: Vec<&Edge> = edges
        .iter()
        .filter(|e| { e.start == loc && (visited_small_caves.get(&e.end).is_none() || visited_small_caves.get(&e.end).unwrap() < &2u64) })
        .collect();

    let backward_edges: Vec<&Edge> = edges
        .iter()
        .filter(|e| { e.end == loc && (visited_small_caves.get(&e.start).is_none() || visited_small_caves.get(&e.start).unwrap() < &2u64) })
        .collect();

    let new_path = path.clone().add(",").add(loc.as_str());

    outward_edges.iter().map(|e| number_of_paths_part_b(e.end.clone(), edges, &mut visited_small_caves.clone(), new_path.clone(), visited_twice)).sum::<u64>()
        + backward_edges.iter().map(|e| number_of_paths_part_b(e.start.clone(), edges, &mut visited_small_caves.clone(), new_path.clone(), visited_twice)).sum::<u64>()
}

