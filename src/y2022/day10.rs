use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn calculate_score(commands: Vec<String>, steps: Vec<usize>) -> i32 {
    let mut ans = vec![];

    let mut stack = Vec::new();

    for cmd in commands {
        if cmd == "noop" {
            stack.push(0);
        } else {
            let part = cmd.splitn(2, ' ').collect::<Vec<_>>();
            let number = part.last().unwrap().parse::<i32>().unwrap();

            stack.push(0);
            stack.push(number);
        }
    }

    for step in steps {
        let mut res = 1;
        for (idx, val) in stack.iter().enumerate() {
            if idx == step - 1 {
                ans.push(res * step as i32);
            }
            res += val;
        }
    }

    ans.iter().sum::<i32>()
}

fn calculate_crt(commands: Vec<String>) {
    let mut ans = [['.'; 40]; 6];

    let mut t = -1;
    let mut target: i32 = 1;
    for cmd in commands {
        if cmd == "noop" {
            t += 1;
            let y = (t / 40) as usize;
            let x = (t % 40) as usize;
            ans[y][x] = if (target - t % 40).abs() <= 1 {
                '#'
            } else {
                ' '
            };
        } else {
            let part = cmd.splitn(2, ' ').collect::<Vec<_>>();
            let number = part.last().unwrap().parse::<i32>().unwrap();

            t += 1;
            let y = (t / 40) as usize;
            let x = (t % 40) as usize;
            ans[y][x] = if (target - t % 40).abs() <= 1 {
                '#'
            } else {
                ' '
            };

            t += 1;
            let y = (t / 40) as usize;
            let x = (t % 40) as usize;
            ans[y][x] = if (target - t % 40).abs() <= 1 {
                '#'
            } else {
                ' '
            };
            target += number;
        }
    }

    for col in ans {
        println!("{}", col.iter().join(""));
    }
}

pub fn puzzle_a(path: &str) -> i32 {
    let f = File::open(path).unwrap_or_else(|_| panic!("Can't open a path: {}", path));
    let reader = BufReader::new(f);

    let input = reader
        .lines()
        .map(|e| e.unwrap())
        .collect::<Vec<_>>();

    calculate_score(input, vec![20, 60, 100, 140, 180, 220])
}

pub fn puzzle_b(path: &str) -> i32 {
    let f = File::open(path).unwrap_or_else(|_| panic!("Can't open a path: {}", path));
    let reader = BufReader::new(f);

    let input = reader
        .lines()
        .map(|e| e.unwrap())
        .collect::<Vec<_>>();

    calculate_crt(input);
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculation_works() {
        let input = include_str!("../input/day10.test.in");
        let input = input
            .lines()
            .map(|e| e.to_string())
            .collect::<Vec<_>>();

        assert_eq!(calculate_score(input, vec![20, 60, 100, 140, 180, 220]), 13140);
    }
}