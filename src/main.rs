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
            use advent_of_code::y2022::{puzzle_a, puzzle_b};
            println!(
                "year: {}, day: {}, puzzle a answer: {}",
                args.year,
                args.day,
                puzzle_a(&args.input)
            );
            println!(
                "year: {}, day: {}, puzzle b answer: {}",
                args.year,
                args.day,
                puzzle_b(&args.input)
            );
        }
        _ => panic!(
            "Can't find the source code year: {}, day: {}",
            args.year, args.day
        ),
    };
}
