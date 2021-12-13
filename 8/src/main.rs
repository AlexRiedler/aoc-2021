use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::iter::Iterator;

fn main() -> Result<(), std::io::Error> {
    let path = Path::new("input_a");

    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let signal_and_segment: Vec<(Vec<String>, Vec<HashSet<char>>)> =
        reader
            .lines()
            .map(|line| {
                let (signals_str, segment_str) = line.unwrap().trim().split_once(" | ").map(|o| (o.0.to_string(), o.1.to_string())).unwrap();
                let signals: Vec<String> = signals_str.trim().split(" ").map(|s| s.to_string()).collect();
                let segments: Vec<HashSet<char>> = segment_str.trim().split(" ").map(|s| s.to_string().chars().collect::<HashSet<char>>()).collect();
                (signals, segments)
            })
            .collect();

    {
        let mut unique_count = 0;
        signal_and_segment
            .iter()
            .for_each(|(_signals, segments)| {
                segments.iter().for_each(|segment| {
                    match segment.len() {
                        2 => { unique_count += 1; } // 1
                        3 => { unique_count += 1; } // 7
                        4 => { unique_count += 1; } // 4
                        7 => { unique_count += 1; } // 8
                        _ => {}
                    }
                });
            });

        println!("part a: {}", unique_count);
    }

    {
        let mut total_sum: u64 = 0;

        signal_and_segment
            .iter()
            .for_each(|(signals, segments)| {
                let mut segments_possibilities = HashMap::new();
                {
                    segments_possibilities.insert('a', create_full_segment_chars());
                    segments_possibilities.insert('b', create_full_segment_chars());
                    segments_possibilities.insert('c', create_full_segment_chars());
                    segments_possibilities.insert('d', create_full_segment_chars());
                    segments_possibilities.insert('e', create_full_segment_chars());
                    segments_possibilities.insert('f', create_full_segment_chars());
                    segments_possibilities.insert('g', create_full_segment_chars());

                    segments_possibilities.insert('r', create_full_segment_chars());
                    segments_possibilities.insert('z', create_full_segment_chars());
                    segments_possibilities.insert('q', create_full_segment_chars());
                }

                let mut segment_signals = HashMap::new();

                signals.iter().for_each(|signal| {
                    let mut set = HashSet::new();
                    signal.chars().for_each(|s| { set.insert(s); });

                    match signal.len() {
                        2 => {
                            // 1: c,f
                            let mut set: HashSet<char> = HashSet::new();
                            signal.chars().for_each(|c| { set.insert(c); });
                            segment_signals.insert(1, set.clone());
                            for ch in vec!['c', 'f'] {
                                let remaining_segments = set.intersection(segments_possibilities.get(&ch).unwrap()).map(|c| c.to_ascii_lowercase()).collect();
                                segments_possibilities.insert(ch, remaining_segments);
                            }
                        }
                        3 => {
                            // 7: a,c,f
                            let mut set = HashSet::new();
                            signal.chars().for_each(|c| { set.insert(c); });
                            segment_signals.insert(7, set.clone());
                            for ch in vec!['a', 'c', 'f'] {
                                let remaining_segments = set.intersection(segments_possibilities.get(&ch).unwrap()).map(|c| c.to_ascii_lowercase()).collect();
                                segments_possibilities.insert(ch, remaining_segments);
                            }
                        }
                        4 => {
                            // 4: b,c,d,f
                            let mut set = HashSet::new();
                            signal.chars().for_each(|c| { set.insert(c); });
                            segment_signals.insert(4, set.clone());
                            for ch in vec!['b', 'c', 'd', 'f'] {
                                let remaining_segments = set.intersection(segments_possibilities.get(&ch).unwrap()).map(|c| c.to_ascii_lowercase()).collect();
                                segments_possibilities.insert(ch, remaining_segments);
                            }
                        }
                        7 => {
                            // 8: a,b,c,d,e,f,g
                            let mut set = HashSet::new();
                            signal.chars().for_each(|c| { set.insert(c); });
                            segment_signals.insert(8, set.clone());
                            for ch in vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'] {
                                let remaining_segments = set.intersection(segments_possibilities.get(&ch).unwrap()).map(|c| c.to_ascii_lowercase()).collect();
                                segments_possibilities.insert(ch, remaining_segments);
                            }
                        },
                        _ => { // 0,6,9,2,3,5: intersection: g
                            let mut set = HashSet::new();
                            signal.chars().for_each(|c| { set.insert(c); });
                            let remaining_segments = set.intersection(segments_possibilities.get(&'g').unwrap()).map(|c| c.to_ascii_lowercase()).collect();
                            segments_possibilities.insert('g', remaining_segments);
                        }
                    }
                    match signal.len() {
                        6 => { // 0, 6, 9: 0 intersect 6 intersect 9 => (inverse of edc)
                            let mut set = HashSet::new();
                            signal.chars().for_each(|c| { set.insert(c); });
                            let remaining_segments = set.intersection(segments_possibilities.get(&'z').unwrap()).map(|c| c.to_ascii_lowercase()).collect();
                            segments_possibilities.insert('z', remaining_segments);
                        },
                        5 => { // 2, 3, 5: 3 intersect 5 intersect 1 => a,d,g
                            let mut set = HashSet::new();
                            signal.chars().for_each(|c| { set.insert(c); });
                            let remaining_segments = set.intersection(segments_possibilities.get(&'q').unwrap()).map(|c| c.to_ascii_lowercase()).collect();
                            segments_possibilities.insert('q', remaining_segments);
                        },
                        _ => {}
                    }
                });

                println!("{:?}", segment_signals);
                println!("{:?}", segments_possibilities);
                let a_segment: HashSet<char> = segment_signals.get(&1).unwrap().symmetric_difference(segment_signals.get(&7).unwrap()).map(|c| c.to_ascii_lowercase()).collect();
                println!("remove {:?}", a_segment);
                let a = a_segment.iter().nth(0).unwrap().to_ascii_lowercase();
                segments_possibilities.iter_mut().for_each(|(key, set)| {
                    set.remove(&a);
                });
                segments_possibilities.insert('a', a_segment);
                println!("{:?}", segments_possibilities);
                // CORRECT

                let d_segment: HashSet<char> = segments_possibilities.get(&'q').unwrap().intersection(segment_signals.get(&4).unwrap()).map(|c| c.to_ascii_lowercase()).collect();
                println!("remove {:?}", d_segment);
                let d = d_segment.iter().nth(0).unwrap().to_ascii_lowercase();
                segments_possibilities.iter_mut().for_each(|(key, set)| {
                    set.remove(&d);
                });
                segments_possibilities.insert('d', d_segment);
                println!("{:?}", segments_possibilities);
                // CORRECT

                let f_segment: HashSet<char> = segments_possibilities.get(&'z').unwrap().intersection(segment_signals.get(&1).unwrap()).map(|c| c.to_ascii_lowercase()).collect();
                println!("remove {:?}", f_segment);
                let f = f_segment.iter().nth(0).unwrap().to_ascii_lowercase();
                segments_possibilities.iter_mut().for_each(|(key, set)| {
                    set.remove(&f);
                });
                segments_possibilities.insert('f', f_segment);
                println!("{:?}", segments_possibilities);
                // CORRECT

                let mut b_segment: HashSet<char> = segment_signals.get(&4).unwrap().difference(segment_signals.get(&1).unwrap()).map(|c| c.to_ascii_lowercase()).collect();
                b_segment.remove(&d);
                println!("remove {:?}", b_segment);
                let b = b_segment.iter().nth(0).unwrap().to_ascii_lowercase();
                segments_possibilities.iter_mut().for_each(|(key, set)| {
                    set.remove(&b);
                });
                segments_possibilities.insert('b', b_segment);
                println!("{:?}", segments_possibilities);

                let g_segment: HashSet<char> = segments_possibilities.get(&'g').unwrap().iter().map(|c| c.to_ascii_lowercase()).collect();
                println!("remove {:?}", g_segment);
                let g = g_segment.iter().nth(0).unwrap().clone();
                segments_possibilities.iter_mut().for_each(|(key, set)| {
                    set.remove(&g);
                });
                segments_possibilities.insert('g', g_segment);
                println!("{:?}", segments_possibilities);

                let c_segment: HashSet<char> = segments_possibilities.get(&'c').unwrap().iter().map(|c| c.to_ascii_lowercase()).collect();
                println!("remove {:?}", c_segment);
                let c = c_segment.iter().nth(0).unwrap().clone();
                segments_possibilities.iter_mut().for_each(|(key, set)| {
                    set.remove(&c);
                });
                segments_possibilities.insert('c', c_segment);
                println!("{:?}", segments_possibilities);

                let e = segments_possibilities.get(&'e').unwrap().iter().map(|c| c.to_ascii_lowercase()).collect::<Vec<char>>().iter().nth(0).unwrap().clone();
                //let b = segments_possibilities.get(&'b').unwrap().iter().map(|c| c.to_ascii_lowercase()).collect::<Vec<char>>().iter().nth(0).unwrap().clone();
                //let c = segments_possibilities.get(&'r').unwrap().iter().map(|c| c.to_ascii_lowercase()).collect::<Vec<char>>().iter().nth(0).unwrap().clone();
                segments_possibilities.remove(&'z');
                segments_possibilities.remove(&'q');
                segments_possibilities.remove(&'r');

                println!("{:?}", signals);
                println!("{:?}", segments_possibilities);
                {
                    let mut set = HashSet::new();
                    set.insert(a.clone());
                    set.insert(b.clone());
                    set.insert(c.clone());
                    set.insert(e.clone());
                    set.insert(f.clone());
                    set.insert(g.clone());
                    segment_signals.insert(0, set);
                }
                {
                    let mut set = HashSet::new();
                    set.insert(a.clone());
                    set.insert(c.clone());
                    set.insert(d.clone());
                    set.insert(e.clone());
                    set.insert(g.clone());
                    segment_signals.insert(2, set);
                }
                {
                    let mut set = HashSet::new();
                    set.insert(a.clone());
                    set.insert(c.clone());
                    set.insert(d.clone());
                    set.insert(f.clone());
                    set.insert(g.clone());
                    segment_signals.insert(3, set);
                }
                {
                    let mut set = HashSet::new();
                    set.insert(a.clone());
                    set.insert(b.clone());
                    set.insert(d.clone());
                    set.insert(f.clone());
                    set.insert(g.clone());
                    segment_signals.insert(5, set);
                }
                {
                    let mut set = HashSet::new();
                    set.insert(a.clone());
                    set.insert(b.clone());
                    set.insert(d.clone());
                    set.insert(e.clone());
                    set.insert(f.clone());
                    set.insert(g.clone());
                    segment_signals.insert(6, set);
                }
                {
                    let mut set = HashSet::new();
                    set.insert(a.clone());
                    set.insert(b.clone());
                    set.insert(c.clone());
                    set.insert(d.clone());
                    set.insert(f.clone());
                    set.insert(g.clone());
                    segment_signals.insert(9, set);
                }

                println!("{:?}", segment_signals);
                println!("{:?}", segments);

                let mut place: u64 = 1000;
                let mut value: u64 = 0;
                segments.iter().for_each(|segment| {
                    let num = segment_signals.iter().find(|(key, signals)| {
                        // println!("{:?} {:?} {:?}", **key, segment, signals);
                        signals.is_subset(&segment) && segment.is_subset(&signals)
                    }).unwrap().0;
                    value += place * num;
                    place = place.div_euclid(10);
                });
                println!("{}", value);
                total_sum += value;
            });

        println!("part b: {}", total_sum);
    }

    Ok(())
}

fn create_full_segment_chars() -> HashSet<char> {
    let mut set = HashSet::<char>::new();
    set.insert('a');
    set.insert('b');
    set.insert('c');
    set.insert('d');
    set.insert('e');
    set.insert('f');
    set.insert('g');
    set
}