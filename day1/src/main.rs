use std::env;
use std::fs;

fn main()  {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments. Usage [input_filename]");
        return;
    }

    let filename = &args[1];

    let content = fs::read_to_string(filename).unwrap();

    let mut elves = prepare_data(&content);
    let calories = part_1(&elves);

    println!("The elf with the most calories has {calories} calories");

    let calories = part_2(&mut elves);
    println!("The top three elves have {calories} calories in total.")
}

fn prepare_data(content: &String) -> Vec<i32> {
    let mut elves = vec![];
    let mut current_elf = 0;

    for line in content.lines() {
        if line.len() == 0 {
            elves.push(current_elf);
            current_elf = 0;
        } else {
            let calories: i32 = line.parse().unwrap();
            current_elf += calories;
        }
    }

    elves.push(current_elf);

    elves
}

fn part_1(elves: &Vec<i32>) -> i32 {
    match elves.iter().max() {
        Some(elf) => *elf,
        _ => -1
    }
}

fn part_2(elves: &mut Vec<i32>) -> i32 {
    elves.sort();
    let top_three = &elves[elves.len() - 3..elves.len()];
    top_three.iter().sum()
}
