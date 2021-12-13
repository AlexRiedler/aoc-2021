use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::iter::Iterator;
use std::ops::Add;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
struct Fold {
    dir: String,
    loc: u64,
}

fn main() -> Result<(), std::io::Error> {
    let path = Path::new("input_a");

    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let data: Vec<String> =
        reader
            .lines()
            .map(|line| {
                line.unwrap().trim().to_string()
            })
            .collect();

    let mut points: Vec<Point> = data.iter().take_while(|s| s.contains(","))
        .map(|s| {
            let d = s.split_once(",").unwrap();
            Point { x: (d.0).parse::<u64>().unwrap(), y: (d.1).parse::<u64>().unwrap() }
        })
        .collect();

    println!("{:?}", points);

    let folds: Vec<Fold> = data.iter().filter(|s| s.contains("="))
        .map(|s| {
            let ss = s.replace("fold along ", "");
            let (dir, loc) = ss.split_once("=").unwrap();
            Fold { dir: dir.to_string(), loc: (loc).parse::<u64>().unwrap() }
        }).collect();

    println!("{:?}", folds);

    for fold in folds {
        if fold.dir == "x" {
            let mut folded_points: Vec<&mut Point> = points.iter_mut().filter(|p| p.x > fold.loc).collect();
            for p in folded_points.iter_mut() {
                p.x = p.x - 2 * (p.x - fold.loc);
            }
        } else if fold.dir == "y" {
            let mut folded_points: Vec<&mut Point> = points.iter_mut().filter(|p| p.y > fold.loc).collect();
            for p in folded_points.iter_mut() {
                p.y = p.y - 2 * (p.y - fold.loc);
            }
        }

        let mut set = HashSet::new();
        for p in points.iter() {
            set.insert(p);
        }
        println!("part a: {}", set.len());
    }

    let mut max_x = 0;
    let mut max_y = 0;
    let mut map: HashMap<(u64, u64), bool> = HashMap::new();
    for p in points.iter() {
        map.insert((p.x, p.y), true);
        if p.x > max_x { max_x = p.x; }
        if p.y > max_y { max_y = p.y; }
    }

    for y in 0..=max_y {
        for x in 0..=max_x {
            if let Some(b) = map.get(&(x,y)) {
                print!(" ");
            } else {
                print!("#");
            }
        }
        println!();
    }


    Ok(())
}
