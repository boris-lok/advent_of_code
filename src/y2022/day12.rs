use std::collections::{HashSet, VecDeque};

fn calculate_steps(grid: &[Vec<char>], mut queue: VecDeque<((usize, usize), usize)>, u8_grid: &[Vec<u8>]) -> usize {
    let w = grid[0].len();
    let h = grid.len();

    let mut set = HashSet::new();

    let directions = [
        (-1, 0),
        (0, -1),
        (0, 1),
        (1, 0)
    ];

    while !queue.is_empty() {
        let ((r, c), distance) = queue.pop_front().unwrap();
        if set.contains(&(r, c)) {
            continue;
        }
        set.insert((r, c));
        if grid[r][c] == 'E' {
            return distance;
        }

        for direction in directions {
            let dr = r as isize + direction.0;
            let dc = c as isize + direction.1;


            if dr >= 0 && dr < h as isize
                && dc >= 0 && dc < w as isize
                && u8_grid[dr as usize][dc as usize] <= 1 + u8_grid[r][c] {
                queue.push_back(((dr as usize, dc as usize), distance + 1))
            }
        }
    }

    0
}

pub fn puzzle_a(path: &str) -> usize {
    let input = std::fs::read_to_string(path).unwrap();

    let grid = input
        .lines()
        .map(|e| {
            e.chars().collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let w = grid[0].len();
    let h = grid.len();

    let u8_grid = grid_to_u8(&grid);
    let mut queue = VecDeque::new();

    for y in 0..h {
        for x in 0..w {
            if grid[y][x] == 'S' {
                queue.push_back(((y, x), 0));
                break;
            }
        }
    }

    calculate_steps(&grid, queue, &u8_grid)
}

pub fn puzzle_b(path: &str) -> usize {
    let input = std::fs::read_to_string(path).unwrap();

    let grid = input
        .lines()
        .map(|e| {
            e.chars().collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let w = grid[0].len();
    let h = grid.len();

    let u8_grid = grid_to_u8(&grid);
    let mut queue = VecDeque::new();

    for y in 0..h {
        for x in 0..w {
            if u8_grid[y][x] == 1 {
                queue.push_back(((y, x), 0));
            }
        }
    }

    calculate_steps(&grid, queue, &u8_grid)
}

fn grid_to_u8(grid: &[Vec<char>]) -> Vec<Vec<u8>> {
    grid
        .iter()
        .map(|row| {
            row
                .iter()
                .map(|c| {
                    match c {
                        'S' => 1,
                        'E' => 26,
                        _ => *c as u8 - b'a' + 1
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculation_of_steps_works() {
        let input = include_str!("../input/day12.test.in");
        let mut start_point = (0, 0);

        let grid = input
            .lines()
            .map(|e| {
                e.chars().collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let u8_grid = grid_to_u8(&grid);
        let mut queue = VecDeque::new();

        let w = grid[0].len();
        let h = grid.len();

        for (y, row) in grid.iter().enumerate().take(h) {
            let x = row.iter().position(|e| *e == 'S');
            if let Some(x) = x {
                queue.push_back(((y, x), 0))
            }
        }

        assert_eq!(calculate_steps(&grid, queue, &u8_grid), 31);

        let mut queue = VecDeque::new();

        for y in 0..h {
            for x in 0..w {
                if u8_grid[y][x] == 1 {
                    queue.push_back(((y, x), 0));
                }
            }
        }

        assert_eq!(calculate_steps(&grid, queue, &u8_grid), 29);
    }
}