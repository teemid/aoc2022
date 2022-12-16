use std::env;
use std::fs;

#[derive(Debug)]
struct Directory {
    name: String,
    size: i32,
    dirs: Vec<Directory>,
    files: Vec<File>,
}

#[derive(Debug)]
struct File {
    name: String,
    size: i32,
}

impl Directory {
    fn new(name: String) -> Directory {
        Directory {
            name: name,
            size: 0,
            dirs: vec![],
            files: vec![],
        }
    }
}

impl<'a> File {
    fn new(name: String, size: i32) -> File {
        File {
            name: name,
            size: size,
        }
    }
}

const TOTAL_SPACE_AVAILABLE: i32 = 70000000;
const TOTAL_SPACE_NEEDED: i32 = 30000000;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return;
    }

    let filename = &args[1];
    let limit: i32 = args[2].parse().unwrap();
    let content = fs::read_to_string(filename).unwrap();

    let mut total_size = 0;
    let mut dir_sizes = vec![];
    let mut stack = vec![Directory::new("/".to_string())];
    for line in content.lines() {
        match line.split_once(' ') {
            Some(("$", "cd /")) => (),
            Some(("$", "cd ..")) => {
                complete_directory(&mut stack);
                let last_dir = stack.last().unwrap().dirs.last().unwrap();
                dir_sizes.push(last_dir.size);
                total_size += dir_limit(last_dir, limit);
            }
            Some(("$", "ls")) => (),
            Some(("$", cmd)) => {
                match cmd.split_once(' ') {
                    Some(("cd", dir_name)) => add_dir(&mut stack, dir_name),
                    _ => unreachable!(),
                }
            }
            Some(("dir", _)) => (),
            Some((file_size, file_name)) => {
                add_file(&mut stack, file_name, file_size.parse().unwrap())
            }
            _ => unreachable!(),
        }
    }

    while stack.len() > 1 {
        complete_directory(&mut stack);

        let last_dir = stack.last().unwrap().dirs.last().unwrap();
        dir_sizes.push(last_dir.size);
    }

    let mut dir = stack.pop().unwrap();
    dir.size = calculate_dir_size(&dir);

    dir_sizes.push(dir.size);

    total_size += dir_limit(&dir, limit);

    println!("The sum of folders with size less than {} is {}", limit, total_size);

    let total_used = dir.size;
    let total_free = TOTAL_SPACE_AVAILABLE - total_used;

    dir_sizes.sort();
    for size in &dir_sizes {
        if total_free + size >= TOTAL_SPACE_NEEDED {
            println!("The smallest directory to get {} of free space is {}", TOTAL_SPACE_NEEDED,size);
            break;
        }
    }
}

fn dir_limit(dir: &Directory, limit: i32) -> i32 {
    if dir.size <= limit {
        dir.size
    } else {
        0
    }
}

fn add_dir(stack: &mut Vec<Directory>, dir_name: &str) {
    stack.push(Directory::new(dir_name.to_string()));
}

fn add_file(stack: &mut Vec<Directory>, file_name: &str, file_size: i32) {
    let dir = stack.last_mut().unwrap();
    dir.files.push(File::new(file_name.to_string(), file_size));
}

fn complete_directory(stack: &mut Vec<Directory>) {
    let mut dir = stack.pop().unwrap();
    dir.size = calculate_dir_size(&dir);

    let d = stack.last_mut().unwrap();
    d.dirs.push(dir);
}

fn calculate_dir_size(dir: &Directory) -> i32 {
    let file_size_total: i32 = dir.files.iter().map(|f| f.size).sum();
    let dir_size_total: i32 = dir.dirs.iter().map(|d| d.size).sum();

    file_size_total + dir_size_total
}
