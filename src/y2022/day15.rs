use itertools::Itertools;
use std::cmp::{max, min};

type Coordinate = (isize, isize);

pub fn puzzle_a(input: &str, target_y: isize) -> usize {
    let lines = input.lines().collect::<Vec<_>>();

    let mut p = Vec::new();
    let mut ignore = Vec::new();

    for line in lines {
        let parts = line.split(' ').collect::<Vec<_>>();
        let c = parts.len();
        let sensor_x = &parts[2][0..parts[2].len() - 1];
        let sensor_y = &parts[3][0..parts[3].len() - 1];

        let beacon_y = &parts[c - 1];
        let beacon_x = &parts[c - 2][0..parts[c - 2].len() - 1];

        let sensor_x = sensor_x
            .split('=')
            .last()
            .unwrap()
            .parse::<isize>()
            .unwrap();
        let sensor_y = sensor_y
            .split('=')
            .last()
            .unwrap()
            .parse::<isize>()
            .unwrap();
        let beacon_x = beacon_x
            .split('=')
            .last()
            .unwrap()
            .parse::<isize>()
            .unwrap();
        let beacon_y = beacon_y
            .split('=')
            .last()
            .unwrap()
            .parse::<isize>()
            .unwrap();

        if beacon_y == target_y {
            ignore.push(beacon_x);
        }

        let mut x = (sensor_x - beacon_x).abs();
        let mut current_y = beacon_y;
        loop {
            if current_y == sensor_y {
                break;
            }
            x += 1;
            current_y += if beacon_y > sensor_y { -1 } else { 1 };
        }

        let top_y = sensor_y - x;
        let bottom_y = sensor_y + x;

        if top_y < target_y && bottom_y > target_y {
            let dy = if sensor_y > target_y && sensor_y - x <= target_y {
                sensor_y - x - target_y
            } else if sensor_y < target_y && sensor_y + x >= target_y {
                sensor_y + x - target_y
            } else {
                0
            }
            .abs();

            dbg!(line, x, dy);

            if dy > 0 {
                for i in 0..=dy {
                    p.push(sensor_x - i);
                    p.push(sensor_x + i);
                }
            }
        }
    }

    p.into_iter()
        .unique()
        .filter(|e| ignore.iter().find(|ee| e == *ee).is_none())
        .collect::<Vec<_>>()
        .len()
}

pub fn puzzle_b(input: &str, size: isize) -> usize {
    let lines = input.lines().collect::<Vec<_>>();

    let mut data: Vec<(Coordinate, Coordinate)> = Vec::new();

    for line in lines {
        let parts = line.split(' ').collect::<Vec<_>>();
        let c = parts.len();
        let sensor_x = &parts[2][0..parts[2].len() - 1];
        let sensor_y = &parts[3][0..parts[3].len() - 1];

        let beacon_y = &parts[c - 1];
        let beacon_x = &parts[c - 2][0..parts[c - 2].len() - 1];

        let sensor_x = sensor_x
            .split('=')
            .last()
            .unwrap()
            .parse::<isize>()
            .unwrap();
        let sensor_y = sensor_y
            .split('=')
            .last()
            .unwrap()
            .parse::<isize>()
            .unwrap();
        let beacon_x = beacon_x
            .split('=')
            .last()
            .unwrap()
            .parse::<isize>()
            .unwrap();
        let beacon_y = beacon_y
            .split('=')
            .last()
            .unwrap()
            .parse::<isize>()
            .unwrap();

        data.push(((sensor_x, sensor_y), (beacon_x, beacon_y)));
    }

    let mut aux: Vec<(isize, isize, isize)> = vec![];
    for &((sx, sy), (bx, by)) in data.iter() {
        let distance = calculate_distance(bx, sx) + calculate_distance(by, sy);
        aux.push((sx, sy, distance));
    }

    for y in 0..=size {
        let mut x = 0_isize;
        'next: while x <= size {
            for &(sx, sy, bs_distance) in &aux {
                let my_distance = calculate_distance(x, sx) + calculate_distance(y, sy);
                if my_distance <= bs_distance {
                    let d = calculate_distance(y, sy);
                    let interval_len = bs_distance - d;
                    x = sx + interval_len + 1;
                    continue 'next;
                }
            }
            return x as usize * 4_000_000 + y as usize;
        }
    }

    unreachable!();

    0
}

fn calculate_distance(x: isize, y: isize) -> isize {
    if x > y {
        x - y
    } else {
        y - x
    }
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
