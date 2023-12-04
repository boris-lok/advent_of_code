use std::cmp;
use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    idx: usize,
    winning_numbers: HashSet<u8>,
    numbers: HashSet<u8>,
    score: usize,
}

fn parse(input: &str) -> Vec<Card> {
    let lines = input.lines();

    lines
        .into_iter()
        .map(|e| e.split(":").collect::<Vec<_>>())
        .map(|e| {
            let idx = e[0].split(" ").last().unwrap().parse::<usize>().unwrap();
            let rest = e[1].split("|").collect::<Vec<_>>();
            let winning_numbers = rest[0]
                .trim()
                .split(" ")
                .map(|e| e.trim())
                .filter(|e| e.len() > 0)
                .map(|e| e.parse::<u8>().unwrap())
                .collect::<HashSet<_>>();
            let numbers = rest[1]
                .trim()
                .split(" ")
                .map(|e| e.trim())
                .filter(|e| e.len() > 0)
                .map(|e| e.parse::<u8>().unwrap())
                .collect::<HashSet<_>>();

            Card {
                idx,
                winning_numbers,
                numbers,
                score: 1,
            }
        })
        .collect::<Vec<_>>()
}

fn solve_a(input: &str) -> usize {
    let cards = parse(input);
    let collection = cards
        .iter()
        .map(|e| {
            e.winning_numbers
                .intersection(&e.numbers)
                .collect::<Vec<_>>()
        })
        .filter(|e| e.len() > 0)
        .map(|e| 2_usize.pow(e.len() as u32 - 1))
        .collect::<Vec<_>>();

    collection.iter().sum()
}

fn solve_b(input: &str) -> usize {
    let mut cards = parse(input);
    let len = cards.len();

    for idx in 0..len {
        let win = cards[idx]
            .winning_numbers
            .intersection(&cards[idx].numbers)
            .collect::<Vec<_>>();

        if win.len() > 0 && idx + 1 < len {
            for i in (idx + 1)..=cmp::min(idx + win.len(), idx + 1) {
                cards[i].score += cards[idx].score;
            }
        }
    }

    0
}

#[cfg(test)]
mod test {
    use crate::y2023::day4::{solve_a, solve_b};

    #[test]
    fn it_works() {
        let raw = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let predict = crate::y2023::day4::parse(raw);
        println!("{:?}", predict);
        println!("{}", solve_a(raw));
        let ans = 13;
        println!("{}", solve_b(raw));
        assert_eq!(solve_a(raw), ans);
    }
    #[test]
    fn run() {
        let input = std::fs::read_to_string("src/in2023/4.in").unwrap();
        let ans = solve_a(&input);
        println!("{}", ans);
        let ans = solve_b(&input);
        println!("{}", ans);
    }
}
