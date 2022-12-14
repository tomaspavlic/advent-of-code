use std::{cell::RefCell, collections::LinkedList, fs};

fn main() {
    let monkeys: Vec<RefCell<Monkey>> = fs::read_to_string("input/day11.txt")
        .unwrap()
        .split("\n\n")
        .map(|m| m.lines().collect::<Vec<&str>>())
        .map(|ml| RefCell::new(Monkey::from(ml)))
        .collect();

    let base = monkeys
        .iter()
        .fold(1, |acc, monkey| acc * monkey.borrow().test);

    let monkeys1 = monkeys.clone();
    let part1 = monkey_business(monkeys1, 20, |worry_level| worry_level / 3);
    println!("1.part {}", part1);

    let part2 = monkey_business(monkeys, 10_000, |worry_level| worry_level % base);
    println!("2.part {}", part2);
}

fn monkey_business<F>(mut monkeys: Vec<RefCell<Monkey>>, n: usize, handle_worry_level: F) -> usize
where
    F: Fn(usize) -> usize,
{
    for _ in 0..n {
        for current_monkey in monkeys.iter() {
            let mut current_monkey = current_monkey.borrow_mut();

            while let Some(mut item) = current_monkey.items.pop_front() {
                current_monkey.visited += 1;

                if current_monkey.operation.0 == '*' {
                    match current_monkey.operation.1 {
                        Operation::Square => item *= item,
                        Operation::Number(n) => item *= n,
                    }
                } else {
                    match current_monkey.operation.1 {
                        Operation::Square => item += item,
                        Operation::Number(n) => item += n,
                    }
                }

                item = handle_worry_level(item);
                let dest_id = if item % current_monkey.test == 0 {
                    current_monkey.if_true
                } else {
                    current_monkey.if_false
                };

                monkeys
                    .get(dest_id)
                    .unwrap()
                    .borrow_mut()
                    .items
                    .push_back(item);
            }
        }
    }

    monkeys.sort_by(|a, b| b.borrow().visited.cmp(&a.borrow().visited));
    monkeys
        .iter()
        .take(2)
        .fold(1, |acc, m| acc * m.borrow().visited)
}

#[derive(Debug, Clone)]
enum Operation {
    Square,
    Number(usize),
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        if let Ok(number) = value.parse::<usize>() {
            return Operation::Number(number);
        }

        Operation::Square
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: LinkedList<usize>,
    operation: (char, Operation),
    test: usize,
    if_true: usize,
    if_false: usize,
    visited: usize,
}

impl From<Vec<&str>> for Monkey {
    fn from(ml: Vec<&str>) -> Self {
        let start_items = ml[1]
            .rsplit_once(":")
            .unwrap()
            .1
            .replace(' ', "")
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        let mut operation = ml[2].split(' ').rev();
        let op: Operation = operation.next().unwrap().into();
        let operator = operation.next().unwrap().chars().next().unwrap();

        Monkey {
            items: start_items,
            operation: (operator, op),
            test: ml[3].split(' ').next_back().unwrap().parse().unwrap(),
            if_true: ml[4].split(' ').next_back().unwrap().parse().unwrap(),
            if_false: ml[5].split(' ').next_back().unwrap().parse().unwrap(),
            visited: 0,
        }
    }
}
