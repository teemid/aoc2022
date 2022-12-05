use std::env;
use std::fs;

type Stack = Vec<char>;
type SupplyStacks = Vec<Stack>;

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Missing arguments. Usage [filename]");
        return;
    }

    let filename = &args[1];
    let part = &args[2];
    let content = fs::read_to_string(&filename).unwrap();
    let (mut stacks, move_list) = parse_input(&content);
    match part.as_str() {
        "1" => {
            let message = part_1(&mut stacks, &move_list);
            println!("The message is {}", message);
        },
        "2" => {
            let message = part_2(&mut stacks, &move_list);
            println!("The message is {}", message);
        }
        _ => panic!("Invalid argument: {}", part),
    }
}

fn part_1(stacks: &mut SupplyStacks, move_list: &Vec<Move>) -> String {
    let stacks: &mut Vec<Vec<char>> = stacks.as_mut();
    for m in move_list {
        let length = stacks[m.from].len();

        let boxes: Vec<char> = stacks[m.from].drain(length-m.count..length).collect();
        for b in boxes.iter().rev() {
            stacks[m.to].push(*b);
        }
    }

    let mut message = String::new();
    for stack in stacks {
        message.push(*stack.last().unwrap());
    }

    message
}

fn part_2(stacks: &mut SupplyStacks, move_list: &Vec<Move>) -> String {
    let stacks: &mut Vec<Vec<char>> = stacks.as_mut();
    for m in move_list {
        let length = stacks[m.from].len();

        let boxes: Vec<char> = stacks[m.from].drain(length-m.count..length).collect();
        for b in boxes.iter() {
            stacks[m.to].push(*b);
        }
    }

    let mut message = String::new();
    for stack in stacks {
        message.push(*stack.last().unwrap());
    }

    message
}

fn parse_input(content: &String) -> (SupplyStacks, Vec<Move>) {
    let mut parts = content.split("\r\n\r\n");
    let start_state_str = parts.nth(0).unwrap();
    let move_list_str = parts.nth(0).unwrap();

    let stacks = parse_start_state(start_state_str);
    let move_list = parse_move_list(move_list_str);

    (stacks, move_list)
}

fn parse_start_state(start_state: &str) -> SupplyStacks {
    let lines = start_state.lines();

    let mut stacks = vec![vec![]; 1];
    for (index, line) in lines.rev().enumerate() {
        if index == 0 {
            let parts = line.split_whitespace();
            let mut count = 0;
            for part in parts {
                let c: usize = part.parse().unwrap();

                count = c;
            }

            stacks.resize(count, vec![]);
        } else {
            for (i, c) in line.chars().enumerate() {
                match c {
                    'A'..='Z' => stacks[i / 4].push(c),
                    _ => continue,
                }
            }
        }
    }

    stacks
}

fn parse_move_list(move_list: &str) -> Vec<Move> {
    let mut moves = vec![];

    for line in move_list.lines() {
        if line.len() == 0 {
            continue;
        }

        let mut parts = line.split(" ");
        let count: usize = parts.nth(1).unwrap().parse().unwrap();
        let from: usize = parts.nth(1).unwrap().parse().unwrap();
        let to: usize = parts.nth(1).unwrap().parse().unwrap();

        moves.push(Move {
            count: count,
            from: from - 1,
            to: to - 1,
        });
    }

    moves
}
