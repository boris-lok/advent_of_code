use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn parse_commands(commands: Vec<String>) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    let mut path: Vec<String> = Vec::new();
    let cd_pattern = Regex::new(r"\$ cd ([\s\S]+)").unwrap();
    let file_pattern = Regex::new(r"([0-9]+) ([\s\S]+)").unwrap();

    for command in commands {
        if cd_pattern.is_match(command.as_str()) {
            let location = cd_pattern.captures(command.as_str())
                .unwrap()
                .get(1)
                .unwrap()
                .as_str();

            if location == ".." {
                path.pop();
            } else {
                path.push(location.to_string());
            }
        } else if file_pattern.is_match(command.as_str()) {
            let caps = file_pattern.captures(command.as_str()).unwrap();

            let size = caps.get(1)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();

            for idx in 1..path.len() + 1 {
                let key = path[..idx].join("/");
                let entry = map.entry(key)
                    .or_insert(0);
                *entry += size;
            }
        }
    }

    dbg!(map)
}

pub fn puzzle_a(path: &str) -> usize {
    let f = File::open(path).unwrap_or_else(|_| panic!("Can't open a path: {}", path));
    let reader = BufReader::new(f);

    let map = parse_commands(reader.lines().map(|e| e.unwrap()).collect());

    let mut res = 0_usize;
    for (_, v) in map {
        if v < 100_000 {
            res += v;
        }
    }

    res
}

pub fn puzzle_b(path: &str) -> usize {
    let f = File::open(path).unwrap_or_else(|_| panic!("Can't open a path: {}", path));
    let reader = BufReader::new(f);

    let map = parse_commands(reader.lines().map(|e| e.unwrap()).collect());

    let free_space = 70_000_000 - 30_000_000;
    let total_used_space = map["/"];
    let need_to_free = total_used_space - free_space;

    let mut res = total_used_space;
    for (_, v) in map {
        if v >= need_to_free {
            res = min(v, res);
        }
    }

    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parsing_commands_worked() {
        let data = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

        let d = data.lines().map(|e| e.to_string()).collect::<Vec<_>>();
        let map = parse_commands(d);

        let mut res = 0_usize;
        for (_, v) in map {
            if v < 100_000 {
                res += v;
            }
        }

        assert_eq!(res, 95437);
    }
}