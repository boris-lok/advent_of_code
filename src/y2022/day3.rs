use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_common_char_in_two_string(part_one: &str, part_two: &str) -> usize {
    let mut array = [0_usize; 54];
    part_one.chars().map(char_to_u8).for_each(|index| {
        array[index] = 1;
    });
    for c in part_two.chars().map(char_to_u8) {
        if array[c] == 1 {
            return c;
        }
    }
    0
}

fn find_common_char_in_three_string(part_one: &str, part_two: &str, part_three: &str) -> usize {
    let mut array = [0_usize; 54];
    part_one.chars().map(char_to_u8).for_each(|index| {
        array[index] = 1;
    });
    part_two.chars().map(char_to_u8).for_each(|index| {
        if array[index] == 1 {
            array[index] = 2;
        }
    });

    for c in part_three.chars().map(char_to_u8) {
        if array[c] == 2 {
            return c;
        }
    }
    0
}

fn char_to_u8(c: char) -> usize {
    let c = c as usize;
    if c > 96 {
        c - 96
    } else {
        (c - 64) + 26
    }
}

pub fn puzzle_a(path: &str) -> usize {
    let f = File::open(path).unwrap_or_else(|_| panic!("Can't open a path: {}", path));
    let reader = BufReader::new(f);

    reader
        .lines()
        .map(|e| e.unwrap())
        .fold(0_usize, |ans, elem| {
            let count = elem.len() / 2;
            ans + find_common_char_in_two_string(&elem[0..count], &elem[count..])
        })
}

pub fn puzzle_b(path: &str) -> usize {
    let f = File::open(path).unwrap_or_else(|_| panic!("Can't open a path: {}", path));
    let reader = BufReader::new(f);

    let data = reader.lines().map(|e| e.unwrap()).collect::<Vec<_>>();

    data.chunks(3).fold(0_usize, |ans, elem| {
        ans + find_common_char_in_three_string(&elem[0], &elem[1], &elem[2])
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_finding_common_char_in_two_string() {
        assert_eq!(
            find_common_char_in_two_string("vJrwpWtwJgWr", "hcsFMMfFFhFp"),
            16
        );
        assert_eq!(
            find_common_char_in_two_string("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"),
            38
        );
        assert_eq!(find_common_char_in_two_string("PmmdzqPrV", "vPwwTWBwg"), 42);
        let s = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn";
        let count = s.len();
        let mid = count / 2;
        let part_one = &s[0..mid];
        let part_two = &s[mid..];
        assert_eq!(find_common_char_in_two_string(part_one, part_two), 22);

        let s = "ttgJtRGJQctTZtZT";
        let count = s.len();
        let mid = count / 2;
        let part_one = &s[0..mid];
        let part_two = &s[mid..];
        assert_eq!(find_common_char_in_two_string(part_one, part_two), 20);

        let s = "CrZsJsPPZsGzwwsLwLmpwMDw";
        let count = s.len();
        let mid = count / 2;
        let part_one = &s[0..mid];
        let part_two = &s[mid..];
        assert_eq!(find_common_char_in_two_string(part_one, part_two), 19);
    }

    #[test]
    fn test_finding_common_char_in_three_string() {
        assert_eq!(
            find_common_char_in_three_string(
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg"
            ),
            18
        );
        assert_eq!(
            find_common_char_in_three_string(
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw"
            ),
            52
        )
    }
}
