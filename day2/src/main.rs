#[derive(Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Result {
    Win,
    Draw,
    Loss,
}

#[derive(Debug)]
struct Round {
    opponent: Hand,
    you: Hand,
    outcome: Result,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Missing arguments. Usage [input_filename]");
        return;
    }

    let filename = &args[1];
    let content = std::fs::read_to_string(filename).unwrap();
    let rounds = parse_input(&content);

    let score = part_1(&rounds);
    println!("Following the strategy guide gives the score {score}");

    let score = part_2(&rounds);
    println!("Following the updated instructions gives a score of {score}");
}

fn part_1(rounds: &Vec<Round>) -> i32 {
    let mut score = 0;

    for round in rounds {
        score += match round.you {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        };
        score += match judge_round(&round.you, &round.opponent) {
            Result::Win => 6,
            Result::Draw => 3,
            Result::Loss => 0,
        };
    }

    score
}

fn part_2(rounds: &Vec<Round>) -> i32 {
    let mut score = 0;

    for round in rounds {
        let you = find_needed_hand(&round.opponent, &round.outcome);

        score += match you {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3
        };

        score += match round.outcome {
            Result::Win => 6,
            Result::Draw => 3,
            Result::Loss => 0,
        };
    }

    score
}

fn find_needed_hand(opponent: &Hand, outcome: &Result) -> Hand {
    match opponent {
        Hand::Rock => match outcome {
            Result::Win => Hand::Paper,
            Result::Draw => Hand::Rock,
            Result::Loss => Hand::Scissors,
        },
        Hand::Paper => match outcome {
            Result::Win => Hand::Scissors,
            Result::Draw => Hand::Paper,
            Result::Loss => Hand::Rock,
        },
        Hand::Scissors => match outcome {
            Result::Win => Hand::Rock,
            Result::Draw => Hand::Scissors,
            Result::Loss => Hand::Paper,
        },
    }
}

fn judge_round(you: &Hand, opponent: &Hand) -> Result {
    match you {
        Hand::Rock => match opponent {
            Hand::Rock => Result::Draw,
            Hand::Paper => Result::Loss,
            Hand::Scissors => Result::Win,
        }
        Hand::Paper => match opponent {
            Hand::Rock => Result::Win,
            Hand::Paper => Result::Draw,
            Hand::Scissors => Result::Loss,
        }
        Hand::Scissors => match opponent {
            Hand::Rock => Result::Loss,
            Hand::Paper => Result::Win,
            Hand::Scissors => Result::Draw,
        }
    }
}

fn parse_input(content: &String) -> Vec<Round> {
    let mut rounds = vec![];

    for line in content.lines() {
        if line.len() == 0 {
            continue;
        }

        let mut it = line.chars();
        let elf = match it.nth(0).unwrap() {
            'A' => Hand::Rock,
            'B' => Hand::Paper,
            'C' => Hand::Scissors,
            _ => panic!("Invalid input"),
        };

        let c = it.nth(1).unwrap();
        let you = match c {
            'X' => Hand::Rock,
            'Y' => Hand::Paper,
            'Z' => Hand::Scissors,
            _ => panic!("Invalid input"),
        };

        let outcome = match c {
            'X' => Result::Loss,
            'Y' => Result::Draw,
            'Z' => Result::Win,
            _ => panic!("Invalid input"),
        };

        rounds.push(Round{ opponent: elf, you: you, outcome: outcome });
    }

    rounds
}
