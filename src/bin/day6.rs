use std::collections::HashMap;

use advent_2015::read_lines;
use min_max::{max, min};

fn main() {
    let puzzle_data = "puzzles/6.txt";

    let answer_a = a(puzzle_data);
    println!("A Score: {}", answer_a);

    let b_result = b(puzzle_data);
    println!("B Score: {}", b_result);
}

fn a(path: &str) -> usize {
    let mut grid: HashMap<(u32, u32), bool> = HashMap::new();
    read_lines(path).unwrap().iter().for_each(|line| {
        let command_parts = line.split(' ').collect::<Vec<&str>>();
        let command = command_parts.get(1).unwrap();
        match *command {
            "on" => {
                let start = command_parts[2];
                let end = command_parts[4];
                let start_parts = start
                    .split(',')
                    .map(|a| a.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                let end_parts = end
                    .split(',')
                    .map(|a| a.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                let sx = start_parts.first().unwrap();
                let sy = start_parts.get(1).unwrap();
                let ex = end_parts.first().unwrap();
                let ey = end_parts.get(1).unwrap();
                for x in *min(sx, ex)..=*max(sx, ex) {
                    for y in *min(sy, ey)..=*max(sy, ey) {
                        grid.entry((x, y)).and_modify(|v| *v = true).or_insert(true);
                    }
                }
            }
            "off" => {
                let start = command_parts[2];
                let end = command_parts[4];
                let start_parts = start
                    .split(',')
                    .map(|a| a.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                let end_parts = end
                    .split(',')
                    .map(|a| a.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                let sx = start_parts.first().unwrap();
                let sy = start_parts.get(1).unwrap();
                let ex = end_parts.first().unwrap();
                let ey = end_parts.get(1).unwrap();
                for x in *min(sx, ex)..=*max(sx, ex) {
                    for y in *min(sy, ey)..=*max(sy, ey) {
                        grid.entry((x, y))
                            .and_modify(|v| *v = false)
                            .or_insert(false);
                    }
                }
            }
            _ => {
                let start = command_parts[1];
                let end = command_parts[3];
                let start_parts = start
                    .split(',')
                    .map(|a| a.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                let end_parts = end
                    .split(',')
                    .map(|a| a.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                let sx = start_parts.first().unwrap();
                let sy = start_parts.get(1).unwrap();
                let ex = end_parts.first().unwrap();
                let ey = end_parts.get(1).unwrap();
                for x in *min(sx, ex)..=*max(sx, ex) {
                    for y in *min(sy, ey)..=*max(sy, ey) {
                        grid.entry((x, y)).and_modify(|v| *v = !*v).or_insert(true);
                    }
                }
            }
        }
    });

    grid.values().filter(|v| **v).count()
}

fn b(path: &str) -> u32 {
    let mut grid: HashMap<(u32, u32), u32> = HashMap::new();
    read_lines(path).unwrap().iter().for_each(|line| {
        let command_parts = line.split(' ').collect::<Vec<&str>>();
        let command = command_parts.get(1).unwrap();
        match *command {
            "on" => {
                let start = command_parts[2];
                let end = command_parts[4];
                let start_parts = start
                    .split(',')
                    .map(|a| a.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                let end_parts = end
                    .split(',')
                    .map(|a| a.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                let sx = start_parts.first().unwrap();
                let sy = start_parts.get(1).unwrap();
                let ex = end_parts.first().unwrap();
                let ey = end_parts.get(1).unwrap();
                for x in *min(sx, ex)..=*max(sx, ex) {
                    for y in *min(sy, ey)..=*max(sy, ey) {
                        grid.entry((x, y)).and_modify(|v| *v += 1).or_insert(1);
                    }
                }
            }
            "off" => {
                let start = command_parts[2];
                let end = command_parts[4];
                let start_parts = start
                    .split(',')
                    .map(|a| a.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                let end_parts = end
                    .split(',')
                    .map(|a| a.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                let sx = start_parts.first().unwrap();
                let sy = start_parts.get(1).unwrap();
                let ex = end_parts.first().unwrap();
                let ey = end_parts.get(1).unwrap();
                for x in *min(sx, ex)..=*max(sx, ex) {
                    for y in *min(sy, ey)..=*max(sy, ey) {
                        grid.entry((x, y))
                            .and_modify(|v| {
                                if *v > 0 {
                                    *v -= 1
                                }
                            })
                            .or_insert(0);
                    }
                }
            }
            _ => {
                let start = command_parts[1];
                let end = command_parts[3];
                let start_parts = start
                    .split(',')
                    .map(|a| a.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                let end_parts = end
                    .split(',')
                    .map(|a| a.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                let sx = start_parts.first().unwrap();
                let sy = start_parts.get(1).unwrap();
                let ex = end_parts.first().unwrap();
                let ey = end_parts.get(1).unwrap();
                for x in *min(sx, ex)..=*max(sx, ex) {
                    for y in *min(sy, ey)..=*max(sy, ey) {
                        grid.entry((x, y)).and_modify(|v| *v += 2).or_insert(2);
                    }
                }
            }
        }
    });

    grid.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let test_path = "test-resources/day6.txt";
        let result = a(test_path);
        assert_eq!(result, 998_996);
    }

    #[test]
    fn test_b() {
        let test_path = "test-resources/day6.txt";
        let result = b(test_path);
        assert_eq!(result, 70);
    }
}
