use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::iter::Iterator;
use regex::Regex;

const SIDE: usize = 5;
type Board = [u64; SIDE * SIDE];

fn main() -> Result<(), std::io::Error> {
    let path = Path::new("input_a");

    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);

    let mut buf = String::new();
    let mut numbers_str = String::new();
    reader.read_line(&mut numbers_str)?;
    reader.read_line(&mut buf)?;

    let regex = Regex::new(r"\s+").unwrap();

    let mut board_numbers: Vec<Vec<u64>> = reader
        .lines()
        .filter(|l| l.as_ref().unwrap().trim().len() != 0)
        .map(|line| {
            regex.split(line.as_ref().unwrap().trim()).map(|s| s.parse::<u64>().unwrap()).collect()
        })
        .collect();

    let boards_vec: Vec<&mut [Vec<u64>]> = board_numbers.chunks_mut(5).collect();
    let mut boards: Vec<Board> = Vec::new();
    for board_vec in boards_vec {
        let mut board = Board::default();
        board_vec.iter().flatten().zip(board.iter_mut()).for_each(|(value, elem)| *elem = *value);
        boards.push(board);
    }

    let numbers: Vec<u64> = numbers_str.trim().split(",").map(|s| s.parse::<u64>().unwrap()).collect();

    {
        let mut drawn_numbers = HashMap::new();
        let mut done = false;

        for number in numbers.iter() {
            drawn_numbers.insert(*number, true);

            for board in boards.iter() {
                if has_won(&drawn_numbers, board) {
                    println!("part a: score: {}", calculate_score(&drawn_numbers, board, *number));
                    done = true;
                    break;
                }
            }
            if done { break; }
        }
    }

    {
        let mut drawn_numbers = HashMap::new();
        let mut finished_boards = HashMap::new();

        for number in numbers.iter() {
            drawn_numbers.insert(*number, true);

            for board in boards.iter() {
                if !finished_boards.contains_key(&*board) && has_won(&drawn_numbers, board) {
                    finished_boards.insert(board, true);
                    if finished_boards.len() == boards.len() {
                        println!("part b: score: {}", calculate_score(&drawn_numbers, board, *number));
                    }
                }
            }
        }
    }

    Ok(())
}

fn calculate_score(drawn_numbers: &HashMap<u64, bool>, board: &Board, last_num_called: u64) -> u64 {
    let mut score = 0;
    for val in board {
        score +=
            if drawn_numbers.contains_key(val) {
                0u64
            } else {
                *val
            };
    }
    score * last_num_called
}

fn has_won(drawn_numbers: &HashMap<u64, bool>, board: &Board) -> bool {
    let full_col = (0..SIDE).any(|y| {
        board[y * SIDE..(y + 1) * SIDE].into_iter().all(|col| drawn_numbers.contains_key(col))
    });

    let full_row = (0..SIDE).any(|x| {
        (0..SIDE).map(|y| board[y * SIDE + x]).all(|col| drawn_numbers.contains_key(&col))
    });

    full_row || full_col
}