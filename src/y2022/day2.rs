use std::fs::File;
use std::io::{BufRead, BufReader};

fn calculate_score(your_pick: u8, opponent_pick: u8) -> u8 {
    if your_pick == opponent_pick {
        3
    } else {
        match your_pick as i8 - opponent_pick as i8 {
            1 | -2 => 6,
            _ => 0,
        }
    }
}

fn calculate_what_you_should_pick(require_score: u8, opponent_pick: u8) -> u8 {
    // if require score is 3, it means your pick is same as opponent pick
    if require_score == 3 {
        opponent_pick
    } else {
        // 1. Rock 2. Paper 3. Scissor
        // if require score is 0, it means you lose.
        // So your pick should add 2 on opponent pick.
        // Otherwise, it add 1 on opponent pick when you
        // want to win.
        let temp = opponent_pick + if require_score == 0 { 2 } else { 1 };
        // it will overflow when add 1 or 2
        let temp = temp % 3;
        if temp == 0 {
            3
        } else {
            temp
        }
    }
}

pub fn puzzle_a(path: &str) -> usize {
    let f = File::open(path).unwrap_or_else(|_| panic!("Can't open a path: {}", path));
    let reader = BufReader::new(f);

    reader
        .lines()
        .map(|s| s.unwrap())
        .fold(0_usize, |ans, elem| {
            let elements = elem
                .splitn(2, ' ')
                .map(|e| e.chars().next().unwrap())
                .collect::<Vec<_>>();

            let opponent_pick = elements[0] as u8 - 64;
            let your_pick = elements[1] as u8 - 87;

            ans + your_pick as usize + calculate_score(your_pick, opponent_pick) as usize
        })
}

pub fn puzzle_b(path: &str) -> usize {
    let f = File::open(path).unwrap_or_else(|_| panic!("Can't open a path: {}", path));
    let reader = BufReader::new(f);

    reader
        .lines()
        .map(|s| s.unwrap())
        .fold(0_usize, |ans, elem| {
            let elements = elem
                .splitn(2, ' ')
                .map(|e| e.chars().next().unwrap())
                .collect::<Vec<_>>();

            let opponent_pick = elements[0] as u8 - 64;
            let require_score = (elements[1] as u8 - 88) * 3;

            ans + require_score as usize
                + calculate_what_you_should_pick(require_score, opponent_pick) as usize
        })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calculate_score_worked() {
        assert_eq!(calculate_score(1, 1), 3);
        assert_eq!(calculate_score(1, 2), 0);
        assert_eq!(calculate_score(1, 3), 6);

        assert_eq!(calculate_score(2, 1), 6);
        assert_eq!(calculate_score(2, 2), 3);
        assert_eq!(calculate_score(2, 3), 0);

        assert_eq!(calculate_score(3, 1), 0);
        assert_eq!(calculate_score(3, 2), 6);
        assert_eq!(calculate_score(3, 3), 3);
    }

    #[test]
    fn calculate_pick_worked() {
        assert_eq!(calculate_what_you_should_pick(0, 1), 3);
        assert_eq!(calculate_what_you_should_pick(0, 2), 1);
        assert_eq!(calculate_what_you_should_pick(0, 3), 2);

        assert_eq!(calculate_what_you_should_pick(6, 1), 2);
        assert_eq!(calculate_what_you_should_pick(6, 2), 3);
        assert_eq!(calculate_what_you_should_pick(6, 3), 1);
    }
}
