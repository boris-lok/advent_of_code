use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn check_is_fully_overlap(a1: i32, a2: i32, b1: i32, b2: i32) -> bool {
    (a1 <= b1 && a2 >= b2) || (b1 <= a1 && b2 >= a2)
}

fn check_is_partial_overlap(a1: i32, a2: i32, b1: i32, b2: i32) -> bool {
    b2 - a1 >= 0 && ((b2 - a1) <= (a2 - a1) + (b2 - b1))
}

pub fn puzzle_a(path: &str) -> usize {
    let f = File::open(path).unwrap_or_else(|_| panic!("Can't open a path: {}", path));
    let reader = BufReader::new(f);

    let pattern = Regex::new(r"([0-9]+)-([0-9]+),([0-9]+)-([0-9]+)").unwrap();

    reader.lines()
        .map(|e| e.unwrap())
        .fold(0_usize, |ans, line| {
            let caps = pattern.captures(&line)
                .unwrap();
            let a1 = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let a2 = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let b1 = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
            let b2 = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();

            if check_is_fully_overlap(a1, a2, b1, b2) {
                ans + 1
            } else {
                ans
            }
        })
}

pub fn puzzle_b(path: &str) -> usize {
    let f = File::open(path).unwrap_or_else(|_| panic!("Can't open a path: {}", path));
    let reader = BufReader::new(f);

    let pattern = Regex::new(r"([0-9]+)-([0-9]+),([0-9]+)-([0-9]+)").unwrap();

    reader.lines()
        .map(|e| e.unwrap())
        .fold(0_usize, |ans, line| {
            let caps = pattern.captures(&line)
                .unwrap();
            let a1 = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let a2 = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let b1 = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
            let b2 = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();

            if check_is_partial_overlap(a1, a2, b1, b2) {
                ans + 1
            } else {
                ans
            }
        })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_is_fully_contain_works() {
        assert!(!check_is_fully_overlap(2, 4, 6, 8));
        assert!(!check_is_fully_overlap(2, 3, 4, 5));
        assert!(!check_is_fully_overlap(5, 7, 7, 9));
        assert!(check_is_fully_overlap(2, 8, 3, 7));
        assert!(check_is_fully_overlap(6, 6, 4, 6));
        assert!(!check_is_fully_overlap(2, 6, 4, 8));
    }

    #[test]
    fn check_is_partial_contain_works() {
        assert!(!check_is_partial_overlap(2, 4, 6, 8));
        assert!(!check_is_partial_overlap(2, 3, 4, 5));
        assert!(check_is_partial_overlap(5, 7, 7, 9));
        assert!(check_is_partial_overlap(2, 8, 3, 7));
        assert!(check_is_partial_overlap(6, 6, 4, 6));
        assert!(check_is_partial_overlap(2, 6, 4, 8));
    }
}