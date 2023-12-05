use std::collections::HashSet;
use clap::Parser;
use itertools::Itertools;
use advent_of_code::y2023::day5::parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    year: u16,
    #[arg(short, long)]
    day: u8,
    #[arg(short, long)]
    input: String,
}

fn main() {
    use advent_of_code::y2023::day5::parser;

    let input = std::fs::read_to_string("./src/in2023/5.in").unwrap();

    let data = parser(&input);
    let mut seeds = HashSet::new();

    for (b, c) in data.seeds.iter().tuples() {
        let mut i = 0;
        while i < *c {
            seeds.insert(b + i);
            i += 1;
        }
    }

    let s = Vec::from_iter(seeds.into_iter());
    dbg!(&s.len());

    // let args = Args::parse();
    //
    // match (args.year, args.day) {
    //     (2022, 1) => {
    //         use advent_of_code::y2022::day1::{puzzle_a, puzzle_b};
    //         println!(
    //             "year: {}, day: {}, puzzle a answer: {}, puzzle b answer: {}",
    //             args.year,
    //             args.day,
    //             puzzle_a(&args.input),
    //             puzzle_b(&args.input)
    //         );
    //     }
    //     (2022, 2) => {
    //         use advent_of_code::y2022::day2::{puzzle_a, puzzle_b};
    //         println!(
    //             "year: {}, day: {}, puzzle a answer: {}, puzzle b answer: {}",
    //             args.year,
    //             args.day,
    //             puzzle_a(&args.input),
    //             puzzle_b(&args.input)
    //         );
    //     }
    //     (2022, 3) => {
    //         use advent_of_code::y2022::day3::{puzzle_a, puzzle_b};
    //         println!(
    //             "year: {}, day: {}, puzzle a answer: {}, puzzle b answer: {}",
    //             args.year,
    //             args.day,
    //             puzzle_a(&args.input),
    //             puzzle_b(&args.input)
    //         );
    //     }
    //     (2022, 4) => {
    //         use advent_of_code::y2022::day4::{puzzle_a, puzzle_b};
    //         println!(
    //             "year: {}, day: {}, puzzle a answer: {}, puzzle b answer: {}",
    //             args.year,
    //             args.day,
    //             puzzle_a(&args.input),
    //             puzzle_b(&args.input)
    //         );
    //     }
    //     (2022, 5) => {
    //         use advent_of_code::y2022::day5::{puzzle_a, puzzle_b};
    //         println!(
    //             "year: {}, day: {}, puzzle a answer: {}, puzzle b answer: {}",
    //             args.year,
    //             args.day,
    //             puzzle_a(&args.input),
    //             puzzle_b(&args.input)
    //         );
    //     }
    //     (2022, 6) => {
    //         use advent_of_code::y2022::day6::{puzzle_a, puzzle_b};
    //         println!(
    //             "year: {}, day: {}, puzzle a answer: {}, puzzle b answer: {}",
    //             args.year,
    //             args.day,
    //             puzzle_a(&args.input),
    //             puzzle_b(&args.input)
    //         )
    //     }
    //     (2022, 7) => {
    //         use advent_of_code::y2022::day7::{puzzle_a, puzzle_b};
    //         println!(
    //             "year: {}, day: {}, puzzle a answer: {}, puzzle b answer: {}",
    //             args.year,
    //             args.day,
    //             puzzle_a(&args.input),
    //             puzzle_b(&args.input)
    //         )
    //     }
    //     (2022, 8) => {
    //         use advent_of_code::y2022::day8::{puzzle_a, puzzle_b};
    //         println!(
    //             "year: {}, day: {}, puzzle a answer: {}, puzzle b answer: {}",
    //             args.year,
    //             args.day,
    //             puzzle_a(&args.input),
    //             puzzle_b(&args.input)
    //         )
    //     }
    //     (2022, 9) => {
    //         use advent_of_code::y2022::day9::{puzzle_a, puzzle_b};
    //         println!(
    //             "year: {}, day: {}, puzzle a answer: {}, puzzle b answer: {}",
    //             args.year,
    //             args.day,
    //             puzzle_a(&args.input),
    //             puzzle_b(&args.input)
    //         )
    //     }
    //     (2022, 10) => {
    //         use advent_of_code::y2022::day10::{puzzle_a, puzzle_b};
    //         println!(
    //             "year: {}, day: {}, puzzle a answer: {}, puzzle b answer: {}",
    //             args.year,
    //             args.day,
    //             puzzle_a(&args.input),
    //             puzzle_b(&args.input)
    //         )
    //     }
    //     (2022, 11) => {
    //         use advent_of_code::y2022::day11::{puzzle_a, puzzle_b};
    //         println!(
    //             "year: {}, day: {}, puzzle a answer: {}, puzzle b answer: {}",
    //             args.year,
    //             args.day,
    //             puzzle_a(&args.input),
    //             puzzle_b(&args.input)
    //         )
    //     }
    //     (2022, 12) => {
    //         use advent_of_code::y2022::day12::{puzzle_a, puzzle_b};
    //         println!(
    //             "year: {}, day: {}, puzzle a answer: {}, puzzle b answer: {}",
    //             args.year,
    //             args.day,
    //             puzzle_a(&args.input),
    //             puzzle_b(&args.input)
    //         )
    //     }
    //     (2022, 13) => {
    //         use advent_of_code::y2022::day13::{puzzle_a, puzzle_b};
    //         println!(
    //             "year: {}, day: {}, puzzle a answer: {}, puzzle b answer: {}",
    //             args.year,
    //             args.day,
    //             puzzle_a(&std::fs::read_to_string(&args.input).unwrap()),
    //             puzzle_b(&std::fs::read_to_string(&args.input).unwrap())
    //         )
    //     }
    //     (2022, 14) => {
    //         use advent_of_code::y2022::day14::{puzzle_a, puzzle_b};
    //         println!(
    //             "year: {}, day: {}, puzzle a answer: {}, puzzle b answer: {}",
    //             args.year,
    //             args.day,
    //             puzzle_a(&std::fs::read_to_string(&args.input).unwrap()),
    //             puzzle_b(&std::fs::read_to_string(&args.input).unwrap())
    //         )
    //     }
    //     (2022, 15) => {
    //         use advent_of_code::y2022::day15::{puzzle_a, puzzle_b};
    //         println!(
    //             "year: {}, day: {}, puzzle a answer: {}, puzzle b answer: {}",
    //             args.year,
    //             args.day,
    //             puzzle_a(&std::fs::read_to_string(&args.input).unwrap(), 10),
    //             puzzle_b(&std::fs::read_to_string(&args.input).unwrap(), 4_000_000)
    //         )
    //     }
    //     (2022, 16) => {
    //         use advent_of_code::y2022::day16::{puzzle_a, puzzle_b};
    //         println!(
    //             "year: {}, day: {}, puzzle a answer: {}, puzzle b answer: {}",
    //             args.year,
    //             args.day,
    //             puzzle_a(&std::fs::read_to_string(&args.input).unwrap()),
    //             puzzle_b(&std::fs::read_to_string(&args.input).unwrap())
    //         )
    //     }
    //     (2022, 17) => {
    //         use advent_of_code::y2022::day17::{puzzle_a, puzzle_b};
    //         println!(
    //             "year: {}, day: {}, puzzle a answer: {}, puzzle b answer: {}",
    //             args.year,
    //             args.day,
    //             puzzle_a(&std::fs::read_to_string(&args.input).unwrap()),
    //             puzzle_b(&std::fs::read_to_string(&args.input).unwrap())
    //         )
    //     }
    //     (2022, 18) => {
    //         use advent_of_code::y2022::day18::{puzzle_a, puzzle_b};
    //         println!(
    //             "year: {}, day: {}, puzzle a answer: {}, puzzle b answer: {}",
    //             args.year,
    //             args.day,
    //             puzzle_a(&std::fs::read_to_string(&args.input).unwrap()),
    //             puzzle_b(&std::fs::read_to_string(&args.input).unwrap())
    //         )
    //     }
    //     (2022, 19) => {
    //         use advent_of_code::y2022::day19::{puzzle_a, puzzle_b};
    //         println!(
    //             "year: {}, day: {}, puzzle a answer: {}, puzzle b answer: {}",
    //             args.year,
    //             args.day,
    //             puzzle_a(&std::fs::read_to_string(&args.input).unwrap(), 24),
    //             puzzle_b(&std::fs::read_to_string(&args.input).unwrap(), 32)
    //         )
    //     }
    //     (2022, 20) => {
    //         use advent_of_code::y2022::day20::{puzzle_a, puzzle_b};
    //         println!(
    //             "year: {}, day: {}, puzzle a answer: {}, puzzle b answer: {}",
    //             args.year,
    //             args.day,
    //             puzzle_a(&std::fs::read_to_string(&args.input).unwrap()),
    //             puzzle_b(&std::fs::read_to_string(&args.input).unwrap())
    //         )
    //     }
    //     _ => {
    //         panic!(
    //             "Can't find the source code year: {}, day: {}",
    //             args.year, args.day
    //         )
    //     }
    // };
}
