use std::collections::HashMap;

use advent_2015::read_lines;

fn main() {
    let puzzle_data = "puzzles/3.txt";
    let input = read_lines(puzzle_data).unwrap();
    let input = input.get(0).unwrap().trim();

    let answer_a = a(input);
    assert!(answer_a > 1762);
    println!("A Score: {}", answer_a);

    let b_result = b(input);
    println!("B Score: {}", b_result);
}

fn a(input: &str) -> usize {
    let mut x = 0;
    let mut y = 0;

    let mut visited: HashMap<(i32, i32), usize> = HashMap::new();
    visited.insert((x, y), 1);
    input.chars().for_each(|c| {
        match c {
            '^' => {
                y -= 1;
            }
            '<' => {
                x -= 1;
            }
            '>' => {
                x += 1;
            }
            'v' => {
                y += 1;
            }
            _ => {}
        }
        visited.entry((x, y)).and_modify(|v| *v += 1).or_insert(1);
    });

    visited.values().count()
}

fn b(input: &str) -> usize {
    let mut x1 = 0;
    let mut y1 = 0;
    let mut x2 = 0;
    let mut y2 = 0;
    let mut round = 0;

    let mut visited: HashMap<(i32, i32), usize> = HashMap::new();
    visited.insert((x1, y1), 1);
    input.chars().for_each(|c| {
        let mut x = 0;
        let mut y = 0;
        match c {
            '^' => {
                y -= 1;
            }
            '<' => {
                x -= 1;
            }
            '>' => {
                x += 1;
            }
            'v' => {
                y += 1;
            }
            _ => {}
        }

        if round % 2 == 0 {
            x1 += x;
            y1 += y;
        } else {
            x2 += x;
            y2 += y;
        }
        round += 1;
        visited.entry((x1, y1)).and_modify(|v| *v += 1).or_insert(1);
        visited.entry((x2, y2)).and_modify(|v| *v += 1).or_insert(1);
    });

    visited.values().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(a("^v^v^v^v<><>>>"), 5);
    }

    #[test]
    fn test_b() {
        assert_eq!(b("^v^v^v^v^v"), 11);
    }
}
