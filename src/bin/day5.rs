/// Santa needs help figuring out which strings in his text file are naughty or nice.

/// A nice string is one with all of the following properties:
///
/// It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
/// It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
/// It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.
/// For example:
///
/// ugknbfddgicrmopn is nice because it has at least three vowels (u...i...o...), a double letter (...dd...), and none of the disallowed substrings.
/// aaa is nice because it has at least three vowels and a double letter, even though the letters used by different rules overlap.
/// jchzalrnumimnmhp is naughty because it has no double letter.
/// haegwjzuvuyypxyu is naughty because it contains the string xy.
/// dvszwmarrgswjxmb is naughty because it contains only one vowel.

fn main() {
    let puzzle_data = "puzzles/5.txt";

    let input: String = std::fs::read_to_string(puzzle_data).unwrap();

    let answer_a = a(&input);
    println!("A Score: {}", answer_a);

    let b_result = b(&input);
    println!("B Score: {}", b_result);
}

fn vowel_test(line: &&str) -> bool {
    line.chars().filter(|c| "aeiou".contains(*c)).count() >= 3
}

fn double_letter_test(line: &&str) -> bool {
    let mut found = false;
    let checks = vec![
        "aa", "bb", "cc", "dd", "ee", "ff", "gg", "hh", "ii", "jj", "kk", "ll", "mm", "nn", "oo",
        "pp", "qq", "rr", "ss", "tt", "uu", "vv", "ww", "xx", "yy", "zz",
    ];
    for check in checks {
        if line.contains(check) {
            found = true;
        }
    }

    found
}

fn missing_naughty_letters(line: &&str) -> bool {
    // ab, cd, pq, or xy,
    let mut nice = true;
    let checks = vec!["ab", "cd", "pq", "xy"];
    for check in checks {
        if line.contains(check) {
            nice = false;
        }
    }
    nice
}

fn a(input: &str) -> usize {
    input
        .split('\n')
        .into_iter()
        .filter(vowel_test)
        .filter(double_letter_test)
        .filter(missing_naughty_letters)
        .count()
}

fn letter_pair_test(line: &&str) -> bool {
    let checks = vec![
        "aa", "bb", "cc", "dd", "ee", "ff", "gg", "hh", "ii", "jj", "kk", "ll", "mm", "nn", "oo",
        "pp", "qq", "rr", "ss", "tt", "uu", "vv", "ww", "xx", "yy", "zz",
    ];

    let mut nice = false;
    for check in checks {
        if !nice {
            let pos1 = line.find(check);
            if let Some(pos1) = pos1 {
                let pos2 = line[pos1 + 2..].find(check);
                if pos2.is_some() {
                    nice = true;
                }
            }
        }
    }

    nice
}
// fn letter_repeats_one_between(line: &&str) -> bool {
//     let checks = vec![
//         "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
//         "s", "t", "u", "v", "w", "x", "y", "z",
//     ];
//     let line_iter = line.chars().peekable();
//     let mut nice = false;
// }

fn b(input: &str) -> usize {
    input
        .split('\n')
        .into_iter()
        .filter(vowel_test)
        .filter(double_letter_test)
        .filter(missing_naughty_letters)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert!(vowel_test(&"aaa"));
        assert!(double_letter_test(&"aaa"));
        assert!(missing_naughty_letters(&"aaa"));
        assert_eq!(
            a("aaa\nugknbfddgicrmopn\njchzalrnumimnmhp\nhaegwjzuvuyypxyu\ndvszwmarrgswjxmb"),
            2
        );
    }

    #[test]
    fn test_b() {
        let test_path = "test-resources/day5.txt";
        let result = b(test_path);
        assert_eq!(result, 70);
    }
}
