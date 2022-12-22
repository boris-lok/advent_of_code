// Reference:https://github.com/morgoth1145/advent-of-code/blob/da432e32c212831fcbb6c8ae000407b63912ec24/2022/22/solution.py

use crate::y2022::day22::Command::{Forward, Turn};
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Copy, Clone)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[derive(Debug)]
enum Command {
    Forward(usize),
    Turn(char),
}

fn parse_input(
    input: &str,
) -> (
    HashMap<(isize, isize), char>,
    VecDeque<Command>,
    (usize, usize),
) {
    let mut data = input.split("\n\n");
    let m = data.next().unwrap();
    let c = data.next().unwrap();

    let mut map = HashMap::new();
    let mut start: Option<(usize, usize)> = None;

    for (y, row) in m.lines().enumerate() {
        for (x, val) in row.chars().enumerate() {
            match val {
                '.' | '#' => {
                    if val == '.' && start.is_none() {
                        start = Some((x + 1, y + 1));
                    }
                    map.insert((x as isize + 1, y as isize + 1), val);
                }
                _ => {}
            }
        }
    }

    let int_pattern = Regex::new(r"-?\d+").unwrap();
    let char_pattern = Regex::new(r"-?[RL]+").unwrap();

    let mut forwards = int_pattern
        .captures_iter(c)
        .map(|e| e.get(0).unwrap().as_str().parse::<usize>().unwrap())
        .map(Forward)
        .collect::<VecDeque<_>>();

    let mut turns = char_pattern
        .captures_iter(c)
        .map(|e| e.get(0).unwrap().as_str().chars().next().unwrap())
        .map(Turn)
        .collect::<VecDeque<_>>();

    let mut commands = VecDeque::new();
    loop {
        if forwards.is_empty() && turns.is_empty() {
            break;
        }

        if let Some(f) = forwards.pop_front() {
            commands.push_back(f);
        }

        if let Some(t) = turns.pop_front() {
            commands.push_back(t);
        }
    }

    (map, commands, start.unwrap())
}

pub fn puzzle_a(input: &str) -> isize {
    let (map, commands, start) = parse_input(input);
    dbg!(&map, &commands, &start);
    let mut x = start.0 as isize;
    let mut y = start.1 as isize;

    let mut dx = 1;
    let mut dy = 0;

    for c in commands.iter() {
        match c {
            Forward(steps) => {
                for _ in 0..*steps {
                    let (mut nx, mut ny) = (x + dx, y + dy);
                    let mut r = map.get(&(nx, ny));
                    if r.is_none() {
                        let (mut kx, mut ky) = (x, y);
                        while map.contains_key(&(kx, ky)) {
                            nx = kx;
                            ny = ky;
                            kx -= dx;
                            ky -= dy;
                        }
                        r = map.get(&(nx, ny));
                    }

                    if let Some(c) = r {
                        if c == &'#' {
                            break;
                        }
                    }

                    x = nx;
                    y = ny;
                }
            }
            Turn(t) => match t {
                'L' => {
                    let t = dy;
                    dy = -dx;
                    dx = t;
                }
                'R' => {
                    let t = dx;
                    dx = -dy;
                    dy = t;
                }
                _ => unreachable!(),
            },
        }
    }

    dbg!(x, y, get_direction_value((dx, dy)));
    y * 1000 + x * 4 + get_direction_value((dx, dy))
}

pub fn puzzle_b(input: &str) -> isize {
    let (map, commands, start) = parse_input(input);
    dbg!(&map, &commands, &start);
    let mut x = start.0 as isize;
    let mut y = start.1 as isize;

    let mut dx = 1;
    let mut dy = 0;

    for c in commands.iter() {
        match c {
            Forward(steps) => {
                for _ in 0..*steps {
                    let (mut nx, mut ny) = (x + dx, y + dy);
                    let mut r = map.get(&(nx, ny));
                    if r.is_none() {
                        let (wnx, wny, wdx, wdy) = wrap(x, y, dx, dy);
                        r = map.get(&(wnx, wny));

                        if let Some(c) = r {
                            if c == &'#' {
                                break;
                            }
                        }
                        nx = wnx;
                        ny = wny;
                        dx = wdx;
                        dy = wdy;
                    }

                    if let Some(c) = r {
                        if c == &'#' {
                            break;
                        }
                    }

                    x = nx;
                    y = ny;
                }
            }
            Turn(t) => match t {
                'L' => {
                    let t = dy;
                    dy = -dx;
                    dx = t;
                }
                'R' => {
                    let t = dx;
                    dx = -dy;
                    dy = t;
                }
                _ => unreachable!(),
            },
        }
    }

    dbg!(x, y, get_direction_value((dx, dy)));
    y * 1000 + x * 4 + get_direction_value((dx, dy))
}

fn get_direction_value(val: (isize, isize)) -> isize {
    match val {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => unreachable!(),
    }
}

fn wrap(x: isize, y: isize, dx: isize, dy: isize) -> (isize, isize, isize, isize) {
    if dx == 1 {
        match x {
            50 => {
                return (50 + (y - 150), 150, 0, -1);
            }
            100 => {
                if y >= 51 && y <= 100 {
                    return (100 + (y - 50), 50, 0, -1);
                }
                if y >= 101 && y <= 150 {
                    return (150, 51 - (y - 100), -1, 0);
                }
            }
            150 => {
                return (100, 151 - y, -1, 0);
            }
            _ => unreachable!(),
        }
    } else if dx == -1 {
        match x {
            51 => {
                if y >= 1 && y <= 50 {
                    return (1, 151 - y, 1, 0);
                }
                if y >= 51 && y <= 100 {
                    return (y - 50, 101, 0, 1);
                }
            }
            1 => {
                if y >= 101 && y <= 150 {
                    return (51, 1 + (150 - y), 1, 0);
                }
                if y >= 151 && y <= 200 {
                    return (y - 150 + 50, 1, 0, 1);
                }
            }
            _ => unreachable!(),
        }
    } else if dy == 1 {
        if y == 50 {
            return (100, x - 50, -1, 0);
        }
        if y == 150 {
            return (50, x + 100, -1, 0);
        }
        if y == 200 {
            return (x + 100, 1, 0, 1);
        }
    } else if dy == -1 {
        if y == 1 {
            if x >= 51 && x <= 100 {
                return (1, x + 100, 1, 0);
            }
            if x >= 101 && x <= 150 {
                return (x - 100, 200, 0, -1);
            }
        }
        if y == 101 {
            return (51, x + 50, 1, 0);
        }
    }

    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_puzzle_a_works() {
        let input = include_str!("../input/day22.test.in");

        assert_eq!(puzzle_a(input), 6032);
    }

    #[test]
    fn test_puzzle_b_works() {
        let input = include_str!("../input/day22.test.in");

        assert_eq!(puzzle_b(input), 5031);
    }
}
