use std::cmp;
use std::collections::HashMap;
#[derive(Debug)]
struct Cubes(u8, u8, u8, u8, bool);

fn parse_line(s: &str) -> Cubes {
    let mut m = HashMap::new();
    m.insert("red", 12);
    m.insert("green", 13);
    m.insert("blue", 14);

    let s = s.split(":").collect::<Vec<_>>();
    let game = s.first().unwrap();
    let rest = s.last().unwrap();
    let idx = game.split(" ").last().unwrap().parse::<u8>().unwrap();
    let group = rest
        .split(";")
        .map(|e| e.split(","))
        .flatten()
        .map(|e| e.trim())
        .collect::<Vec<_>>();

    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    let mut possible = true;

    for g in &group {
        let gg = g.split(" ").collect::<Vec<_>>();
        let c = gg.first().unwrap().parse::<u8>().unwrap();
        let colour = gg.last().unwrap();

        let limit = m.get(colour).unwrap();
        if possible && c > *limit {
            possible = false;
        }

        if *colour == "red" {
            red = cmp::max(c, red);
        }
        if *colour == "green" {
            green = cmp::max(c, green);
        }
        if *colour == "blue" {
            blue = cmp::max(c, blue);
        }
    }

    Cubes(idx, red, green, blue, possible)
}

#[cfg(test)]
mod test {
    use super::parse_line;

    #[test]
    fn parse_line_works() {
        let input = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];

        let expected = input.into_iter().map(parse_line).collect::<Vec<_>>();

        assert!(expected[0].4);
        assert!(expected[1].4);
        assert!(!expected[2].4);
        assert!(!expected[3].4);
        assert!(expected[4].4);
    }

    #[test]
    fn run() {
        let input = std::fs::read_to_string("src/in2023/2.in").unwrap();
        let ans = input.lines().map(parse_line).fold(0u32, |acc, e| {
            return acc + (e.1 as u32 * e.2 as u32 * e.3 as u32);
        });
        println!("{:?}", ans);
    }
}
