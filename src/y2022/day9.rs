use std::fs::File;
use std::io::{BufRead, BufReader};

fn calculate_visited_for_task_a(commands: Vec<String>) -> usize {
    let mut head_position = (0_i32, 0_i32);
    let mut tail_position = (0_i32, 0_i32);

    let mut tail_visited = Vec::new();
    tail_visited.push(tail_position);


    for command in commands {
        let part = command.splitn(2, " ").collect::<Vec<_>>();
        let direction = part.first().unwrap();
        let distance = part.last().unwrap().parse::<i32>().unwrap();

        for _ in 0..distance {
            match *direction {
                "U" => head_position.1 += 1,
                "R" => head_position.0 += 1,
                "D" => head_position.1 -= 1,
                "L" => head_position.0 -= 1,
                _ => unreachable!()
            }

            let dx: i32 = head_position.0 - tail_position.0;
            let dy: i32 = head_position.1 - tail_position.1;

            if (dx == 0 && dy.abs() == 1) || (dy == 0 && dx.abs() == 1) {
                continue;
            } else if dx.abs() == 1 && dy.abs() == 1 {
                continue;
            } else {
                tail_position.0 += to_one(dx);
                tail_position.1 += to_one(dy);

                tail_visited.push(tail_position);
            }
        }
    }

    let mut unique_tuples = Vec::new();
    for tuple in tail_visited {
        if !unique_tuples.contains(&tuple) {
            unique_tuples.push(tuple);
        }
    }

    unique_tuples.len()
}

fn calculate_visited_for_task_b(commands: Vec<String>) -> usize {
    let mut head_position = (0_i32, 0_i32);
    let mut tail_position = vec![(0_i32, 0_i32); 9];

    let mut tail_visited = Vec::new();
    tail_visited.push(tail_position[0]);


    for command in commands {
        let part = command.splitn(2, " ").collect::<Vec<_>>();
        let direction = part.first().unwrap();
        let distance = part.last().unwrap().parse::<i32>().unwrap();

        for _ in 0..distance {
            match *direction {
                "U" => head_position.1 += 1,
                "R" => head_position.0 += 1,
                "D" => head_position.1 -= 1,
                "L" => head_position.0 -= 1,
                _ => unreachable!()
            }

            if let Some(tail) = adjust_tail(head_position, tail_position[0]) {
                tail_position[0] = tail;
            }

            for i in 1..9 {
                if let Some(tail) = adjust_tail(tail_position[i - 1], tail_position[i]) {
                    tail_position[i] = tail;
                }
            }
            tail_visited.push(tail_position[8]);
        }
    }

    let mut unique_tuples = Vec::new();
    for tuple in tail_visited {
        if !unique_tuples.contains(&tuple) {
            unique_tuples.push(tuple);
        }
    }

    unique_tuples.len()
}

fn adjust_tail(h: (i32, i32), t: (i32, i32)) -> Option<(i32, i32)> {
    let dx: i32 = h.0 - t.0;
    let dy: i32 = h.1 - t.1;

    if (dx == 0 && dy.abs() == 1) || (dy == 0 && dx.abs() == 1) {
        None
    } else if dx.abs() == 1 && dy.abs() == 1 {
        None
    } else {
        Some((t.0 + to_one(dx), t.1 + to_one(dy)))
    }
}

pub fn puzzle_a(path: &str) -> usize {
    let f = File::open(path).unwrap_or_else(|_| panic!("Can't open a path: {}", path));
    let reader = BufReader::new(f);

    let input = reader
        .lines()
        .map(|e| e.unwrap())
        .collect::<Vec<_>>();

    calculate_visited_for_task_a(input)
}

pub fn puzzle_b(path: &str) -> usize {
    let f = File::open(path).unwrap_or_else(|_| panic!("Can't open a path: {}", path));
    let reader = BufReader::new(f);

    let input = reader
        .lines()
        .map(|e| e.unwrap())
        .collect::<Vec<_>>();

    calculate_visited_for_task_b(input)
}

fn to_one(n: i32) -> i32 {
    if n > 0 {
        1
    } else if n < 0 {
        -1
    } else {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_visited_for_task_a_works() {
        let data = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        let input = data.lines()
            .map(|e| e.to_string())
            .collect::<Vec<_>>();

        assert_eq!(calculate_visited_for_task_a(input), 13);
    }

    #[test]
    fn test_calculate_visited_for_task_b_works() {
        let data = r"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

        let input = data.lines()
            .map(|e| e.to_string())
            .collect::<Vec<_>>();

        assert_eq!(calculate_visited_for_task_b(input), 36);
    }
}