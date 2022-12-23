// Reference: https://github.com/AxlLind/AdventOfCode2022/blob/main/src/bin/23.rs

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> HashSet<(i32, i32)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(x, l)| {
            l.bytes()
                .enumerate()
                .filter(|&(_, b)| b == b'#')
                .map(move |(y, _)| (x as i32, y as i32))
        })
        .collect::<HashSet<_>>()
}

fn check_all_direction(x: i32, y: i32, state: &HashSet<(i32, i32)>) -> Vec<bool> {
    let direction = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    direction
        .iter()
        .map(|&e| state.contains(&(x + e.0, y + e.1)))
        .collect()
}

pub fn puzzle_a(input: &str) -> usize {
    let mut state = parse_input(input);

    for t in 0..10 {
        let mut map: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();

        for &(x, y) in state.iter() {
            let checking = check_all_direction(x, y, &state);
            if checking.iter().all(|&c| !c) {
                continue;
            }

            let moves = [
                (!checking[0] && !checking[1] && !checking[2], (x - 1, y)),
                (!checking[5] && !checking[6] && !checking[7], (x + 1, y)),
                (!checking[0] && !checking[3] && !checking[5], (x, y - 1)),
                (!checking[2] && !checking[4] && !checking[7], (x, y + 1)),
            ];

            for i in 0..4 {
                let (can_move, new_pos) = moves[(t + i) % 4];
                if can_move {
                    map.entry(new_pos).or_default().push((x, y));
                    break;
                }
            }
        }

        for (&pos, moves) in map.iter() {
            if moves.len() == 1 {
                state.remove(&moves[0]);
                state.insert(pos);
            }
        }
    }

    let (&min_x, &max_x) = state.iter().map(|(x, _)| x).minmax().into_option().unwrap();
    let (&min_y, &max_y) = state.iter().map(|(_, y)| y).minmax().into_option().unwrap();
    (min_x..=max_x)
        .cartesian_product(min_y..=max_y)
        .filter(|p| !state.contains(p))
        .count()
}

pub fn puzzle_b(input: &str) -> usize {
    let mut state = parse_input(input);
    let mut ans = 0;

    loop {
        let mut map: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();

        for &(x, y) in state.iter() {
            let checking = check_all_direction(x, y, &state);
            if checking.iter().all(|&c| !c) {
                continue;
            }

            let moves = [
                (!checking[0] && !checking[1] && !checking[2], (x - 1, y)),
                (!checking[5] && !checking[6] && !checking[7], (x + 1, y)),
                (!checking[0] && !checking[3] && !checking[5], (x, y - 1)),
                (!checking[2] && !checking[4] && !checking[7], (x, y + 1)),
            ];

            for i in 0..4 {
                let (can_move, new_pos) = moves[(ans + i) % 4];
                if can_move {
                    map.entry(new_pos).or_default().push((x, y));
                    break;
                }
            }
        }

        let mut is_done = false;
        for (&pos, moves) in map.iter() {
            if moves.len() == 1 {
                is_done = true;
                state.remove(&moves[0]);
                state.insert(pos);
            }
        }

        if !is_done {
            break;
        }

        ans += 1;
    }

    ans + 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_puzzle_a_works() {
        let input = include_str!("../input/day23.test.in");
        assert_eq!(puzzle_a(input), 110);
    }

    #[test]
    fn test_puzzle_b_works() {
        let input = include_str!("../input/day23.test.in");
        assert_eq!(puzzle_b(input), 20);
    }
}
