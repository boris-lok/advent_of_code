#[derive(Debug, Eq, PartialEq, Clone)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Value {
    Old,
    Value(isize),
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Monkey {
    id: u8,
    operation: Operation,
    value: Value,
    worries: Vec<isize>,
    test: isize,
    if_true: u8,
    if_false: u8,
    inspected: usize,
}

fn parse_input<'a, 'b>(input: &'a str) -> Vec<Monkey> {
    let group = input
        .split("\n\n");

    let monkeys = group
        .map(|g| {
            let mut line = g.split("\n");

            let id = line.next().unwrap()
                .split(" ")
                .nth(1)
                .unwrap();

            let id = id.split(":")
                .nth(0)
                .unwrap()
                .parse::<u8>()
                .expect("Can't parse id to u8");

            let parse_line = |e: Option<&'a str>, pat: &'b str| -> &'a str {
                e
                    .unwrap()
                    .split(" ")
                    .last()
                    .unwrap()
            };

            let worries = line
                .next()
                .unwrap()
                .split(": ")
                .nth(1)
                .unwrap()
                .split(", ")
                .map(|e| e.parse::<isize>().unwrap())
                .collect::<Vec<_>>();

            let part = line
                .next()
                .unwrap()
                .split(" ")
                .collect::<Vec<_>>();

            let value = part.last()
                .unwrap()
                .parse::<isize>()
                .map(|e| Value::Value(e))
                .unwrap_or_else(|_| Value::Old);

            let operation = part[part.len() - 2];

            let operation = match operation {
                "+" => Operation::Add,
                "*" => Operation::Multiply,
                _ => unreachable!(),
            };

            let test = parse_line(line.next(), " ")
                .parse::<isize>()
                .unwrap();

            let if_true = parse_line(line.next(), " ")
                .parse::<u8>()
                .unwrap();

            let if_false = parse_line(line.next(), " ")
                .parse::<u8>()
                .unwrap();


            Monkey {
                id,
                operation,
                value,
                worries,
                test,
                if_true,
                if_false,
                inspected: 0,
            }
        })
        .collect::<Vec<_>>();
    monkeys
}

fn calculate_worry<F>(mut monkeys: Vec<Monkey>, f: F, iteration: usize) -> usize
    where F: FnOnce(isize) -> isize + Copy {
    for _ in 0..iteration {
        for idx in 0..monkeys.len() {
            let res = {
                let monkey = &mut monkeys[idx];
                let mut ids = vec![];
                monkey.inspected += monkey.worries.len();

                for _ in 0..monkey.worries.len() {
                    let worry = monkey.worries.pop().unwrap();
                    let res = match monkey.operation {
                        Operation::Add => {
                            match monkey.value {
                                Value::Old => { worry + worry }
                                Value::Value(val) => {
                                    worry + val
                                }
                            }
                        }
                        Operation::Multiply => {
                            match monkey.value {
                                Value::Old => {
                                    worry * worry
                                }
                                Value::Value(val) => {
                                    worry * val
                                }
                            }
                        }
                    };

                    let res = f(res);

                    let next_monkey_id = if res % monkey.test == 0 {
                        monkey.if_true
                    } else {
                        monkey.if_false
                    };

                    ids.push((next_monkey_id, res));
                }
                ids
            };

            for (next_monkey_id, val) in res {
                let next_monkey = &mut monkeys[next_monkey_id as usize];
                next_monkey.worries.push(val);
            }
        }
    }

    monkeys.sort_by(|a, b| a.inspected.cmp(&b.inspected));
    let length = monkeys.len();
    monkeys[length - 1].inspected * monkeys[length - 2].inspected
}

pub fn puzzle_a(path: &str) -> usize {
    let input = std::fs::read_to_string(path)
        .unwrap();

    let monkeys = parse_input(input.as_str());

    calculate_worry(monkeys, |e| e / 3, 20)
}

pub fn puzzle_b(path: &str) -> usize {
    let input = std::fs::read_to_string(path)
        .unwrap();

    let monkeys = parse_input(input.as_str());

    let m = monkeys
        .iter()
        .fold(1_isize, |ans, elem| ans * elem.test);

    calculate_worry(monkeys.clone(), |e| e % m, 10000)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculation_of_worry() {
        let data = include_str!("../input/day11.test.in");
        let input = parse_input(data);
        assert_eq!(calculate_worry(input), 10605);
    }

    #[test]
    fn test_parsing_input_works() {
        let data = include_str!("../input/day11.test.in");

        let monkeys = parse_input(data);
        let monkey0 = Monkey {
            id: 0,
            operation: Operation::Multiply,
            value: Value::Value(19),
            worries: vec![79, 98],
            test: 23,
            if_true: 2,
            if_false: 3,
            inspected: 0,
        };
        let monkey1 = Monkey {
            id: 1,
            operation: Operation::Add,
            value: Value::Value(6),
            worries: vec![54, 65, 75, 74],
            test: 19,
            if_true: 2,
            if_false: 0,
            inspected: 0,
        };
        let monkey2 = Monkey {
            id: 2,
            operation: Operation::Multiply,
            value: Value::Old,
            worries: vec![79, 60, 97],
            test: 13,
            if_true: 1,
            if_false: 3,
            inspected: 0,
        };
        let monkey3 = Monkey {
            id: 3,
            operation: Operation::Add,
            value: Value::Value(3),
            worries: vec![74],
            test: 17,
            if_true: 0,
            if_false: 1,
            inspected: 0,
        };

        assert_eq!(monkeys[0], monkey0);
        assert_eq!(monkeys[1], monkey1);
        assert_eq!(monkeys[2], monkey2);
        assert_eq!(monkeys[3], monkey3);
    }
}