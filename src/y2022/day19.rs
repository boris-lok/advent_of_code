use itertools::{min, Itertools};
use regex::Regex;
use std::cmp::max;
use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
pub struct Cost(pub usize, pub usize, pub usize, pub usize);

pub fn puzzle_a(input: &str, minutes: usize) -> usize {
    let mut costs = Vec::new();
    let single_int_pattern = Regex::new(r"(\d+)").unwrap();
    let multiple_int_pattern = Regex::new(r"(\d+) ore and (\d+)").unwrap();

    for (idx, line) in input.lines().enumerate() {
        let part = line.split(':').last().unwrap();
        let part = part.split('.').collect::<Vec<_>>();
        let o = single_int_pattern
            .captures(part[0])
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let co = Cost(o, 0, 0, 0);
        let c = single_int_pattern
            .captures(part[1])
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let cc = Cost(c, 0, 0, 0);
        let ob1 = multiple_int_pattern
            .captures(part[2])
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let ob2 = multiple_int_pattern
            .captures(part[2])
            .unwrap()
            .get(2)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let cob = Cost(ob1, ob2, 0, 0);
        let g1 = multiple_int_pattern
            .captures(part[3])
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let g2 = multiple_int_pattern
            .captures(part[3])
            .unwrap()
            .get(2)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let cg = Cost(g1, 0, g2, 0);
        costs.push((idx, (co, cc, cob, cg)));
    }

    let mut ans = 0;

    for (idx, (co, cc, cob, cg)) in costs.iter() {
        ans = ans + (idx + 1) * solve(co, cc, cob, cg, minutes);
    }

    ans
}

pub fn puzzle_b(input: &str, minutes: usize) -> usize {
    let mut costs = Vec::new();
    let single_int_pattern = Regex::new(r"(\d+)").unwrap();
    let multiple_int_pattern = Regex::new(r"(\d+) ore and (\d+)").unwrap();

    for (idx, line) in input.lines().enumerate() {
        let part = line.split(':').last().unwrap();
        let part = part.split('.').collect::<Vec<_>>();
        let o = single_int_pattern
            .captures(part[0])
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let co = Cost(o, 0, 0, 0);
        let c = single_int_pattern
            .captures(part[1])
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let cc = Cost(c, 0, 0, 0);
        let ob1 = multiple_int_pattern
            .captures(part[2])
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let ob2 = multiple_int_pattern
            .captures(part[2])
            .unwrap()
            .get(2)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let cob = Cost(ob1, ob2, 0, 0);
        let g1 = multiple_int_pattern
            .captures(part[3])
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let g2 = multiple_int_pattern
            .captures(part[3])
            .unwrap()
            .get(2)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let cg = Cost(g1, 0, g2, 0);
        costs.push((idx, (co, cc, cob, cg)));
    }

    let mut ans = 1;

    for (idx, (co, cc, cob, cg)) in costs.iter() {
        if idx < &3 {
            ans = ans * solve(co, cc, cob, cg, minutes);
        }
    }

    ans
}

pub fn solve(co: &Cost, cc: &Cost, cob: &Cost, cg: &Cost, minutes: usize) -> usize {
    let mut best = 0;
    let state = (1, 0, 0, 0, 0, 0, 0, 0, minutes);
    let mut queue = VecDeque::new();
    queue.push_back(state);
    let mut saw = HashSet::new();

    while !queue.is_empty() {
        let (mut r1, mut r2, mut r3, r4, mut o, mut c, mut ob, g, t) = queue.pop_front().unwrap();

        best = max(best, g);
        if t == 0 {
            continue;
        }

        let core = max(max(max(co.0, cc.0), cob.0), cg.0);

        if r1 > core {
            r1 = core;
        }
        if r2 > cob.1 {
            r2 = cob.1;
        }
        if r3 > cg.2 {
            r3 = cg.2;
        }
        if o >= t * core - r1 * (t - 1) {
            o = t * core - r1 * (t - 1);
        }
        if c >= t * cob.1 - r2 * (t - 1) {
            c = t * cob.1 - r2 * (t - 1);
        }
        if ob >= t * cg.2 - r3 * (t - 1) {
            ob = t * cg.2 - r3 * (t - 1);
        }

        let new_state = (r1, r2, r3, r4, o, c, ob, g, t);
        if saw.contains(&new_state) {
            continue;
        }
        saw.insert(new_state);

        queue.push_back((r1, r2, r3, r4, o + r1, c + r2, ob + r3, g + r4, t - 1));

        if o >= co.0 {
            queue.push_back((
                r1 + 1,
                r2,
                r3,
                r4,
                o + r1 - co.0,
                c + r2,
                ob + r3,
                g + r4,
                t - 1,
            ));
        }
        if o >= cc.0 {
            queue.push_back((
                r1,
                r2 + 1,
                r3,
                r4,
                o + r1 - cc.0,
                c + r2,
                ob + r3,
                g + r4,
                t - 1,
            ));
        }
        if o >= cob.0 && c >= cob.1 {
            queue.push_back((
                r1,
                r2,
                r3 + 1,
                r4,
                o + r1 - cob.0,
                c + r2 - cob.1,
                ob + r3,
                g + r4,
                t - 1,
            ));
        }
        if o >= cg.0 && ob >= cg.2 {
            queue.push_back((
                r1,
                r2,
                r3,
                r4 + 1,
                o + r1 - cg.0,
                c + r2,
                ob + r3 - cg.2,
                g + r4,
                t - 1,
            ));
        }
    }

    best
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_puzzle_a_works() {
        let input = include_str!("../input/day19.test.in");

        assert_eq!(puzzle_a(input, 24), 33);
    }

    #[test]
    fn test_puzzle_b_works() {
        let input = include_str!("../input/day19.test.in");

        assert_eq!(puzzle_b(input, 32), 1);
    }
}
