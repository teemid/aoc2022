use std::collections::VecDeque;
use std::env;
use std::fs;
use std::str::Lines;

#[derive(Debug)]
enum Operation {
    Old,
    Plus,
    Mul,
    Number(WorryLevel),
}

type WorryLevel = i64;

#[derive(Debug)]
struct Monkey {
    items: VecDeque<WorryLevel>,
    operation: Vec<Operation>,
    inspected_items: i32,
    test: WorryLevel,
    truth_branch: usize,
    false_branch: usize,
}

impl Monkey {
    fn new() -> Monkey {
        Monkey {
            items: VecDeque::new(),
            operation: vec![],
            inspected_items: 0,
            test: 1,
            truth_branch: 0,
            false_branch: 0,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Missing arguments. Usage [filename] [part]");
        return;
    }

    let filename = &args[1];
    let content = fs::read_to_string(filename).unwrap();

    let mut monkeys = vec![];
    let mut lines = content.lines();
    loop {
        match parse_monkey(&mut lines) {
            Some(monkey) => monkeys.push(monkey),
            None => break,
        }

        lines.next();
    }

    let mut rounds = 0;
    let mut divisor = 1;

    match args[2].as_str() {
        "1" => {
            rounds = 20;
            divisor = 3;
        }
        "2" => {
            rounds = 10000;
            divisor = 1;
        }
        _ => unreachable!(),
    };

    for _ in 0..rounds {
        let mut items: Vec<Vec<WorryLevel>> = vec![vec![]; monkeys.len()];
        for (i, monkey) in monkeys.iter_mut().enumerate() {
            // println!("Monkey {}", i);
            let len = items[i].len();
            for item in items[i].drain(0..len) {
                monkey.items.push_back(item);
            }

            while monkey.items.len() > 0 {
                monkey.inspected_items += 1;
                let item = monkey.items.pop_front().unwrap();
                // println!("Inspect item with worry level {}", item);
                let new_worry_level = calculate_new_worry_level(item, &monkey.operation);
                // println!("Worry level: {}", new_worry_level);
                let new_worry_level = new_worry_level / divisor;
                // println!("Worry level: {}", new_worry_level);

                if new_worry_level % monkey.test == 0 {
                    // println!("{} IS divisible by {}", new_worry_level, monkey.test);
                    items[monkey.truth_branch].push(new_worry_level);
                } else {
                    // println!("{} is NOT divisible by {}", new_worry_level, monkey.test);
                    items[monkey.false_branch].push(new_worry_level);
                }
            }

            // println!("\n");
        }

        for i in 0..items.len() {
            let monkey = &mut monkeys[i];

            for item in &items[i] {
                monkey.items.push_back(*item);
            }
        }
    }

    let mut most_active_list: Vec<i32> = monkeys.iter().map(|m| m.inspected_items).collect();
    most_active_list.sort();
    most_active_list.reverse();
    let one = most_active_list[0];
    let two = most_active_list[1];

    println!("{:?}", most_active_list);

    println!("The level of monkey business is {}", one * two);
}

#[allow(dead_code)]
fn print_state(monkeys: &Vec<Monkey>) {
    for (i, monkey) in monkeys.iter().enumerate() {
        println!("{}: {:?}", i, monkey.items);
    }

    println!("");
}

#[allow(dead_code)]
fn print_inspected_items(monkeys: &Vec<Monkey>) {
    for (i, monkey) in monkeys.iter().enumerate() {
        println!(
            "Monkey {} inspected items {} times",
            i, monkey.inspected_items
        );
    }
}

fn calculate_new_worry_level(item: WorryLevel, operations: &Vec<Operation>) -> WorryLevel {
    let arg1 = &operations[0];
    let op = &operations[1];
    let arg2 = &operations[2];

    let operand1 = match arg1 {
        Operation::Number(n) => *n,
        Operation::Old => item,
        _ => unreachable!(),
    };

    let operand2 = match arg2 {
        Operation::Number(n) => *n,
        Operation::Old => item,
        _ => unreachable!(),
    };

    match op {
        Operation::Mul => operand1 * operand2,
        Operation::Plus => operand1 + operand2,
        _ => unreachable!(),
    }
}

fn parse_monkey(lines: &mut Lines) -> Option<Monkey> {
    let line = lines.next();
    if line.is_none() {
        return Option::None;
    }

    let line = line.unwrap();
    if !line.starts_with("Monkey") {
        panic!("Called parse_monkey with invalid data");
    }

    let mut monkey = Monkey::new();

    let item_str = lines.next().unwrap();
    let trimmed = item_str.trim();
    monkey.items = match trimmed.split_once(':') {
        Some(("Starting items", items)) => items
            .trim()
            .split(", ")
            .map(|item| item.parse::<WorryLevel>().unwrap())
            .collect(),
        _ => unreachable!(),
    };

    let operation_str = lines.next().unwrap();
    let trimmed = operation_str.trim();
    monkey.operation = match trimmed.split_once(':') {
        Some(("Operation", operation)) => parse_operation(operation),
        _ => unreachable!(),
    };

    let test_str = lines.next().unwrap();
    let trimmed = test_str.trim();
    monkey.test = match trimmed.split_once(':') {
        Some(("Test", test)) => {
            let parts = test.split(" ");
            parts.last().unwrap().parse().unwrap()
        }
        _ => unreachable!(),
    };

    let truth_branch_str = lines.next().unwrap();
    let trimmed = truth_branch_str.trim();
    monkey.truth_branch = match trimmed.split_once(':') {
        Some(("If true", truth_branch)) => {
            let parts = truth_branch.split(" ");
            parts.last().unwrap().parse().unwrap()
        }
        _ => unreachable!(),
    };

    let false_branch_str = lines.next().unwrap();
    let trimmed = false_branch_str.trim();
    monkey.false_branch = match trimmed.split_once(':') {
        Some(("If false", false_branch)) => {
            let parts = false_branch.split(" ");
            parts.last().unwrap().parse().unwrap()
        }
        _ => unreachable!(),
    };

    Option::Some(monkey)
}

fn parse_operation(operation: &str) -> Vec<Operation> {
    let mut ops = vec![];

    let parts = operation.trim().split(" ");
    for part in parts {
        let trimmed = part.trim();
        match trimmed {
            "old" => ops.push(Operation::Old),
            "*" => ops.push(Operation::Mul),
            "+" => ops.push(Operation::Plus),
            "new" => (),
            "=" => (),
            value => ops.push(Operation::Number(value.parse().unwrap())),
        };
    }

    ops
}
