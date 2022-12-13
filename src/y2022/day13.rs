use std::cmp::Ordering;
use itertools::Itertools;


#[derive(Debug, Eq, PartialEq, Clone)]
enum ListOrInt {
    List(Vec<ListOrInt>),
    Int(isize),
}

impl Ord for ListOrInt {
    fn cmp(&self, other: &Self) -> Ordering {
        use ListOrInt::{Int, List};

        match (self, other) {
            (Int(l), Int(r)) => l.cmp(r),
            (List(l), List(r)) => l.cmp(r),
            (Int(l), List(r)) => [Int(*l)][..].cmp(r),
            (List(l), Int(r)) => (**l).cmp(&[Int(*r)][..])
        }
    }
}

impl PartialOrd for ListOrInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(s: &mut &str) -> ListOrInt {
    if &s[..1] == "[" {
        let mut list = vec![];
        loop {
            *s = &s[1..];
            if &s[..1] == "]" {
                *s = &s[1..];
                return ListOrInt::List(list);
            }
            list.push(parse(s));
            if &s[..1] == "]" {
                *s = &s[1..];
                return ListOrInt::List(list);
            }
        }
    } else {
        let (a, b) = s.split_once([',', ']']).unwrap();
        *s = &s[a.len()..];
        ListOrInt::Int(a.parse().unwrap())
    }
}

pub fn puzzle_a(input: &str) -> usize {
    input
        .split("\n\n")
        .enumerate()
        .fold(0, |ans, (idx, part)| {
            let lines = part.lines().collect::<Vec<_>>();
            let left = parse(&mut { lines[0] });
            let right = parse(&mut { lines[1] });
            ans + if left < right {
                idx + 1
            } else {
                0
            }
        })
}


pub fn puzzle_b(input: &str) -> usize {
    let extra_package_a = parse(&mut "[[2]]");
    let extra_package_b = parse(&mut "[[6]]");

    let mut packages = input.split("\n\n")
        .fold(vec![
            extra_package_a.clone(),
            extra_package_b.clone(),
        ], |mut list, part| {
            let lines = part.lines().collect::<Vec<_>>();
            let left = parse(&mut { lines[0] });
            let right = parse(&mut { lines[1] });
            list.push(left);
            list.push(right);
            list
        });

    packages.sort();

    let index_a = packages.iter().position(|e| e == &extra_package_a).unwrap();
    let index_b = packages.iter().position(|e| e == &extra_package_b).unwrap();
    (index_a + 1) * (index_b + 1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution_works() {
        assert_eq!(puzzle_a(include_str!("../input/day13.test.in")), 13);
        assert_eq!(puzzle_b(include_str!("../input/day13.test.in")), 140);
    }
}