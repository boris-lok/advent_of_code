use clap::Parser;


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
    let args = Args::parse();

    match (args.year, args.day) {
        (2022, 1) => {
            use advent_of_code::y2022::day1::{puzzle_a, puzzle_b};
            println!(
                "year: {}, day: {}, puzzle a answer: {}, puzzle b answer: {}",
                args.year,
                args.day,
                puzzle_a(&args.input),
                puzzle_b(&args.input)
            );
        }
        (2022, 2) => {
            use advent_of_code::y2022::day2::{puzzle_a, puzzle_b};
            println!(
                "year: {}, day: {}, puzzle a answer: {}, puzzle b answer: {}",
                args.year,
                args.day,
                puzzle_a(&args.input),
                puzzle_b(&args.input)
            );
        }
        (2022, 3) => {
            use advent_of_code::y2022::day3::{puzzle_a, puzzle_b};
            println!(
                "year: {}, day: {}, puzzle a answer: {}, puzzle b answer: {}",
                args.year,
                args.day,
                puzzle_a(&args.input),
                puzzle_b(&args.input)
            );
        }
        (2022, 4) => {
            use advent_of_code::y2022::day4::{puzzle_a, puzzle_b};
            println!(
                "year: {}, day: {}, puzzle a answer: {}, puzzle b answer: {}",
                args.year,
                args.day,
                puzzle_a(&args.input),
                puzzle_b(&args.input)
            );
        }
        (2022, 5) => {
            use advent_of_code::y2022::day5::{puzzle_a, puzzle_b};
            println!(
                "year: {}, day: {}, puzzle a answer: {}, puzzle b answer: {}",
                args.year,
                args.day,
                puzzle_a(&args.input),
                puzzle_b(&args.input)
            );
        }
        (2022, 6) => {
            use advent_of_code::y2022::day6::{puzzle_a, puzzle_b};
            println!(
                "year: {}, day: {}, puzzle a answer: {}, puzzle b answer: {}",
                args.year,
                args.day,
                puzzle_a(&args.input),
                puzzle_b(&args.input)
            )
        }
        (2022, 7) => {
            use advent_of_code::y2022::day7::{puzzle_a, puzzle_b};
            println!(
                "year: {}, day: {}, puzzle a answer: {}, puzzle b answer: {}",
                args.year,
                args.day,
                puzzle_a(&args.input),
                puzzle_b(&args.input)
            )
        }
        _ => {
            panic!(
                "Can't find the source code year: {}, day: {}",
                args.year, args.day
            )
        }
    };
}
