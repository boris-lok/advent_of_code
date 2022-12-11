use std::borrow::BorrowMut;
use regex::Regex;

#[derive(Debug, Eq, PartialEq)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug, Eq, PartialEq)]
enum Value {
    Old,
    Value(isize),
}

#[derive(Debug, Eq, PartialEq)]
struct Monkey {
    id: u8,
    operation: Operation,
    value: Value,
    worries: Vec<isize>,
    test: isize,
    action: Vec<(bool, u8)>,
    inspected: usize,
}

// fn parse_input(input: &str) -> Vec<Monkey> {
//     let id_pattern = Regex::new(r"Monkey ([0-9]+?):").unwrap();
//
//     for line in input.lines() {
//         if id_pattern.is_match(line) {
//             let caps = id_pattern.captures(line)
//                 .unwrap();
//             let id = caps.get(1)
//                 .unwrap().as_str().parse::<u8>().unwrap();
//         }
//     }
//
//     vec![]
// }

fn calculate_worry(mut monkeys: Vec<Monkey>) {
    for _ in 0..20 {
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

                    let res = res / 3;

                    let next_monkey_id = if res % monkey.test == 0 {
                        monkey.action.first().unwrap().1
                    } else {
                        monkey.action.last().unwrap().1
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

    dbg!(monkeys);
}

fn calculate_worry_b(mut monkeys: Vec<Monkey>) {
    let m = 13 * 11 * 2 * 5 * 7 * 3 * 19 * 17;
    // let m = 23 * 19 * 13 * 17;
    for _ in 0..10000 {
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

                    let res = res % m;

                    let next_monkey_id = if res % monkey.test == 0 {
                        monkey.action.first().unwrap().1
                    } else {
                        monkey.action.last().unwrap().1
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

    dbg!(monkeys);
}

pub fn puzzle_a() {
    let monkey0 = Monkey {
        id: 0,
        operation: Operation::Multiply,
        value: Value::Value(11),
        worries: vec![63, 84, 80, 83, 84, 53, 88, 72],
        test: 13,
        action: vec![(true, 4), (false, 7)],
        inspected: 0,
    };
    let monkey1 = Monkey {
        id: 1,
        operation: Operation::Add,
        value: Value::Value(4),
        worries: vec![67, 56, 92, 88, 84],
        test: 11,
        action: vec![(true, 5), (false, 3)],
        inspected: 0,
    };
    let monkey2 = Monkey {
        id: 2,
        operation: Operation::Multiply,
        value: Value::Old,
        worries: vec![52],
        test: 2,
        action: vec![(true, 3), (false, 1)],
        inspected: 0,
    };
    let monkey3 = Monkey {
        id: 3,
        operation: Operation::Add,
        value: Value::Value(2),
        worries: vec![59, 53, 60, 92, 69, 72],
        test: 5,
        action: vec![(true, 5), (false, 6)],
        inspected: 0,
    };
    let monkey4 = Monkey {
        id: 4,
        operation: Operation::Add,
        value: Value::Value(3),
        worries: vec![61, 52, 55, 61],
        test: 7,
        action: vec![(true, 7), (false, 2)],
        inspected: 0,
    };
    let monkey5 = Monkey {
        id: 5,
        operation: Operation::Add,
        value: Value::Value(1),
        worries: vec![79, 53],
        test: 3,
        action: vec![(true, 0), (false, 6)],
        inspected: 0,
    };
    let monkey6 = Monkey {
        id: 6,
        operation: Operation::Add,
        value: Value::Value(5),
        worries: vec![59, 86, 67, 95, 92, 77, 91],
        test: 19,
        action: vec![(true, 4), (false, 0)],
        inspected: 0,
    };
    let monkey7 = Monkey {
        id: 7,
        operation: Operation::Multiply,
        value: Value::Value(19),
        worries: vec![58, 83, 89],
        test: 17,
        action: vec![(true, 2), (false, 1)],
        inspected: 0,
    };

    calculate_worry(vec![monkey0, monkey1, monkey2, monkey3, monkey4, monkey5, monkey6, monkey7]);
}

pub fn puzzle_b() {
    let monkey0 = Monkey {
        id: 0,
        operation: Operation::Multiply,
        value: Value::Value(11),
        worries: vec![63, 84, 80, 83, 84, 53, 88, 72],
        test: 13,
        action: vec![(true, 4), (false, 7)],
        inspected: 0,
    };
    let monkey1 = Monkey {
        id: 1,
        operation: Operation::Add,
        value: Value::Value(4),
        worries: vec![67, 56, 92, 88, 84],
        test: 11,
        action: vec![(true, 5), (false, 3)],
        inspected: 0,
    };
    let monkey2 = Monkey {
        id: 2,
        operation: Operation::Multiply,
        value: Value::Old,
        worries: vec![52],
        test: 2,
        action: vec![(true, 3), (false, 1)],
        inspected: 0,
    };
    let monkey3 = Monkey {
        id: 3,
        operation: Operation::Add,
        value: Value::Value(2),
        worries: vec![59, 53, 60, 92, 69, 72],
        test: 5,
        action: vec![(true, 5), (false, 6)],
        inspected: 0,
    };
    let monkey4 = Monkey {
        id: 4,
        operation: Operation::Add,
        value: Value::Value(3),
        worries: vec![61, 52, 55, 61],
        test: 7,
        action: vec![(true, 7), (false, 2)],
        inspected: 0,
    };
    let monkey5 = Monkey {
        id: 5,
        operation: Operation::Add,
        value: Value::Value(1),
        worries: vec![79, 53],
        test: 3,
        action: vec![(true, 0), (false, 6)],
        inspected: 0,
    };
    let monkey6 = Monkey {
        id: 6,
        operation: Operation::Add,
        value: Value::Value(5),
        worries: vec![59, 86, 67, 95, 92, 77, 91],
        test: 19,
        action: vec![(true, 4), (false, 0)],
        inspected: 0,
    };
    let monkey7 = Monkey {
        id: 7,
        operation: Operation::Multiply,
        value: Value::Value(19),
        worries: vec![58, 83, 89],
        test: 17,
        action: vec![(true, 2), (false, 1)],
        inspected: 0,
    };

    calculate_worry_b(vec![monkey0, monkey1, monkey2, monkey3, monkey4, monkey5, monkey6, monkey7]);
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculation_of_worry() {
        let monkey0 = Monkey {
            id: 0,
            operation: Operation::Multiply,
            value: Value::Value(19),
            worries: vec![79, 98],
            test: 23,
            action: vec![(true, 2), (false, 3)],
            inspected: 0,
        };
        let monkey1 = Monkey {
            id: 1,
            operation: Operation::Add,
            value: Value::Value(6),
            worries: vec![54, 65, 75, 74],
            test: 19,
            action: vec![(true, 2), (false, 0)],
            inspected: 0,
        };
        let monkey2 = Monkey {
            id: 2,
            operation: Operation::Multiply,
            value: Value::Old,
            worries: vec![79, 60, 97],
            test: 13,
            action: vec![(true, 1), (false, 3)],
            inspected: 0,
        };
        let monkey3 = Monkey {
            id: 3,
            operation: Operation::Add,
            value: Value::Value(3),
            worries: vec![74],
            test: 17,
            action: vec![(true, 0), (false, 1)],
            inspected: 0,
        };

        // calculate_worry(vec![monkey0, monkey1, monkey2, monkey3]);
        // calculate_worry_b(vec![monkey0, monkey1, monkey2, monkey3]);
        puzzle_b();
    }

    // #[test]
    // fn test_parsing_input_works() {
    //     let data = include_str!("../input/day11.test.in");
    //
    //     let monkeys = parse_input(data);
    //     let monkey0 = Monkey {
    //         id: 0,
    //         operation: Operation::Multiply,
    //         value: Value::Value(19),
    //         worries: vec![78, 98],
    //         test: 23,
    //         action: vec![(true, 2), (false, 3)],
    //         inspected: 0,
    //     };
    //     let monkey1 = Monkey {
    //         id: 1,
    //         operation: Operation::Add,
    //         value: Value::Value(6),
    //         worries: vec![54, 65, 75, 74],
    //         test: 19,
    //         action: vec![(true, 2), (false, 0)],
    //         inspected: 0,
    //     };
    //     let monkey2 = Monkey {
    //         id: 2,
    //         operation: Operation::Multiply,
    //         value: Value::Old,
    //         worries: vec![79, 60, 97],
    //         test: 0,
    //         action: vec![(true, 1), (false, 3)],
    //         inspected: 0,
    //     };
    //     let monkey3 = Monkey {
    //         id: 3,
    //         operation: Operation::Add,
    //         value: Value::Value(3),
    //         worries: vec![74],
    //         test: 17,
    //         action: vec![(true, 0), (false, 1)],
    //         inspected: 0,
    //     };
    //     assert_eq!(monkeys[0], monkey0);
    //     assert_eq!(monkeys[1], monkey1);
    //     assert_eq!(monkeys[2], monkey2);
    //     assert_eq!(monkeys[3], monkey3);
    // }
}