use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::iter::Iterator;

struct Octopus {
    energy: u32,
    flashed: bool,
}
impl Octopus {
    fn new(energy: u32) -> Octopus {
        Octopus {
            energy,
            flashed: false
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let path = Path::new("input_a");

    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let mut octo: Vec<Vec<Octopus>> =
        reader
            .lines()
            .map(|line| {
                line.unwrap().trim().to_string().chars().map(|c|
                    Octopus::new(c.to_digit(10).unwrap())
                ).collect()
            })
            .collect();

    let row_len = octo.len();
    let col_len = octo.first().unwrap().len();
    let full_flash = (row_len * col_len) as u64;

    let mut total_flashes: u64 = 0;

    let mut step = 0;
    loop {
        step += 1;
        println!("After step {}:", step);

        // increase energy
        (0..=row_len).for_each(|row| {
            (0..=col_len).for_each(|col| {
                increase_octopus_energy(&mut octo, row as i32, col as i32, row_len, col_len);
            });
        });

        // reset energy for those that flashed

        let mut flashes: u64 = 0;
        for row in octo.iter_mut() {
            for mut octopus in row.iter_mut() {
                if octopus.flashed {
                    octopus.flashed = false;
                    octopus.energy = 0;
                    flashes += 1;
                }
            }
        }
        total_flashes += flashes;

        print_octopus(&octo);
        println!("total_flashes: {}", total_flashes);
        println!();
        if flashes == full_flash { break }
    }

    Ok(())
}

fn increase_octopus_energy(octo: &mut Vec<Vec<Octopus>>, row: i32, col: i32, row_len: usize, col_len: usize) {
    if row < 0 { return }
    if col < 0 { return }
    if row >= row_len as i32 { return }
    if col >= col_len as i32 { return }

    let octopus = octo.get_mut(row as usize).unwrap().get_mut(col as usize).unwrap();
    octopus.energy += 1;
    if octopus.energy > 9 && !octopus.flashed {
        octopus.flashed = true;
        increase_octopus_energy(octo, row +1, col-1, row_len, col_len);
        increase_octopus_energy(octo, row -1, col+1, row_len, col_len);
        increase_octopus_energy(octo, row +1, col+1, row_len, col_len);
        increase_octopus_energy(octo, row -1, col-1, row_len, col_len);
        increase_octopus_energy(octo, row +1, col, row_len, col_len);
        increase_octopus_energy(octo, row, col+1, row_len, col_len);
        increase_octopus_energy(octo, row -1, col, row_len, col_len);
        increase_octopus_energy(octo, row, col-1, row_len, col_len);
    }
}

fn print_octopus(octo: &Vec<Vec<Octopus>>) {
    for row in octo {
        for col in row {
            print!("{}", col.energy);
        }
        println!();
    }
}
