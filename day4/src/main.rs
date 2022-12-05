use std::env;
use std::fs;

#[derive(Debug)]
struct Assignment {
    start: i32,
    stop: i32,
}

impl Assignment {
    fn contains(self: &Self, assignment: &Assignment) -> bool {
        self.start <= assignment.start && assignment.stop <= self.stop
        && !self.stop < assignment.start
        && !assignment.stop < self.start
    }

    fn is_overlapping(self: &Self, assignment: &Assignment) -> bool {
        (self.start <= assignment.start && assignment.start <= self.stop)
        || (assignment.start <= self.start && self.start <= assignment.stop)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let content = fs::read_to_string(filename).unwrap();
    let assignments = parse_input(&content);
    let n = part_1(&assignments);
    println!("Number of completely overlapping ranges: {n}");
    let n = part_2(&assignments);
    println!("Number of overlapping ranges: {n}");
}

fn part_1(assignments: &Vec<(Assignment, Assignment)>) -> i32 {
    let mut count = 0;
    for (assignment1, assignment2) in assignments {
        if assignment1.contains(assignment2) {
            count += 1;
        } else if assignment2.contains(assignment1) {
            count += 1;
        }
    }

    count
}

fn part_2(assignments: &Vec<(Assignment, Assignment)>) -> i32 {
    let mut count = 0;
    for (assignment1, assignment2) in assignments {
        if assignment1.is_overlapping(assignment2) {
            count += 1;
        } else if assignment2.is_overlapping(assignment1) {
            count += 1;
        }
    }

    count
}

fn parse_input(content: &String) -> Vec<(Assignment, Assignment)> {
    let mut assignments = vec![];
    for line in content.lines() {
        let mut parts = line.split(",");
        let a = &parts.nth(0).unwrap();
        let b = &parts.nth(0).unwrap();

        let assignment_1 = parse(a);
        let assignment_2 = parse(b);

        assignments.push((assignment_1, assignment_2))
    }

    assignments
}

fn parse(part: &str) -> Assignment {
    let mut parts = part.split("-");
    let start: i32 = parts.nth(0).unwrap().parse().unwrap();
    let stop: i32 = parts.nth(0).unwrap().parse().unwrap();

    Assignment{ start: start, stop: stop }
}
