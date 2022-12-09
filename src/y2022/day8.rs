use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::izip;

fn cal_x(grid: &Vec<Vec<u8>>, w: usize, h: usize) -> Vec<Vec<bool>> {
    let mut ans = vec![vec![false; w]; h];

    for y in 0..h {
        let mut left_max = 0_u8;
        let mut right_max = 0_u8;

        for x in 0..w {
            let left_val = grid[y][x];
            let right_val = grid[y][w - x - 1];
            if y == 0 || y == h - 1 {
                ans[y][x] = true;
            }

            if left_val > left_max {
                ans[y][x] = true;
                left_max = left_val;
            }

            if right_val > right_max {
                ans[y][w - x - 1] = true;
                right_max = right_val;
            }
        }
    }

    ans
}

fn cal_y(grid: &Vec<Vec<u8>>, w: usize, h: usize) -> Vec<Vec<bool>> {
    let mut ans = vec![vec![false; w]; h];

    for x in 0..w {
        let mut top_max = 0_u8;
        let mut bottom_max = 0_u8;

        for y in 0..h {
            let top_val = grid[y][x];
            let bottom_val = grid[h - y - 1][x];

            if x == 0 || x == w - 1 {
                ans[y][x] = true;
            }

            if top_val > top_max {
                ans[y][x] = true;
                top_max = top_val;
            }

            if bottom_val > bottom_max {
                ans[h - y - 1][x] = true;
                bottom_max = bottom_val;
            }
        }
    }

    ans
}

fn calculate_right(grid: &Vec<Vec<u8>>, w: usize, h: usize) -> Vec<Vec<(usize, bool)>> {
    let mut ans = vec![vec![(0_usize, false); w]; h];

    for y in 0..h {
        let mut cursor_x = 0_usize;
        loop {
            if cursor_x == w - 1 {
                break;
            }

            let val = &grid[y][cursor_x];

            for i in cursor_x + 1..w {
                if val > &grid[y][i] {
                    ans[y][cursor_x] = (ans[y][cursor_x].0 + 1, true);
                } else {
                    ans[y][cursor_x] = (ans[y][cursor_x].0 + 1, false);
                    break;
                }
            }

            cursor_x += 1;
        }
    }

    let ans = ans.iter()
        .enumerate()
        .map(|(idx, e)| {
            e.iter().map(|&ee| {
                if idx == 0 || idx == h - 1 {
                    (ee.0, true)
                } else {
                    ee
                }
            })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    ans
}

fn calculate_left(grid: &Vec<Vec<u8>>, w: usize, h: usize) -> Vec<Vec<(usize, bool)>> {
    let mut ans = vec![vec![(0_usize, false); w]; h];

    for y in 0..h {
        let mut cursor_x = w - 1;
        loop {
            if cursor_x == 0 {
                break;
            }

            let val = &grid[y][cursor_x];

            for i in (0..cursor_x).rev() {
                if val > &grid[y][i] {
                    ans[y][cursor_x] = (ans[y][cursor_x].0 + 1, true);
                } else {
                    ans[y][cursor_x] = (ans[y][cursor_x].0 + 1, false);
                    break;
                }
            }

            cursor_x -= 1;
        }
    }

    let ans = ans.iter()
        .enumerate()
        .map(|(idx, e)| {
            e.iter().map(|&ee| {
                if idx == 0 || idx == h - 1 {
                    (ee.0, true)
                } else {
                    ee
                }
            })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    ans
}

fn calculate_bottom(grid: &Vec<Vec<u8>>, w: usize, h: usize) -> Vec<Vec<(usize, bool)>> {
    let mut ans = vec![vec![(0_usize, false); w]; h];

    for x in 0..w {
        let mut cursor_y = 0_usize;
        loop {
            if cursor_y == h - 1 {
                break;
            }

            let val = &grid[cursor_y][x];

            for i in cursor_y + 1..h {
                if val > &grid[i][x] {
                    ans[cursor_y][x] = (ans[cursor_y][x].0 + 1, true);
                } else {
                    ans[cursor_y][x] = (ans[cursor_y][x].0 + 1, false);
                    break;
                }
            }

            cursor_y += 1;
        }
    }

    let ans = ans.iter()
        .map(|e| {
            e.iter()
                .enumerate()
                .map(|(idx, &ee)| {
                    if idx == 0 || idx == w - 1 {
                        (ee.0, true)
                    } else {
                        ee
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    ans
}

fn calculate_top(grid: &Vec<Vec<u8>>, w: usize, h: usize) -> Vec<Vec<(usize, bool)>> {
    let mut ans = vec![vec![(0_usize, false); w]; h];

    for x in 0..w {
        let mut cursor_y = h - 1;
        loop {
            if cursor_y == 0 {
                break;
            }

            let val = &grid[cursor_y][x];

            for i in (0..cursor_y).rev() {
                if val > &grid[i][x] {
                    ans[cursor_y][x] = (ans[cursor_y][x].0 + 1, true);
                } else {
                    ans[cursor_y][x] = (ans[cursor_y][x].0 + 1, false);
                    break;
                }
            }

            cursor_y -= 1;
        }
    }

    let ans = ans.iter()
        .map(|e| {
            e.iter()
                .enumerate()
                .map(|(idx, &ee)| {
                    if idx == 0 || idx == w - 1 {
                        (ee.0, true)
                    } else {
                        ee
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    ans
}

pub fn puzzle_a(path: &str) -> usize {
    let f = File::open(path).unwrap_or_else(|_| panic!("Can't open a path: {}", path));
    let reader = BufReader::new(f);

    let grid = reader.lines()
        .map(|e| e.unwrap())
        .map(|e| e.as_bytes().iter().map(|e| *e).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let w = grid[0].len();
    let h = grid.len();

    let x = cal_x(&grid, w, h);
    let y = cal_y(&grid, w, h);

    izip!(
        x.iter().flatten(),
            y.iter().flatten(),
        ).filter(
        |(&x, &y)| {
            x | y
        }
    ).count()
}

pub fn puzzle_b(path: &str) -> usize {
    let f = File::open(path).unwrap_or_else(|_| panic!("Can't open a path: {}", path));
    let reader = BufReader::new(f);

    let grid = reader.lines()
        .map(|e| e.unwrap())
        .map(|e| e.as_bytes().iter().map(|e| *e).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let w = grid[0].len();
    let h = grid.len();

    let top = calculate_top(&grid, w, h);
    let right = calculate_right(&grid, w, h);
    let bottom = calculate_bottom(&grid, w, h);
    let left = calculate_left(&grid, w, h);

    izip!(
            left.iter().flatten(),
            right.iter().flatten(),
            bottom.iter().flatten(),
            top.iter().flatten()
        )
        .map(|(&l, &r, &b, &t)| {
            l.0 as usize * r.0 as usize * b.0 as usize * t.0 as usize
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use itertools::izip;
    use super::*;

    #[test]
    fn it_works() {
        let data = r"30373
25512
65332
33549
35390";

        let grid = data.lines()
            .map(|e| e.bytes().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let right = calculate_right(&grid, 5, 5);
        let left = calculate_left(&grid, 5, 5);
        let bottom = calculate_bottom(&grid, 5, 5);
        let top = calculate_top(&grid, 5, 5);

        let x = cal_x(&grid, 5, 5);
        let y = cal_y(&grid, 5, 5);

        let visible2 = izip!(
            x.iter().flatten(),
            y.iter().flatten()
        ).filter(|(&x, &y)| x | y).count();

        let visible = izip!(
            left.iter().flatten(),
            right.iter().flatten(),
            bottom.iter().flatten(),
            top.iter().flatten()
        ).filter(|(&l, &r, &b, &t)| l.1 | r.1 | b.1 | t.1).count();

        let highest_score = izip!(
            left.iter().flatten(),
            right.iter().flatten(),
            bottom.iter().flatten(),
            top.iter().flatten()
        ).map(|(&l, &r, &b, &t)| l.0 * r.0 * b.0 * t.0).max().unwrap();

        assert_eq!(visible2, 21);
        assert_eq!(visible, 21);
        assert_eq!(highest_score, 8);
    }
}