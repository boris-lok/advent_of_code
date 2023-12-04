use std::collections::HashSet;

#[derive(Debug)]
struct PartNumber {
    value: usize,
    points: HashSet<(i64, i64)>,
}

impl PartNumber {
    fn new(row: i64, col: i64, ch: char) -> Self {
        let points = HashSet::from([
            (row - 1, col - 1),
            (row, col - 1),
            (row + 1, col - 1), //left hand side
            (row - 1, col),
            (row + 1, col), //above and below
            (row - 1, col + 1),
            (row, col + 1),
            (row + 1, col + 1), //right hand side
        ]);
        Self {
            value: (ch as u8 - b'0') as usize,
            points,
        }
    }

    fn add_digit(&mut self, row: i64, col: i64, ch: char) {
        self.value = self.value * 10 + (ch as u8 - b'0') as usize;
        self.points
            .extend([(row - 1, col + 1), (row, col + 1), (row + 1, col + 1)]);
    }
}

#[derive(Debug)]
struct Data {
    values: Vec<PartNumber>,
    symbols: HashSet<(i64, i64)>,
    gears: HashSet<(i64, i64)>,
}

impl Data {
    fn new() -> Self {
        Self {
            values: vec![],
            symbols: HashSet::new(),
            gears: HashSet::new(),
        }
    }
}

fn parse(input: &str) -> Data {
    let lines = input.lines();
    let mut data = Data::new();

    let mut cur_num: Option<PartNumber> = None;
    for (row, line) in lines.into_iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch.is_ascii_digit() {
                if let Some(ref mut num) = cur_num {
                    num.add_digit(row as i64, col as i64, ch);
                } else {
                    cur_num = Some(PartNumber::new(row as i64, col as i64, ch))
                }
            } else {
                if let Some(num) = cur_num.take() {
                    data.values.push(num);
                }
                if ch != '.' {
                    data.symbols.insert((row as i64, col as i64));
                    if ch == '*' {
                        data.gears.insert((row as i64, col as i64));
                    }
                }
            }
        }
        if let Some(num) = cur_num.take() {
            data.values.push(num);
        }
    }

    data
}

fn solve_a(input: &str) -> usize {
    let data = parse(input);

    let total = data
        .values
        .iter()
        .filter(|e| e.points.intersection(&data.symbols).next().is_some())
        .map(|e| e.value)
        .sum::<usize>();

    total
}

fn solve_b(input: &str) -> usize {
    let data = parse(input);
    let mut total = 0;

    'next_gear: for gear in &data.gears {
        let mut matches = Vec::new();
        for num in &data.values {
            if num.points.contains(gear) {
                if matches.len() == 2 {
                    continue 'next_gear;
                }
                matches.push(num.value);
            }
        }

        if matches.len() == 2 {
            total += matches[0] * matches[1];
        }
    }

    total
}

#[cfg(test)]
mod test {
    use super::{solve_a, solve_b};

    #[test]
    fn solve_a_should_work() {
        let raw = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let ans = 4361_usize;
        let predict = solve_a(raw);
        assert_eq!(predict, ans);
    }

    #[test]
    fn run() {
        let input = std::fs::read_to_string("src/in2023/3.in").unwrap();
        let ans = solve_a(&input);
        println!("{:?}", ans);
        let ans = solve_b(&input);
        println!("{:?}", ans);
    }
}
