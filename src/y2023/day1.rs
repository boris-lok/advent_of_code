use std::cmp::min;

pub fn puzzle_a(input: &str) -> u32 {
    return input
        .lines()
        .map(|e| e.to_string())
        .map(to_num)
        .fold(0, |acc, e| acc + e);
}

pub fn puzzle_b(input: &str) -> u32 {
    return input
        .lines()
        .map(|e| e.to_string())
        .map(to_num_b)
        .fold(0, |acc, e| acc + e);
}

fn to_num(s: String) -> u32 {
    let c = s
        .chars()
        .filter(|cc| cc.is_numeric())
        .map(|cc| cc.to_digit(10))
        .filter(|cc| cc.is_some())
        .map(|cc| cc.unwrap())
        .collect::<Vec<_>>();

    assert!(c.len() > 0);

    c.first().unwrap() * 10 + c.last().unwrap()
}

fn to_num_b(s: String) -> u32 {
    let mut b = 10u32;
    let mut e = 10u32;

    for i in 0..s.len() {
        if b != 10 {
            if b == 10 && s[i..].starts_with("0") {
                b = 0;
            } else if b == 10 && (s[i..].starts_with("1") || s[i..].starts_with("one")) {
                b = 1;
            } else if b == 10 && (s[i..].starts_with("2") || s[i..].starts_with("two")) {
                b = 2;
            } else if b == 10 && (s[i..].starts_with("3") || s[i..].starts_with("three")) {
                b = 3;
            } else if b == 10 && (s[i..].starts_with("4") || s[i..].starts_with("four")) {
                b = 4;
            } else if b == 10 && (s[i..].starts_with("5") || s[i..].starts_with("five")) {
                b = 5;
            } else if b == 10 && (s[i..].starts_with("6") || s[i..].starts_with("six")) {
                b = 6;
            } else if b == 10 && (s[i..].starts_with("7") || s[i..].starts_with("seven")) {
                b = 7;
            } else if b == 10 && (s[i..].starts_with("8") || s[i..].starts_with("eight")) {
                b = 8;
            } else if b == 10 && (s[i..].starts_with("9") || s[i..].starts_with("nine")) {
                b = 9;
            }
        }

        let slice = &s[..(s.len() - i)];

        if e != 10 {
            if e == 10 && slice.ends_with("0") {
                e = 0;
            } else if e == 10 && (slice.ends_with("1") || slice.ends_with("one")) {
                e = 1;
            } else if e == 10 && (slice.ends_with("2") || slice.ends_with("two")) {
                e = 2;
            } else if e == 10 && (slice.ends_with("3") || slice.ends_with("three")) {
                e = 3;
            } else if e == 10 && (slice.ends_with("4") || slice.ends_with("four")) {
                e = 4;
            } else if e == 10 && (slice.ends_with("5") || slice.ends_with("five")) {
                e = 5;
            } else if e == 10 && (slice.ends_with("6") || slice.ends_with("six")) {
                e = 6;
            } else if e == 10 && (slice.ends_with("7") || slice.ends_with("seven")) {
                e = 7;
            } else if e == 10 && (slice.ends_with("8") || slice.ends_with("eight")) {
                e = 8;
            } else if e == 10 && (slice.ends_with("9") || slice.ends_with("nine")) {
                e = 9;
            }
        }

        if b != 10 && e != 10 {
            break;
        }
    }

    b * 10 + e
}

#[cfg(test)]
mod test {
    use crate::y2023::day1::{puzzle_a, puzzle_b, to_num, to_num_b};

    #[test]
    fn test_calibration_value() {
        let data = vec![
            "1abc2".to_string(),
            "pqr3stu8vwx".to_string(),
            "a1b2c3d4e5f".to_string(),
            "treb7uchet".to_string(),
        ];

        let ans = data.into_iter().fold(0, |acc, e| acc + to_num(e));

        assert_eq!(ans, 142);
    }

    #[test]
    fn test_replace_string() {
        let data = "two1nine".to_string();
        assert_eq!(to_num_b(data), 29);

        let data = "eightwothree".to_string();
        assert_eq!(to_num_b(data), 83);

        let data = "abcone2threexyz".to_string();
        assert_eq!(to_num_b(data), 13);

        let data = "xtwone3four".to_string();
        assert_eq!(to_num_b(data), 24);

        let data = "4nineeightseven2".to_string();
        assert_eq!(to_num_b(data), 42);

        let data = "zoneight234".to_string();
        assert_eq!(to_num_b(data), 14);

        let data = "7pqrstsixteen".to_string();
        assert_eq!(to_num_b(data), 76);
    }
}
