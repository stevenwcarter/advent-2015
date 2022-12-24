use advent_2015::read_lines;

fn main() {
    let puzzle_data = "puzzles/1.txt";
    let data = read_lines(puzzle_data).unwrap();
    let input = data.get(0).unwrap();

    let answer_a = a(input);
    println!("A Score: {}", answer_a);

    let b_result = b(input);
    println!("B Score: {}", b_result);
}

fn a(input: &str) -> i32 {
    input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .sum()
}

fn b(input: &str) -> i32 {
    let mut pos = 0;
    let mut index = 0;
    for c in input.chars() {
        if pos != -1 {
            let adj = match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            };

            pos += adj;
            index += 1;
        }
    }

    index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(a("((("), 3);
        assert_eq!(a("((())"), 1);
    }

    #[test]
    fn test_b() {
        assert_eq!(b(")"), 1);
        assert_eq!(b("()())"), 5);
    }
}
