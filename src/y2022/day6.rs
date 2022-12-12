use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn find_the_position(s: &str, offset: usize) -> usize {
    s.as_bytes()
        .windows(offset)
        .position(|chunk| {
            chunk
                .iter()
                .enumerate()
                .all(|(idx, c)| !chunk[..idx].contains(c))
        })
        .unwrap_or(s.len()) + offset
}

pub fn puzzle_a(path: &str) -> usize {
    let f = File::open(path).unwrap_or_else(|_| panic!("Can't open a path: {}", path));
    let reader = BufReader::new(f);

    reader
        .lines()
        .map(|e| e.unwrap())
        .fold(0_usize, |ans, elem| {
            ans + find_the_position(&elem, 4)
        })
}

pub fn puzzle_b(path: &str) -> usize {
    let f = File::open(path).unwrap_or_else(|_| panic!("Can't open a path: {}", path));
    let reader = BufReader::new(f);

    reader
        .lines()
        .map(|e| e.unwrap())
        .fold(0_usize, |ans, elem| {
            ans + find_the_position(&elem, 14)
        })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_finding_the_position_worked() {
        assert_eq!(find_the_position("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(find_the_position("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(find_the_position("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(find_the_position("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);

        assert_eq!(find_the_position("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(find_the_position("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(find_the_position("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(find_the_position("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
        assert_eq!(find_the_position("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }
}
