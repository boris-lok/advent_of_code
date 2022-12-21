use std::collections::HashMap;

#[derive(Debug)]
enum Operation {
    Add(String, String),
    Minus(String, String),
    Multiple(String, String),
    Div(String, String),
}

#[derive(Debug)]
enum MonkeyOperation {
    Int(isize),
    Action(Operation),
}

pub fn puzzle_a(input: &str) -> isize {
    let mut map = HashMap::new();

    for line in input.lines() {
        let mut part = line.split(": ");
        let key = part.next().unwrap();
        let rest = part.next().unwrap();
        let operation = if let Ok(n) = rest.parse::<isize>() {
            MonkeyOperation::Int(n)
        } else {
            let mut part = rest.split(" ");
            let k1 = part.next().unwrap().to_string();
            let o = part.next().unwrap();
            let k2 = part.next().unwrap().to_string();

            MonkeyOperation::Action(match o {
                "+" => Operation::Add(k1, k2),
                "-" => Operation::Minus(k1, k2),
                "*" => Operation::Multiple(k1, k2),
                "/" => Operation::Div(k1, k2),
                _ => unreachable!(),
            })
        };
        map.insert(key, operation);
    }

    let root = map.get("root").unwrap();
    calculate(root, &map)
}

pub fn puzzle_b(input: &str) -> isize {
    let mut map = HashMap::new();

    for line in input.lines() {
        let mut part = line.split(": ");
        let key = part.next().unwrap();
        if key == "humn" {
            continue;
        }
        let rest = part.next().unwrap();
        let operation = if let Ok(n) = rest.parse::<isize>() {
            MonkeyOperation::Int(n)
        } else {
            let mut part = rest.split(" ");
            let k1 = part.next().unwrap().to_string();
            let o = part.next().unwrap();
            let k2 = part.next().unwrap().to_string();

            MonkeyOperation::Action(match o {
                "+" => Operation::Add(k1, k2),
                "-" => Operation::Minus(k1, k2),
                "*" => Operation::Multiple(k1, k2),
                "/" => Operation::Div(k1, k2),
                _ => unreachable!(),
            })
        };
        map.insert(key, operation);
    }

    // let c1 = map.get("pppw").unwrap();
    // let c2 = map.get("sjmn").unwrap();
    // let mut c1 = dbg!(calculate_2(c1, &map));
    // let mut c2 = dbg!(calculate_2(c2, &map));
    //
    // if c1.is_none() {
    //     c1 = c2;
    // }
    // if c2.is_none() {
    //     c2 = c1;
    // }

    // map.insert("pppw", MonkeyOperation::Int(150));

    // c1.unwrap() + c2.unwrap()
    let mut map2 = HashMap::new();

    for (k, v) in map.into_iter() {
        match v {
            MonkeyOperation::Int(_) => {
                map2.insert(k.to_string(), v);
            }
            MonkeyOperation::Action(action) => match action {
                Operation::Add(k1, k2) => {
                    if k1 == "humn".to_string() {
                        map2.insert(
                            k1,
                            MonkeyOperation::Action(Operation::Minus(k.to_string(), k2)),
                        );
                    } else if k2 == "humn".to_string() {
                        map2.insert(
                            k2,
                            MonkeyOperation::Action(Operation::Minus(k.to_string(), k1)),
                        );
                    } else {
                        map2.insert(
                            k.to_string(),
                            MonkeyOperation::Action(Operation::Add(k1, k2)),
                        );
                    }
                }
                Operation::Minus(k1, k2) => {
                    if k1 == "humn".to_string() {
                        map2.insert(
                            k1,
                            MonkeyOperation::Action(Operation::Add(k.to_string(), k2)),
                        );
                    } else if k2 == "humn".to_string() {
                        map2.insert(
                            k2,
                            MonkeyOperation::Action(Operation::Add(k.to_string(), k1)),
                        );
                    } else {
                        map2.insert(
                            k.to_string(),
                            MonkeyOperation::Action(Operation::Minus(k1, k2)),
                        );
                    }
                }
                Operation::Multiple(k1, k2) => {
                    if k1 == "humn".to_string() {
                        map2.insert(
                            k1,
                            MonkeyOperation::Action(Operation::Div(k.to_string(), k2)),
                        );
                    } else if k2 == "humn".to_string() {
                        map2.insert(
                            k2,
                            MonkeyOperation::Action(Operation::Div(k.to_string(), k1)),
                        );
                    } else {
                        map2.insert(
                            k.to_string(),
                            MonkeyOperation::Action(Operation::Multiple(k1, k2)),
                        );
                    }
                }
                Operation::Div(k1, k2) => {
                    if k1 == "humn".to_string() {
                        map2.insert(
                            k1,
                            MonkeyOperation::Action(Operation::Multiple(k.to_string(), k2)),
                        );
                    } else if k2 == "humn".to_string() {
                        map2.insert(
                            k2,
                            MonkeyOperation::Action(Operation::Multiple(k.to_string(), k1)),
                        );
                    } else {
                        map2.insert(
                            k.to_string(),
                            MonkeyOperation::Action(Operation::Div(k1, k2)),
                        );
                    }
                }
            },
        }
    }

    map2.insert("pppw".to_string(), MonkeyOperation::Int(150));

    let root = map2.get("humn").unwrap();
    let ans = dbg!(calculate_3(root, &map2));

    0
}

fn calculate_2(cur: &MonkeyOperation, map: &HashMap<&str, MonkeyOperation>) -> Option<isize> {
    match cur {
        MonkeyOperation::Int(n) => {
            return Some(*n);
        }
        MonkeyOperation::Action(operation) => match operation {
            Operation::Add(k1, k2) => {
                let m1 = map.get(k1.as_str());
                let m2 = map.get(k2.as_str());
                if m1.is_none() || m2.is_none() {
                    return None;
                }
                let c1 = calculate_2(m1.unwrap(), map);
                let c2 = calculate_2(m2.unwrap(), map);
                if c1.is_some() && c2.is_some() {
                    return Some(c1.unwrap() + c2.unwrap());
                } else {
                    None
                }
            }
            Operation::Minus(k1, k2) => {
                let m1 = map.get(k1.as_str());
                let m2 = map.get(k2.as_str());
                if m1.is_none() || m2.is_none() {
                    return None;
                }
                let c1 = calculate_2(m1.unwrap(), map);
                let c2 = calculate_2(m2.unwrap(), map);
                if c1.is_some() && c2.is_some() {
                    return Some(c1.unwrap() - c2.unwrap());
                } else {
                    None
                }
            }
            Operation::Multiple(k1, k2) => {
                let m1 = map.get(k1.as_str());
                let m2 = map.get(k2.as_str());
                if m1.is_none() || m2.is_none() {
                    return None;
                }
                let c1 = calculate_2(m1.unwrap(), map);
                let c2 = calculate_2(m2.unwrap(), map);
                if c1.is_some() && c2.is_some() {
                    return Some(c1.unwrap() * c2.unwrap());
                } else {
                    None
                }
            }
            Operation::Div(k1, k2) => {
                let m1 = map.get(k1.as_str());
                let m2 = map.get(k2.as_str());
                if m1.is_none() || m2.is_none() {
                    return None;
                }
                let c1 = calculate_2(m1.unwrap(), map);
                let c2 = calculate_2(m2.unwrap(), map);
                if c1.is_some() && c2.is_some() {
                    return Some(c1.unwrap() / c2.unwrap());
                } else {
                    None
                }
            }
        },
    }
}

fn calculate(cur: &MonkeyOperation, map: &HashMap<&str, MonkeyOperation>) -> isize {
    match cur {
        MonkeyOperation::Int(n) => {
            return *n;
        }
        MonkeyOperation::Action(operation) => match operation {
            Operation::Add(k1, k2) => {
                let m1 = map.get(k1.as_str()).unwrap();
                let m2 = map.get(k2.as_str()).unwrap();
                calculate(m1, map) + calculate(m2, map)
            }
            Operation::Minus(k1, k2) => {
                let m1 = map.get(k1.as_str()).unwrap();
                let m2 = map.get(k2.as_str()).unwrap();
                calculate(m1, map) - calculate(m2, map)
            }
            Operation::Multiple(k1, k2) => {
                let m1 = map.get(k1.as_str()).unwrap();
                let m2 = map.get(k2.as_str()).unwrap();
                calculate(m1, map) * calculate(m2, map)
            }
            Operation::Div(k1, k2) => {
                let m1 = map.get(k1.as_str()).unwrap();
                let m2 = map.get(k2.as_str()).unwrap();
                calculate(m1, map) / calculate(m2, map)
            }
        },
    }
}

fn calculate_3(cur: &MonkeyOperation, map: &HashMap<String, MonkeyOperation>) -> isize {
    match cur {
        MonkeyOperation::Int(n) => {
            return *n;
        }
        MonkeyOperation::Action(operation) => match operation {
            Operation::Add(k1, k2) => {
                dbg!(&k1);
                let m1 = map.get(k1.as_str()).unwrap();
                let m2 = map.get(k2.as_str()).unwrap();
                calculate_3(m1, map) + calculate_3(m2, map)
            }
            Operation::Minus(k1, k2) => {
                let m1 = map.get(k1.as_str()).unwrap();
                let m2 = map.get(k2.as_str()).unwrap();
                calculate_3(m1, map) - calculate_3(m2, map)
            }
            Operation::Multiple(k1, k2) => {
                let m1 = map.get(k1.as_str()).unwrap();
                let m2 = map.get(k2.as_str()).unwrap();
                calculate_3(m1, map) * calculate_3(m2, map)
            }
            Operation::Div(k1, k2) => {
                let m1 = map.get(k1.as_str()).unwrap();
                let m2 = map.get(k2.as_str()).unwrap();
                calculate_3(m1, map) / calculate_3(m2, map)
            }
        },
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_puzzle_a_works() {
        let input = include_str!("../input/day21.test.in");

        assert_eq!(puzzle_a(input), 152);
    }

    #[test]
    fn test_puzzle_b_works() {
        let input = include_str!("../input/day21.test.in");

        assert_eq!(puzzle_b(input), 301);
    }
}
