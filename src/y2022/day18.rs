// Reference: https://github.com/AxlLind/AdventOfCode2022/blob/main/src/bin/18.rs
use std::collections::{HashSet, VecDeque};
use itertools::Itertools;

pub fn puzzle_a(input: &str) -> usize {
    let mut set = HashSet::new();

    for line in input.split('\n') {
        let mut part = line.split(',');
        let x = part.next().unwrap().parse::<isize>().unwrap();
        let y = part.next().unwrap().parse::<isize>().unwrap();
        let z = part.next().unwrap().parse::<isize>().unwrap();

        set.insert((x, y, z));
    }

    set
        .iter()
        .fold(0_usize, |ans, (x, y, z)| {
            let mut t = ans;

            if !set.contains(&(x + 1, *y, *z)) {
                t += 1;
            }
            if !set.contains(&(x - 1, *y, *z)) {
                t += 1;
            }
            if !set.contains(&(*x, y + 1, *z)) {
                t += 1;
            }
            if !set.contains(&(*x, y - 1, *z)) {
                t += 1;
            }
            if !set.contains(&(*x, *y, z + 1)) {
                t += 1;
            }
            if !set.contains(&(*x, *y, z - 1)) {
                t += 1;
            }

            t
        })
}

fn sides((x, y, z): (isize, isize, isize)) -> [(isize, isize, isize); 6] {
    [(x - 1, y, z), (x + 1, y, z), (x, y - 1, z), (x, y + 1, z), (x, y, z - 1), (x, y, z + 1)]
}

pub fn puzzle_b(input: &str) -> usize {
    let points = input
        .lines()
        .filter_map(|line| {
            line.split(',')
                .map(|p| p.parse().unwrap())
                .collect_tuple()
        })
        .collect::<HashSet<_>>();

    let max = points.iter().flat_map(|&(x, y, z)| [x, y, z]).max().unwrap() + 1;
    let (mut seen, mut q) = (HashSet::new(), vec![(0, 0, 0)]);

    while let Some(p) = q.pop() {
        for (x, y, z) in sides(p) {
            if !points.contains(&(x, y, z)) && !seen.contains(&(x, y, z)) && [x, y, z].iter().all(|&i| -1 <= i && i <= max) {
                seen.insert((x, y, z));
                q.push((x, y, z));
            }
        }
    }

    points.iter().flat_map(|&p| sides(p)).filter(|s| seen.contains(s)).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_puzzle_a_works() {
        let input = include_str!("../input/day18.test.in");
        assert_eq!(puzzle_a(input), 64);
    }

    #[test]
    fn test_puzzle_b_works() {
        let input = include_str!("../input/day18.test.in");
        assert_eq!(puzzle_b(input), 58);
    }
}

