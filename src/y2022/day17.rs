// Part 2 reference from: https://github.com/HoshigaIkaro/aoc-2022/blob/main/src/days/day_17.rs
use std::arch::aarch64::vabd_f32;
use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet, VecDeque};
use itertools::Itertools;

fn get_piece(round: usize, y: usize) -> Vec<(usize, usize)> {
    let mut piece = Vec::new();
    match round {
        0 => {
            piece.push((2, y));
            piece.push((3, y));
            piece.push((4, y));
            piece.push((5, y));
        }
        1 => {
            piece.push((3, y + 2));
            piece.push((2, y + 1));
            piece.push((3, y + 1));
            piece.push((4, y + 1));
            piece.push((3, y));
        }
        2 => {
            piece.push((4, y + 2));
            piece.push((4, y + 1));
            piece.push((2, y));
            piece.push((3, y));
            piece.push((4, y));
        }
        3 => {
            piece.push((2, y + 3));
            piece.push((2, y + 2));
            piece.push((2, y + 1));
            piece.push((2, y));
        }
        4 => {
            piece.push((2, y + 1));
            piece.push((3, y + 1));
            piece.push((2, y));
            piece.push((3, y));
        }
        _ => unreachable!()
    }
    return piece;
}

fn move_left(piece: &mut Vec<(usize, usize)>) {
    let can_move = piece.iter().all(|e| e.0 >= 1);
    if can_move {
        piece.iter_mut()
            .for_each(|e| {
                *e = (e.0 - 1, e.1);
            });
    }
}

fn move_right(piece: &mut Vec<(usize, usize)>) {
    let can_move = piece.iter().all(|e| e.0 < 6);
    if can_move {
        piece.iter_mut()
            .for_each(|e| {
                *e = (e.0 + 1, e.1);
            });
    }
}

fn move_up(piece: &mut Vec<(usize, usize)>) {
    piece.iter_mut()
        .for_each(|e| {
            *e = (e.0, e.1 + 1);
        });
}

fn move_down(piece: &mut Vec<(usize, usize)>) {
    piece.iter_mut()
        .for_each(|e| {
            *e = (e.0, e.1 - 1);
        });
}

fn show_zone(set: &HashSet<(usize, usize)>) {
    let max_y = set.iter().map(|e| e.1).max().unwrap();

    for y in (0..=max_y).rev() {
        for x in 0..7 {
            if set.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".")
            }
        }
        print!("\n");
    }
}

pub fn puzzle_a(input: &str) -> usize {
    let mut direction = input.chars()
        .collect::<VecDeque<_>>();

    let mut t = 0;
    let mut top = 0;

    let mut zone = (0..7)
        .into_iter()
        .map(|e| (e, 0))
        .collect::<HashSet<_>>();

    loop {
        if t >= 2022 {
            break;
        }
        let mut piece = get_piece(t % 5, top + 4);
        loop {
            let d = direction.pop_front().unwrap();

            match d {
                '>' => {
                    move_right(&mut piece);
                    let mut flag = false;
                    for p in &piece {
                        if zone.contains(p) {
                            flag = true;
                        }
                    }
                    if flag {
                        move_left(&mut piece);
                    }
                }
                '<' => {
                    move_left(&mut piece);
                    let mut flag = false;
                    for p in &piece {
                        if zone.contains(p) {
                            flag = true;
                        }
                    }
                    if flag {
                        move_right(&mut piece);
                    }
                }
                _ => {
                    dbg!(&d);
                    unreachable!();
                }
            }

            direction.push_back(d);
            move_down(&mut piece);
            let mut flag = false;
            for p in &piece {
                if zone.contains(p) {
                    flag = true;
                }
            }
            if flag {
                move_up(&mut piece);

                for p in &piece {
                    zone.insert(*p);
                }

                top = zone.iter().map(|e| e.1).max().unwrap();
                break;
            }
        }

        t += 1;
    }

    top
}

pub fn puzzle_b(input: &str) -> usize {
    const CACHED_SIZE: usize = 5_000;
    let direction = input.chars()
        .collect::<Vec<_>>();

    let mut t = 0;
    let mut top = 0;

    let mut zone = (0..7)
        .into_iter()
        .map(|e| (e, 0))
        .collect::<HashSet<_>>();

    let mut deltas = [0; CACHED_SIZE];

    let mut direction_index = 0;
    let mut c = 0;
    let mut previous_top = 0;

    let mut piece = get_piece(t % 5, top + 4);

    while c < CACHED_SIZE {
        let d = direction[direction_index];

        match d {
            '>' => {
                move_right(&mut piece);
                let mut flag = false;
                for p in &piece {
                    if zone.contains(p) {
                        flag = true;
                    }
                }
                if flag {
                    move_left(&mut piece);
                }
            }
            '<' => {
                move_left(&mut piece);
                let mut flag = false;
                for p in &piece {
                    if zone.contains(p) {
                        flag = true;
                    }
                }
                if flag {
                    move_right(&mut piece);
                }
            }
            _ => {
                dbg!(&d);
                unreachable!();
            }
        }

        direction_index = (direction_index + 1) % direction.len();

        move_down(&mut piece);
        let mut flag = false;
        for p in &piece {
            if zone.contains(p) {
                flag = true;
            }
        }

        if flag {
            move_up(&mut piece);

            for p in &piece {
                zone.insert(*p);
            }

            top = zone.iter().map(|e| e.1).max().unwrap();
            deltas[c] = top - previous_top;
            previous_top = top;
            t += 1;
            piece = get_piece(t % 5, top + 4);
            c += 1;
        }
    }

    let (offset, size) = (0..500)
        .find_map(|offset| {
            let delta_iter = deltas.iter().skip(offset);
            let size = (2..=2500).find(|size| {
                let window = deltas[offset..offset + size].iter().cycle();
                delta_iter.clone().zip(window).all(|(a, b)| a == b)
            });
            size.map(|size| (offset, size))
        })
        .unwrap();

    let mut count = 1_000_000_000_000;
    let offset_delta = deltas.iter().by_ref().take(offset).sum::<usize>();
    count -= offset;
    let cycle_deltas: Vec<usize> = deltas.iter().take(size).copied().collect();
    let cycle_delta = cycle_deltas.iter().sum::<usize>();
    let cycle_count = count / size;
    count %= size;
    let remaining_height = cycle_deltas.into_iter().take(count).sum::<usize>();
    let height = offset_delta + cycle_count * cycle_delta + remaining_height;

    height
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_puzzle_a_works() {
        let input = include_str!("../input/day17.test.in");

        assert_eq!(puzzle_a(input), 3068);
    }

    #[test]
    fn test_puzzle_b_works() {
        let input = include_str!("../input/day17.test.in");

        assert_eq!(puzzle_b(input), 1514285714288);
    }
}