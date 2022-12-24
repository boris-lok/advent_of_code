use std::collections::{HashSet, VecDeque};

fn parse_input(input: &str) -> ([HashSet<(i32, i32)>; 4], i32, i32) {
    let mut up = HashSet::new();
    let mut right = HashSet::new();
    let mut down = HashSet::new();
    let mut left = HashSet::new();
    let mut h = 0;
    let mut w = 0;

    for (y, row) in input.lines().skip(1).enumerate() {
        h = y;
        for (x, c) in row.chars().skip(1).enumerate() {
            match c {
                '^' => {
                    up.insert((y as i32, x as i32));
                }
                '>' => {
                    right.insert((y as i32, x as i32));
                }
                'v' => {
                    down.insert((y as i32, x as i32));
                }
                '<' => {
                    left.insert((y as i32, x as i32));
                }
                _ => w = x,
            }
        }
    }

    ([left, right, up, down], h as i32, w as i32)
}

pub fn puzzle_a(input: &str) -> i32 {
    let (blizzards, r, c) = parse_input(input);

    let mut queue = VecDeque::new();
    queue.push_back((-1_i32, 0_i32, 0_i32));
    let goal = (r, c - 1);

    let direction = [(0, 1), (0, -1), (-1, 0), (1, 0), (0, 0)];

    let mut saw = HashSet::new();
    let lcm = dbg!(num::integer::lcm(r, c));

    while !queue.is_empty() {
        let (y, x, time) = queue.pop_front().unwrap();

        let time = time + 1;

        for &d in direction.iter() {
            let ny: i32 = y + d.0;
            let nx: i32 = x + d.1;

            if (ny, nx) == goal {
                return time;
            }

            if (nx < 0 || ny < 0 || nx >= c || ny >= r) && (ny, nx) != (-1, 0) {
                continue;
            }

            let mut flag = true;
            for (i, ty, tx) in [(0, 0, -1), (1, 0, 1), (2, -1, 0), (3, 1, 0)] {
                let dx = (nx - tx * time).rem_euclid(c);
                let dy = (ny - ty * time).rem_euclid(r);
                if blizzards[i].contains(&(dy, dx)) {
                    flag = false;
                }
            }

            if flag {
                let key = (ny, nx, time % lcm);

                if saw.contains(&key) {
                    continue;
                }

                saw.insert(key);
                queue.push_back((ny, nx, time));
            }
        }
    }
    0
}

pub fn puzzle_b(input: &str) -> i32 {
    let (blizzards, r, c) = parse_input(input);

    let mut queue = VecDeque::new();
    queue.push_back((-1_i32, 0_i32, 0_i32, 0));
    let goals = [(r, c - 1), (-1, 0)];

    let direction = [(0, 1), (0, -1), (-1, 0), (1, 0), (0, 0)];

    let mut saw = HashSet::new();
    let lcm = dbg!(num::integer::lcm(r, c));

    while !queue.is_empty() {
        let (y, x, time, stage) = queue.pop_front().unwrap();

        let time = time + 1;

        for &d in direction.iter() {
            let ny: i32 = y + d.0;
            let nx: i32 = x + d.1;

            let mut n_stage = stage;

            if (ny, nx) == goals[stage % 2] {
                if n_stage == 2 {
                    return time;
                }
                n_stage += 1;
            }

            if (nx < 0 || ny < 0 || nx >= c || ny >= r)
                && !((ny, nx) == goals[0] || (ny, nx) == goals[1])
            {
                continue;
            }

            let mut flag = true;
            if !((ny, nx) == goals[0] || (ny, nx) == goals[1]) {
                for (i, ty, tx) in [(0, 0, -1), (1, 0, 1), (2, -1, 0), (3, 1, 0)] {
                    let dx = (nx - tx * time).rem_euclid(c);
                    let dy = (ny - ty * time).rem_euclid(r);
                    if blizzards[i].contains(&(dy, dx)) {
                        flag = false;
                        break;
                    }
                }
            }

            if flag {
                let key = (ny, nx, time % lcm, n_stage);

                if saw.contains(&key) {
                    continue;
                }

                saw.insert(key);
                queue.push_back((ny, nx, time, n_stage));
            }
        }
    }
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_puzzle_a_works() {
        let input = include_str!("../input/day24.test.in");

        assert_eq!(puzzle_a(input), 18);
    }

    #[test]
    fn test_puzzle_b_works() {
        let input = include_str!("../input/day24.test.in");

        assert_eq!(puzzle_b(input), 54);
    }
}
