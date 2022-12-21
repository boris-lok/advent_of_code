// Reference: https://github.com/AxlLind/AdventOfCode2022/blob/main/src/bin/20.rs
pub fn puzzle_a(input: &str) -> isize {
    let data = input
        .lines()
        .map(|e| e.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    let mut ans = (0..data.len()).collect::<Vec<_>>();
    for (i, &x) in data.iter().enumerate() {
        let pos = ans.iter().position(|e| *e == i).unwrap();
        ans.remove(pos);
        let new_idx = (pos as isize + x).rem_euclid(ans.len() as isize) as usize;
        ans.insert(new_idx, i);
    }

    let orig_zero_i = data.iter().position(|&i| i == 0).unwrap();
    let zero_i = ans.iter().position(|&i| i == orig_zero_i).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|i| data[ans[(zero_i + i) % ans.len()]])
        .sum()
}

pub fn puzzle_b(input: &str) -> isize {
    let data = input
        .lines()
        .map(|e| e.parse::<isize>().unwrap() * 811589153)
        .collect::<Vec<_>>();

    let mut ans = (0..data.len()).collect::<Vec<_>>();
    let mut c = 10;
    while c > 0 {
        for (i, &x) in data.iter().enumerate() {
            let pos = ans.iter().position(|e| *e == i).unwrap();
            ans.remove(pos);
            let new_idx = (pos as isize + x).rem_euclid(ans.len() as isize) as usize;
            ans.insert(new_idx, i);
        }

        c -= 1;
    }

    let orig_zero_i = data.iter().position(|&i| i == 0).unwrap();
    let zero_i = ans.iter().position(|&i| i == orig_zero_i).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|i| data[ans[(zero_i + i) % ans.len()]])
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_puzzle_a_works() {
        let input = include_str!("../input/day20.test.in");

        assert_eq!(puzzle_a(input), 3);
    }

    #[test]
    fn test_puzzle_b_works() {
        let input = include_str!("../input/day20.test.in");

        assert_eq!(puzzle_b(input), 1623178306);
    }
}
