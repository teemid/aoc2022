use std::env;
use std::fs;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Missing arguments. Usage [filename]");
        return;
    }

    let filename = &args[1];
    let content = fs::read_to_string(filename).unwrap();

    let mut forest = vec![];
    for line in content.lines() {
        let l: Vec<i32> = line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
        forest.push(l);
    }

    let forest = forest;
    let visibility = part_1(&forest);
    let scenic_score = part_2(&forest);

    println!("There are {} visible trees from outside the grid", visibility);
    println!("The highest scenic score is {}", scenic_score);
}

fn part_1(forest: &Vec<Vec<i32>>) -> usize {
    let height = forest.len();
    let width = forest[0].len();
    let mut visibility = HashSet::new();

    for y in 0..height {
        let line = &forest[y];
        let mut current_height = -1;
        for x in 0..width {
            let tree = line[x];
            if current_height < tree {
                current_height = tree;
                let pos = (x, y);
                visibility.insert(pos);
            }
        }

        current_height = -1;
        for x in (0..width).rev() {
            let tree = line[x];
            if current_height < tree {
                current_height = tree;
                let pos = (x, y);
                visibility.insert(pos);
            }
        }
    }


    for x in 0..width {
        let mut current_height = -1;
        for y in 0..height {
            let tree = forest[y][x];
            if current_height < tree {
                current_height = tree;
                let pos = (x, y);
                visibility.insert(pos);
            }
        }

        current_height = -1;
        for y in (0..height).rev() {
            let tree = forest[y][x];
            if current_height < tree {
                current_height = tree;
                let pos = (x, y);
                visibility.insert(pos);
            }
        }
    }

    visibility.len()
}

fn part_2(forest: &Vec<Vec<i32>>) -> i32 {
    let height = forest.len();
    let width = forest[0].len();

    let mut max_scenic_score = 0;
    for x in 1..width-1 {
        for y in 1..height-1 {
            let scenic_score = calculate_scenic_score(forest, x, y);
            if max_scenic_score < scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    max_scenic_score
}

fn calculate_scenic_score(forest: &Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    let height = forest.len();
    let width = forest[0].len();
    let tree_height = forest[y][x];

    let mut left_scenic_score = 0;
    for x in (0..x).rev() {
        left_scenic_score += 1;

        let h = forest[y][x];
        if h >= tree_height {
            break;
        }
    }

    let mut right_scenic_score = 0;
    for x in x+1..width {
        right_scenic_score += 1;

        let h = forest[y][x];
        if h >= tree_height {
            break;
        }
    }

    let mut up_scenic_score = 0;
    for y in (0..y).rev() {
        up_scenic_score += 1;

        let h = forest[y][x];
        if h >= tree_height {
            break;
        }
    }

    let mut down_scenic_score = 0;
    for y in y+1..height {
        down_scenic_score += 1;

        let h = forest[y][x];
        if h >= tree_height {
            break;
        }
    }

    left_scenic_score * right_scenic_score * up_scenic_score * down_scenic_score
}
