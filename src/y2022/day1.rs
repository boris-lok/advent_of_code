use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_maximum_from_two_numbers(a: i32, b: i32) -> i32 {
    max(a, b)
}

fn find_maximum_three_from_four_numbers(a: i32, b: i32, c: i32, d: i32) -> (i32, i32, i32) {
    let (l1, s) = (max(a, b), min(a, b));
    let (l2, s) = (max(c, s), min(s, c));
    let l3 = max(d, s);
    (l1, l2, l3)
}

pub fn puzzle_a(path: &str) -> usize {
    let f = File::open(path).unwrap_or_else(|_| panic!("Can't open a path: {}", path));
    let reader = BufReader::new(f);

    reader
        .lines()
        .map(|s| s.unwrap())
        .fold((0_usize, 0_usize), |(ans, temp), elem| {
            if elem.is_empty() {
                let res = find_maximum_from_two_numbers(temp as i32, ans as i32) as usize;
                (res, 0)
            } else {
                (ans, temp + elem.parse::<usize>().unwrap())
            }
        })
        .0
}

pub fn puzzle_b(path: &str) -> usize {
    let f = File::open(path).unwrap_or_else(|_| panic!("Can't open a path: {}", path));
    let reader = BufReader::new(f);

    let ans = reader
        .lines()
        .map(|s| s.unwrap())
        .fold(
            ((0_usize, 0_usize, 0_usize), 0_usize),
            |(ans, temp), elem| {
                if elem.is_empty() {
                    let res = find_maximum_three_from_four_numbers(
                        ans.0 as i32,
                        ans.1 as i32,
                        ans.2 as i32,
                        temp as i32,
                    );
                    ((res.0 as usize, res.1 as usize, res.2 as usize), temp)
                } else {
                    (ans, temp + elem.parse::<usize>().unwrap())
                }
            },
        )
        .0;

    ans.0 + ans.1 + ans.2
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_to_find_maximum_three_from_four_numbers_worked() {
        let mut rng = rand::thread_rng();
        // if three numbers are i32, there maybe has overflow when the sum of three number.
        let mut input = (0..4)
            .map(|_| rng.gen::<i8>())
            .map(|e| e as i32)
            .collect::<Vec<_>>();
        input.sort();

        let expected: i32 = input[1..4].iter().sum();

        let (m1, m2, m3) =
            find_maximum_three_from_four_numbers(input[0], input[1], input[2], input[3]);
        assert_eq!(m1 + m2 + m3, expected);
    }
}
