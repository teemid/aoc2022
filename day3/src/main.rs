use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Missing arguments. Usage [filename]");
        return;
    }

    let filename = &args[1];
    let content = fs::read_to_string(filename).unwrap();
    let rucksacks = parse_input(&content);
    let sum = part_1(&rucksacks);
    println!("The sum of the priority of the misplaced items is {}", sum);

    let a = part_2(&rucksacks);
    println!("The sum of the priority of the misplaced items is {}", a);
}

fn part_1(rucksacks: &Vec<&str>) -> u32 {
    let mut sum = 0;

    for rucksack in rucksacks {
        let length = rucksack.len();
        let half_length = length / 2;
        let first_compartment = &rucksack[0..half_length];
        let second_compartment = &rucksack[half_length..length];

        let f: HashSet<char> = HashSet::from_iter(first_compartment.chars());
        let s: HashSet<char> = HashSet::from_iter(second_compartment.chars());

        let intersection = f.intersection(&s);
        for c in intersection {
            sum += char_to_priority(c);
        }
    }

    sum
}

fn part_2(rucksacks: &Vec<&str>) -> u32 {
    let mut sum = 0;

    let range = 3..rucksacks.len() + 1;
    let mut prev = 0;
    for curr in range.step_by(3) {
        let group = &rucksacks[prev..curr];

        let elf1: HashSet<char> = HashSet::from_iter(group[0].chars());
        let elf2: HashSet<char> = HashSet::from_iter(group[1].chars());
        let elf3: HashSet<char> = HashSet::from_iter(group[2].chars());

        let intersection = elf1
            .iter()
            .filter(|i| elf2.contains(i))
            .filter(|i| elf3.contains(i));

        prev = curr;

        let list: Vec<&char> = intersection.collect();
        sum += char_to_priority(list[0]);

    }

    sum
}

fn char_to_priority(c: &char) -> u32 {
    let p = *c as u32;
    let a_lower = 'a' as u32;
    let a_upper = 'A' as u32;

    match c {
        'a'..='z' => p - a_lower + 1,
        'A'..='Z' => p - a_upper + 27,
        _ => panic!("Invalid input"),
    }
}

fn parse_input(content: &String) -> Vec<&str> {
    let mut rucksacks = vec![];

    for line in content.lines() {
        rucksacks.push(line);
    }

    rucksacks
}
