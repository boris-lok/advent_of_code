use std::collections::HashMap;

#[derive(Debug)]
enum Op {
    Add(String, String),
    Minus(String, String),
    Multiple(String, String),
    Div(String, String),
}

#[derive(Debug)]
enum MonkeyOperation {
    Int(isize),
    Action(Op),
}

fn parse_input(input: &str) -> (HashMap<String, isize>, HashMap<String, MonkeyOperation>) {
    let mut resolved = HashMap::new();
    let mut unresolved = HashMap::new();

    for line in input.lines() {
        let mut part = line.split(": ");
        let key = part.next().unwrap();
        let rest = part.next().unwrap();
        if let Ok(n) = rest.parse::<isize>() {
            resolved.insert(key.to_string(), n);
        } else {
            let mut part = rest.split(" ");
            let k1 = part.next().unwrap().to_string();
            let o = part.next().unwrap();
            let k2 = part.next().unwrap().to_string();

            let op = MonkeyOperation::Action(match o {
                "+" => Op::Add(k1, k2),
                "-" => Op::Minus(k1, k2),
                "*" => Op::Multiple(k1, k2),
                "/" => Op::Div(k1, k2),
                _ => unreachable!(),
            });
            unresolved.insert(key.to_string(), op);
        };
    }

    (resolved, unresolved)
}

pub fn puzzle_a(input: &str) -> isize {
    let (mut resolved, mut unresolved) = parse_input(input);

    while !unresolved.is_empty() {
        let can_solved = unresolved
            .iter()
            .filter(|(key, val)| {
                let (k1, k2) = match val {
                    MonkeyOperation::Int(_) => unreachable!(),
                    MonkeyOperation::Action(op) => match op {
                        Op::Add(k1, k2) => (k1, k2),
                        Op::Minus(k1, k2) => (k1, k2),
                        Op::Multiple(k1, k2) => (k1, k2),
                        Op::Div(k1, k2) => (k1, k2),
                    },
                };

                resolved.contains_key(k1.as_str()) && resolved.contains_key(k2.as_str())
            })
            .map(|(k, _)| k.to_string())
            .collect::<Vec<_>>();

        for k in can_solved.iter() {
            let solved = unresolved.get(k.as_str());
            if let Some(op) = solved {
                let res = match op {
                    MonkeyOperation::Int(_) => unreachable!(),
                    MonkeyOperation::Action(op) => match op {
                        Op::Add(k1, k2) => {
                            let n1 = resolved.get(k1.as_str()).unwrap();
                            let n2 = resolved.get(k2.as_str()).unwrap();
                            n1 + n2
                        }
                        Op::Minus(k1, k2) => {
                            let n1 = resolved.get(k1.as_str()).unwrap();
                            let n2 = resolved.get(k2.as_str()).unwrap();
                            n1 - n2
                        }
                        Op::Multiple(k1, k2) => {
                            let n1 = resolved.get(k1.as_str()).unwrap();
                            let n2 = resolved.get(k2.as_str()).unwrap();
                            n1 * n2
                        }
                        Op::Div(k1, k2) => {
                            let n1 = resolved.get(k1.as_str()).unwrap();
                            let n2 = resolved.get(k2.as_str()).unwrap();
                            n1 / n2
                        }
                    },
                };

                unresolved.remove(k.as_str());
                resolved.insert(k.to_string(), res);
            }
        }
    }

    *resolved.get("root").unwrap()
}

pub fn puzzle_b(input: &str) -> isize {
    let (mut resolved, mut unresolved) = parse_input(input);
    resolved.remove("humn");
    let root = unresolved.remove("root").unwrap();

    loop {
        let can_solved = unresolved
            .iter()
            .filter(|(key, val)| {
                let (k1, k2) = match val {
                    MonkeyOperation::Int(_) => unreachable!(),
                    MonkeyOperation::Action(op) => match op {
                        Op::Add(k1, k2) => (k1, k2),
                        Op::Minus(k1, k2) => (k1, k2),
                        Op::Multiple(k1, k2) => (k1, k2),
                        Op::Div(k1, k2) => (k1, k2),
                    },
                };

                resolved.contains_key(k1.as_str()) && resolved.contains_key(k2.as_str())
            })
            .map(|(k, _)| k.to_string())
            .collect::<Vec<_>>();

        if can_solved.is_empty() {
            break;
        }

        for k in can_solved.iter() {
            let solved = unresolved.get(k.as_str());
            if let Some(op) = solved {
                let res = match op {
                    MonkeyOperation::Int(_) => unreachable!(),
                    MonkeyOperation::Action(op) => match op {
                        Op::Add(k1, k2) => {
                            let n1 = resolved.get(k1.as_str()).unwrap();
                            let n2 = resolved.get(k2.as_str()).unwrap();
                            n1 + n2
                        }
                        Op::Minus(k1, k2) => {
                            let n1 = resolved.get(k1.as_str()).unwrap();
                            let n2 = resolved.get(k2.as_str()).unwrap();
                            n1 - n2
                        }
                        Op::Multiple(k1, k2) => {
                            let n1 = resolved.get(k1.as_str()).unwrap();
                            let n2 = resolved.get(k2.as_str()).unwrap();
                            n1 * n2
                        }
                        Op::Div(k1, k2) => {
                            let n1 = resolved.get(k1.as_str()).unwrap();
                            let n2 = resolved.get(k2.as_str()).unwrap();
                            n1 / n2
                        }
                    },
                };

                unresolved.remove(k.as_str());
                resolved.insert(k.to_string(), res);
            }
        }
    }

    match root {
        MonkeyOperation::Int(_) => unreachable!(),
        MonkeyOperation::Action(op) => match op {
            Op::Add(k1, k2) => {
                let r1 = resolved.get(k1.as_str());
                let r2 = resolved.get(k2.as_str());
                if let Some(r1) = r1 {
                    resolved.insert(k2, *r1);
                } else if let Some(r2) = r2 {
                    resolved.insert(k1, *r2);
                }
            }
            Op::Minus(k1, k2) => {
                let r1 = resolved.get(k1.as_str());
                let r2 = resolved.get(k2.as_str());
                if let Some(r1) = r1 {
                    resolved.insert(k2, *r1);
                } else if let Some(r2) = r2 {
                    resolved.insert(k1, *r2);
                }
            }
            Op::Multiple(k1, k2) => {
                let r1 = resolved.get(k1.as_str());
                let r2 = resolved.get(k2.as_str());
                if let Some(r1) = r1 {
                    resolved.insert(k2, *r1);
                } else if let Some(r2) = r2 {
                    resolved.insert(k1, *r2);
                }
            }
            Op::Div(k1, k2) => {
                let r1 = resolved.get(k1.as_str());
                let r2 = resolved.get(k2.as_str());
                if let Some(r1) = r1 {
                    resolved.insert(k2, *r1);
                } else if let Some(r2) = r2 {
                    resolved.insert(k1, *r2);
                }
            }
        },
    }

    loop {
        let can_solved = unresolved
            .iter()
            .filter(|(key, val)| {
                let (k1, k2) = match val {
                    MonkeyOperation::Int(_) => unreachable!(),
                    MonkeyOperation::Action(op) => match op {
                        Op::Add(k1, k2) => (k1, k2),
                        Op::Minus(k1, k2) => (k1, k2),
                        Op::Multiple(k1, k2) => (k1, k2),
                        Op::Div(k1, k2) => (k1, k2),
                    },
                };

                resolved.contains_key(key.as_str())
                    && (resolved.contains_key(k1.as_str()) || resolved.contains_key(k2.as_str()))
            })
            .map(|(k, _)| k.to_string())
            .collect::<Vec<_>>();

        if can_solved.is_empty() {
            break;
        }

        for k in can_solved.iter() {
            let solved = unresolved.get(k.as_str());
            if let Some(op) = solved {
                match op {
                    MonkeyOperation::Int(_) => unreachable!(),
                    MonkeyOperation::Action(op) => match op {
                        Op::Add(k1, k2) => {
                            let n = resolved.get(k.as_str()).unwrap();
                            let n1 = resolved.get(k1.as_str());
                            let n2 = resolved.get(k2.as_str());
                            if let Some(n1) = n1 {
                                resolved.insert(k2.to_string(), n - n1);
                            } else if let Some(n2) = n2 {
                                resolved.insert(k1.to_string(), n - n2);
                            }
                        }
                        Op::Minus(k1, k2) => {
                            let n = resolved.get(k.as_str()).unwrap();
                            let n1 = resolved.get(k1.as_str());
                            let n2 = resolved.get(k2.as_str());
                            if let Some(n1) = n1 {
                                resolved.insert(k2.to_string(), n1 - n);
                            } else if let Some(n2) = n2 {
                                resolved.insert(k1.to_string(), n + n2);
                            }
                        }
                        Op::Multiple(k1, k2) => {
                            let n = resolved.get(k.as_str()).unwrap();
                            let n1 = resolved.get(k1.as_str());
                            let n2 = resolved.get(k2.as_str());
                            if let Some(n1) = n1 {
                                resolved.insert(k2.to_string(), n / n1);
                            } else if let Some(n2) = n2 {
                                resolved.insert(k1.to_string(), n / n2);
                            }
                        }
                        Op::Div(k1, k2) => {
                            let n = resolved.get(k.as_str()).unwrap();
                            let n1 = resolved.get(k1.as_str());
                            let n2 = resolved.get(k2.as_str());
                            if let Some(n1) = n1 {
                                resolved.insert(k2.to_string(), n * n1);
                            } else if let Some(n2) = n2 {
                                resolved.insert(k1.to_string(), n * n2);
                            }
                        }
                    },
                }

                unresolved.remove(k.as_str());
            }
        }
    }

    *resolved.get("humn").unwrap()
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
