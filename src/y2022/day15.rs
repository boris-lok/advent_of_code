// Reference From: https://github.com/grhkm21/advent-of-code-2022/blob/master/src/solutions/day_15.rs
use std::cmp::{max, min};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

type Coordinate = (isize, isize);

lazy_static! {
    static ref INT_PATTERN: Regex = Regex::new(r"-?\d+").unwrap();
}

fn dist(x: isize, y: isize, u: isize, v: isize) -> isize {
    (x - u).abs() + (y - v).abs()
}

pub fn puzzle_a(input: &str, target_y: isize) -> usize {
    let mut data = Vec::new();
    let mut min_x = isize::MAX;
    let mut max_x = isize::MIN;
    let mut max_distance = isize::MIN;

    for line in input.lines() {
        let mut integers = INT_PATTERN.captures_iter(line);
        let sx = integers.next().unwrap()[0].parse::<isize>().unwrap();
        let sy = integers.next().unwrap()[0].parse::<isize>().unwrap();
        let bx = integers.next().unwrap()[0].parse::<isize>().unwrap();
        let by = integers.next().unwrap()[0].parse::<isize>().unwrap();
        min_x = min(min_x, min(sx, bx));
        max_x = max(max_x, max(sx, bx));
        let distance = dist(sx, sy, bx, by);
        max_distance = max(max_distance, distance);
        data.push(((sx, sy, bx, by), distance));
    }

    let mut ans = 0;

    for u in min_x - max_distance..=max_x + max_distance {
        let mut possible = false;
        for ((sx, sy, bx, by), distance) in data.iter() {
            let s = (*sx, *sy);
            let b = (*bx, *by);

            if s == (u, target_y) || b == (u, target_y) {
                possible = false;
                continue;
            }

            if dist(*sx, *sy, u, target_y) <= *distance {
                possible = true;
                break;
            }
        }

        if possible {
            ans += 1;
        }
    }

    ans
}

pub fn puzzle_b(input: &str, size: isize) -> usize {
    const N: isize = 4_000_000;
    let mut data = Vec::new();
    let mut min_x = isize::MAX;
    let mut max_x = isize::MIN;
    let mut max_distance = isize::MIN;

    for line in input.lines() {
        let mut integers = INT_PATTERN.captures_iter(line);
        let sx = integers.next().unwrap()[0].parse::<isize>().unwrap();
        let sy = integers.next().unwrap()[0].parse::<isize>().unwrap();
        let bx = integers.next().unwrap()[0].parse::<isize>().unwrap();
        let by = integers.next().unwrap()[0].parse::<isize>().unwrap();
        min_x = min(min_x, min(sx, bx));
        max_x = max(max_x, max(sx, bx));
        let distance = dist(sx, sy, bx, by);
        max_distance = max(max_distance, distance);
        data.push(((sx, sy, bx, by), distance));
    }

    for u in 0..=size {
        let mut impossible = Vec::new();

        for ((sx, sy, _, _), dt) in data.iter() {
            let new_dt = dt - (u - sx).abs();
            if new_dt <= 0 {
                continue;
            }

            let p = (max(0, sy - new_dt), min(size, sy + new_dt));

            impossible.push(p);
        }

        impossible.sort();

        fn intersect(l1: isize, r1: isize, l2: isize, r2: isize) -> Option<(isize, isize)> {
            if r1 + 1 < l2 {
                return None;
            }
            Some((min(l1, l2), max(r1, r2)))
        }

        let mut i = 0;
        while i < impossible.len() - 1 {
            let (l1, r1) = impossible[i];
            let (l2, r2) = impossible[i + 1];
            if let Some((l, r)) = intersect(l1, r1, l2, r2) {
                impossible[i] = (l, r);
                impossible.remove(i + 1);
            } else {
                i += 1;
            }
        }

        if impossible.len() > 1 {
            assert_eq!(impossible[0].1 + 1, impossible[1].0 - 1);
            let v = impossible[0].1 + 1;
            return (u * N + v).try_into().unwrap();
        }
    }

    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_puzzle_a_works() {
        let input = include_str!("../input/day15.test.in");
        assert_eq!(puzzle_a(input, 10), 26);
    }

    #[test]
    fn test_puzzle_b_works() {
        let input = include_str!("../input/day15.test.in");
        assert_eq!(puzzle_b(input, 20), 56000011);
    }
}
