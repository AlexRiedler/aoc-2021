use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::iter::Iterator;

type Line = ((i64, i64), (i64, i64));

fn main() -> Result<(), std::io::Error> {
    let path = Path::new("input_a");

    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let lines: Vec<Line> = reader
        .lines()
        .map(|line| {
            match line {
                Ok(str) => {
                    let (start, end) = str.trim().split_once(" -> ").unwrap();
                    let (x1, y1) = start.split_once(",").unwrap();
                    let (x2, y2) = end.split_once(",").unwrap();
                    (
                        (x1.parse::<i64>().unwrap(), (y1.parse::<i64>().unwrap())),
                        (x2.parse::<i64>().unwrap(), (y2.parse::<i64>().unwrap()))
                    )
                }
                Err(_e) => ((0,0), (0,0))
            }
        })
        .collect();

    {
        let mut thermal_counts: HashMap<(i64, i64), i64> = HashMap::new();

        lines
            .iter()
            .filter(|((x1, y1), (x2, y2))| { x1 == x2 || y1 == y2 })
            .for_each(|((x1, y1), (x2, y2))| {
                let (x_min, x_max) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
                let (y_min, y_max) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };

                (*x_min..=*x_max).for_each(|x| {
                    (*y_min..=*y_max).for_each(|y| {
                        *thermal_counts.entry((x, y)).or_insert(0) += 1;
                    });
                });
            });

        let thermals = thermal_counts.iter()
            .filter(|(_k, v)| v >= &&2)
            .count();

        println!("part a: thermals={}", thermals);
    }

    {
        let mut thermal_counts: HashMap<(i64, i64), i64> = HashMap::new();

        lines
            .iter()
            .for_each(|((x1, y1), (x2, y2))| {
                println!("LINE: {:?}", ((x1, y1), (x2, y2)));
                let (x_min, x_max) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
                let (y_min, y_max) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };

                if x1 == x2 || y1 == y2 {
                    (*x_min..=*x_max).for_each(|x| {
                        (*y_min..=*y_max).for_each(|y| {
                            *thermal_counts.entry((x, y)).or_insert(0) += 1;
                        });
                    });
                } else {
                    let sign_x = (x2 - x1).signum();
                    let sign_y = (y2 - y1).signum();
                    let magnitude = y_max - y_min;
                    (0..=magnitude).for_each(|m| {
                        let x = x1 + (sign_x * m);
                        let y = y1 + (sign_y * m);
                        *thermal_counts.entry((x, y)).or_insert(0) += 1;
                    })
                }
            });

        let thermals = thermal_counts.iter()
            .filter(|(_k, v)| v >= &&2)
            .count();

        println!("part b: thermals={}", thermals);
    }

    Ok(())
}