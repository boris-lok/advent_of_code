use itertools::Itertools;
use std::cmp::{max, min};

fn parse_input(input: &str) -> (Vec<Vec<i8>>, usize) {
    let mut grid = vec![vec![0_i8; 1000]; 1000];
    let mut abs_y = 0;
    input
        .lines()
        .into_iter()
        .map(|e| e.split(" -> ").collect::<Vec<_>>())
        .for_each(|p| {
            p.windows(2).for_each(|ee| {
                let first = ee.first().unwrap();
                let last = ee.last().unwrap();

                let first = string_to_coordinate(first);
                let last = string_to_coordinate(last);

                abs_y = max(max(abs_y, first.1), last.1);

                let min_x = min(first.0, last.0);
                let max_x = max(first.0, last.0);
                let min_y = min(first.1, last.1);
                let max_y = max(first.1, last.1);

                for y in min_y..=max_y {
                    for x in min_x..=max_x {
                        grid[y][x] = -1;
                    }
                }
            })
        });

    (grid, abs_y)
}

fn string_to_coordinate(input: &str) -> (usize, usize) {
    input
        .split(",")
        .map(|e| e.parse::<usize>().unwrap())
        .collect_tuple::<(usize, usize)>()
        .unwrap()
}

pub fn puzzle_a(input: &str) -> usize {
    let (mut grid, abs_y) = parse_input(input);

    let mut c = 0;

    loop {
        if let Some(p) = move_sand_for_puzzle_a((abs_y + 2) as isize, (500, 0), &grid) {
            grid[p.1 as usize][p.0 as usize] = -2;
            c += 1;
        } else {
            break;
        }
    }
    c
}

pub fn puzzle_b(input: &str) -> usize {
    let (mut grid, abs_y) = parse_input(input);

    let mut c = 0;

    loop {
        if let Some(p) = move_sand_for_puzzle_b((abs_y + 2) as isize, (500, 0), &grid) {
            grid[p.1 as usize][p.0 as usize] = -2;
            c += 1;
            if p.0 == 500 && p.1 == 0 {
                break;
            }
        }
    }
    c
}

fn move_sand_for_puzzle_a(
    abs_y: isize,
    current_position: (isize, isize),
    grid: &[Vec<i8>],
) -> Option<(isize, isize)> {
    if current_position.1 == abs_y {
        return None;
    }

    let direction = [(0, 1), (-1, 1), (1, 1)];

    for d in direction {
        let dx = current_position.0 + d.0;
        let dy = current_position.1 + d.1;

        if dx < 0 || dx > 999 || dy < 0 || dy > 999 {
            return None;
        }

        if grid[dy as usize][dx as usize] == 0 {
            return move_sand_for_puzzle_a(abs_y, (dx, dy), grid);
        }
    }

    return Some(current_position);
}

fn move_sand_for_puzzle_b(
    abs_y: isize,
    current_position: (isize, isize),
    grid: &[Vec<i8>],
) -> Option<(isize, isize)> {
    if current_position.1 == abs_y {
        return None;
    }

    let direction = [(0, 1), (-1, 1), (1, 1)];

    for d in direction {
        let dx = current_position.0 + d.0;
        let dy = current_position.1 + d.1;

        if dx < 0 || dx > 999 || dy < 0 || dy > 999 {
            return None;
        }

        if grid[dy as usize][dx as usize] == 0 {
            if let Some(position) = move_sand_for_puzzle_b(abs_y, (dx, dy), grid) {
                return Some(position);
            } else {
                continue;
            }
        }
    }

    return Some(current_position);
}

fn print_gird(grid: &[Vec<i8>]) {
    for y in 0..12 {
        for x in 490..=510 {
            if grid[y][x] == -1 {
                print!("#");
            } else if grid[y][x] == -2 {
                print!("o");
            } else {
                print!(".")
            }
        }
        print!("{}", '\n');
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_puzzle_a_works() {
        let input = include_str!("../input/day14.test.in");

        assert_eq!(puzzle_a(input), 24);
    }

    #[test]
    fn test_puzzle_b_works() {
        let input = include_str!("../input/day14.test.in");

        assert_eq!(puzzle_b(input), 93);
    }
}
