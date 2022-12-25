fn to_decimal(s: &str) -> i64 {
    s.lines()
        .into_iter()
        .map(|line| {
            line.chars().fold(0, |ans, c| {
                5 * ans
                    + match c {
                        '-' => -1,
                        '=' => -2,
                        _ => c.to_digit(3).unwrap() as i64,
                    }
            })
        })
        .sum()
}

fn to_snafu(n: i64) -> String {
    let mut t = n;

    let mut res = vec![];
    let sanfu = ['=', '-', '0', '1', '2'];

    while t != 0 {
        res.push(sanfu[(t + 2).rem_euclid(5) as usize]);
        t = (t + 2).div_euclid(5);
    }

    res.iter().rev().collect()
}

pub fn puzzle_a(input: &str) -> String {
    let n = to_decimal(input);
    dbg!(n);
    to_snafu(n)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_puzzle_a_works() {
        let input = include_str!("../input/day25.test.in");
        assert_eq!(puzzle_a(input), "2=-1=0".to_string());
    }
}
