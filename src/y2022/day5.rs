/// Reference from: https://dev.to/nickymeuleman/advent-of-code-2022-day-5-29ok

use itertools::Itertools;

struct Command {
    steps: usize,
    from: usize,
    to: usize,
}


fn parse_input(path: &str) -> Option<(Vec<Vec<char>>, Vec<Command>)> {
    let input = std::fs::read_to_string(path).ok()?;
    let (left, instructions_str) = input.split_once("\n\n")?;
    let (stacks_str, platforms) = left.rsplit_once('\n')?;

    // parse crates
    let num_stacks = platforms.split_whitespace().last()?.parse().ok()?;
    let mut stacks = vec![Vec::new(); num_stacks];

    for line in stacks_str.lines().rev() {
        for (idx, mut chunk) in line.chars().chunks(4).into_iter().enumerate() {
            let second = chunk.nth(1)?;
            if second.is_alphabetic() {
                stacks[idx].push(second);
            }
        }
    }

    // parse instructions
    let mut instructions = Vec::new();
    for line in instructions_str.lines() {
        let rest = line.strip_prefix("move ")?;
        let (steps, rest) = rest.split_once(" from ")?;
        let (from, to) = rest.split_once(" to ")?;
        let instruction = Command {
            steps: steps.parse().ok()?,
            from: from.parse::<usize>().ok()? - 1,
            to: to.parse::<usize>().ok()? - 1,
        };
        instructions.push(instruction);
    }

    Some((stacks, instructions))
}

pub fn puzzle_a(path: &str) -> String {
    let (mut stacks, commands) = parse_input(path).unwrap();
    for Command { steps, from, to } in commands {
        for _ in 0..steps {
            if let Some(removed) = stacks[from].pop() {
                stacks[to].push(removed);
            }
        }
    }
    stacks
        .iter()
        .map(|s| s.last().unwrap())
        .collect::<String>()
}

pub fn puzzle_b(path: &str) -> String {
    let (mut stacks, commands) = parse_input(path).unwrap();
    for Command { steps, from, to } in commands {
        let from_len = stacks[from].len();
        let removed = stacks[from].split_off(from_len - steps);
        stacks[to].extend(removed);
    }
    stacks
        .iter()
        .map(|s| s.last().unwrap())
        .collect::<String>()
}