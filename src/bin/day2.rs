use advent_2015::read_lines;
use min_max::min;

fn main() {
    let puzzle_data = "puzzles/2.txt";

    let answer_a = a(puzzle_data);
    println!("A Score: {}", answer_a);

    let b_result = b(puzzle_data);
    println!("B Score: {}", b_result);
}

pub struct Present {
    pub l: usize,
    pub w: usize,
    pub h: usize,
}

impl Present {
    pub fn new(dims: &str) -> Present {
        let parts = dims
            .trim()
            .split('x')
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        Present {
            l: parts[0],
            w: parts[1],
            h: parts[2],
        }
    }

    pub fn a(&self) -> usize {
        self.l * self.w
    }
    pub fn b(&self) -> usize {
        self.w * self.h
    }
    pub fn c(&self) -> usize {
        self.l * self.h
    }

    pub fn wrapping_needed(&self) -> usize {
        let a = self.a();
        let b = self.b();
        let c = self.c();

        let min = min![a, b, c];

        a * 2 + b * 2 + c * 2 + min
    }
    pub fn ribbon_needed(&self) -> usize {
        let a = self.l + self.w;
        let b = self.w + self.h;
        let c = self.l + self.h;

        let min = min![a, b, c];

        (min * 2) + (self.l * self.w * self.h)
    }
}

fn a(path: &str) -> usize {
    let presents: Vec<Present> = read_lines(path)
        .unwrap()
        .iter()
        .map(|l| Present::new(l))
        .collect();

    presents.iter().map(|p| p.wrapping_needed()).sum()
}

fn b(path: &str) -> usize {
    let presents: Vec<Present> = read_lines(path)
        .unwrap()
        .iter()
        .map(|l| Present::new(l))
        .collect();

    presents.iter().map(|p| p.ribbon_needed()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let test_path = "test-resources/day2.txt";
        let result = a(test_path);
        assert_eq!(result, 101);
    }

    #[test]
    fn test_b() {
        let test_path = "test-resources/day2.txt";
        let result = b(test_path);
        assert_eq!(result, 48);
    }
}
