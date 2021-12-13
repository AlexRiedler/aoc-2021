use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::iter::Iterator;

fn main() -> Result<(), std::io::Error> {
    let path = Path::new("input_a");

    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let heat_map: Vec<Vec<u32>> =
        reader
            .lines()
            .map(|line| {
                line.unwrap().trim().chars().map(|c| c.to_digit(10).unwrap()).collect()
            })
            .collect();

    {
        let mut low_points = vec![];

        let heat_map2 = heat_map.clone();

        heat_map.iter().enumerate().for_each(|(row_idx, row)| {
            row.iter().enumerate().for_each(|(col_idx, num)| {
                let mut is_low_point = true;
                if row_idx > 0 {
                    let row = heat_map2.get(row_idx - 1).unwrap();
                    if let Some(outer_num) = row.get(col_idx) {
                        is_low_point = is_low_point && outer_num > num;
                    }
                }
                if let Some(row) = heat_map2.get(row_idx) {
                    if col_idx > 0 {
                        let outer_num = row.get(col_idx - 1).unwrap();
                        is_low_point = is_low_point && outer_num > num;
                    }
                }
                if let Some(row) = heat_map2.get(row_idx) {
                    if let Some(outer_num) = row.get(col_idx + 1) {
                        is_low_point = is_low_point && outer_num > num;
                    }
                }
                if let Some(row) = heat_map2.get(row_idx + 1) {
                    if let Some(outer_num) = row.get(col_idx) {
                        is_low_point = is_low_point && outer_num > num;
                    }
                }
                if is_low_point {
                    low_points.push(num + 1);
                }
            });
        });

        println!("part a:");
        println!("{:?}", low_points.iter().sum::<u32>());
    }

    {
        let mut low_point_sizes = vec![];

        let heat_map2 = heat_map.clone();

        heat_map.iter().enumerate().for_each(|(row_idx, row)| {
            row.iter().enumerate().for_each(|(col_idx, num)| {
                let mut is_low_point = true;
                if row_idx > 0 {
                    let row = heat_map2.get(row_idx - 1).unwrap();
                    if let Some(outer_num) = row.get(col_idx) {
                        is_low_point = is_low_point && outer_num > num;
                    }
                }
                if let Some(row) = heat_map2.get(row_idx) {
                    if col_idx > 0 {
                        let outer_num = row.get(col_idx - 1).unwrap();
                        is_low_point = is_low_point && outer_num > num;
                    }
                }
                if let Some(row) = heat_map2.get(row_idx) {
                    if let Some(outer_num) = row.get(col_idx + 1) {
                        is_low_point = is_low_point && outer_num > num;
                    }
                }
                if let Some(row) = heat_map2.get(row_idx + 1) {
                    if let Some(outer_num) = row.get(col_idx) {
                        is_low_point = is_low_point && outer_num > num;
                    }
                }
                if is_low_point {
                    let mut visited: HashSet<(usize, usize)> = HashSet::new();
                    visit_nodes(&heat_map2, row_idx, col_idx, &mut visited);

                    low_point_sizes.push(visited.len());
                }
            });
        });

        println!("part b:");
        low_point_sizes.sort();
        low_point_sizes.reverse();
        low_point_sizes.truncate(3);
        println!("{:?}", low_point_sizes);
        println!("{:?}", low_point_sizes.iter().fold(1, |a,b| a * *b));
    }

    Ok(())
}

fn visit_nodes(heat_map: &Vec<Vec<u32>>, row_idx: usize, col_idx: usize, mut visited: &mut HashSet<(usize, usize)>) {
    if visited.contains(&(row_idx, col_idx)) { return };
    if let Some(col) = heat_map.get(row_idx).map(|row| row.get(col_idx)) {
        if let Some(value) = col {
            if value == &9 { return };

            visited.insert((row_idx, col_idx));

            if row_idx > 0 {
                visit_nodes(heat_map, row_idx - 1, col_idx, visited);
            }
            if col_idx > 0 {
                visit_nodes(heat_map, row_idx, col_idx - 1, visited);
            }
            visit_nodes(heat_map, row_idx + 1, col_idx, visited);
            visit_nodes(heat_map, row_idx, col_idx + 1, visited);
        }
    };
}
