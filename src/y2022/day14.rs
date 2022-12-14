use std::cmp::{max, min};
use itertools::Itertools;

pub fn puzzle_a(input: &str) -> usize {
    let mut grid = vec![vec![0_i8; 1000]; 1000];

    let lines = input.lines()
        .collect::<Vec<_>>();

    let part = lines
        .iter()
        .map(|&e| {
            e.split(" -> ")
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut abs_y = 0;

    for p in part {
        p.windows(2)
            .for_each(|e| {
                let first = e.first().unwrap();
                let last = e.last().unwrap();

                let first = first.split(",")
                    .map(|e| e.parse::<usize>().unwrap()).collect_tuple::<(usize, usize)>()
                    .unwrap();

                let last = last.split(",")
                    .map(|e| e.parse::<usize>().unwrap()).collect_tuple::<(usize, usize)>()
                    .unwrap();

                let begin = min(first.0, last.0);
                let end = max(first.0, last.0);

                abs_y = max(first.1, abs_y);
                abs_y = max(last.1, abs_y);

                for dx in begin..=end {
                    grid[first.1][dx] = -1;
                }

                let begin = min(first.1, last.1);
                let end = max(first.1, last.1);

                for dy in begin..=end {
                    grid[dy][first.0] = -1;
                }
            });
    }

    let mut c = 0;

    loop {
        if let Some(p) = move_sand((abs_y + 2) as isize, (500, 0), &grid) {
            grid[p.1 as usize][p.0 as usize] = -2;
            c += 1;
        } else {
            break;
        }
    }
    c
}

pub fn puzzle_b(input: &str) -> usize {
    let mut grid = vec![vec![0_i8; 1000]; 1000];

    let lines = input.lines()
        .collect::<Vec<_>>();

    let part = lines
        .iter()
        .map(|&e| {
            e.split(" -> ")
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut abs_y = 0;

    for p in part {
        p.windows(2)
            .for_each(|e| {
                let first = e.first().unwrap();
                let last = e.last().unwrap();

                let first = first.split(",")
                    .map(|e| e.parse::<usize>().unwrap()).collect_tuple::<(usize, usize)>()
                    .unwrap();

                let last = last.split(",")
                    .map(|e| e.parse::<usize>().unwrap()).collect_tuple::<(usize, usize)>()
                    .unwrap();

                let begin = min(first.0, last.0);
                let end = max(first.0, last.0);

                abs_y = max(first.1, abs_y);
                abs_y = max(last.1, abs_y);

                for dx in begin..=end {
                    grid[first.1][dx] = -1;
                }

                let begin = min(first.1, last.1);
                let end = max(first.1, last.1);

                for dy in begin..=end {
                    grid[dy][first.0] = -1;
                }
            });
    }

    let mut c = 0;

    loop {
        if let Some(p) = move_sand_2((abs_y + 2) as isize, (500, 0), &grid) {
            grid[p.1 as usize][p.0 as usize] = -2;
            c += 1;
            if p.0 == 500 && p.1 == 0 {
                break;
            }
        }
    }
    c
}


fn move_sand(abs_y: isize, current_position: (isize, isize), grid: &[Vec<i8>]) -> Option<(isize, isize)> {
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
            return move_sand(abs_y, (dx, dy), grid);
        }
    }

    return Some(current_position);
}

fn move_sand_2(abs_y: isize, current_position: (isize, isize), grid: &[Vec<i8>]) -> Option<(isize, isize)> {
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
            if let Some(position) = move_sand_2(abs_y, (dx, dy), grid) {
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