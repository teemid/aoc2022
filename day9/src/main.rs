use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Debug)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

#[derive(Debug)]
struct State {
    rope: Vec<(i32, i32)>,
    tail_positions: Vec<(i32, i32)>,
}

impl State {
    fn new(length: usize) -> State {
        State {
            rope: vec![(0, 0); length],
            tail_positions: vec![(0, 0)],
        }
    }

    #[allow(dead_code)]
    fn display(self: &Self, grid_start: (i32, i32), grid_end: (i32, i32)) {
        let icons: Vec<char> = self.icons();
        let width = (grid_end.0 - grid_start.0) as usize;
        let height = (grid_end.1 - grid_start.1) as usize;
        let mut grid = vec![vec!['.'; width]; height];

        let start_pos_x = (0 - grid_start.0) as usize;
        let start_pos_y = (grid_end.1 - 1 - 0) as usize;
        grid[start_pos_y][start_pos_x] = 's';

        for (i, (x, y)) in self.rope.iter().enumerate().rev() {
            let adjusted_x = (*x - grid_start.0) as usize;
            let adjusted_y = (grid_end.1 - *y - 1) as usize;

            grid[adjusted_y][adjusted_x] = icons[i];
        }

        for row in grid {
            for c in row {
                print!("{}", c);
            }

            print!("\n");
        }

        print!("\n\n");
    }

    fn icons(self: &Self) -> Vec<char> {
        self.rope
            .iter()
            .enumerate()
            .map(|(i, _)| {
                if i == 0 {
                    'H'
                } else if i == self.rope.len() - 1 {
                    'T'
                } else {
                    char::from_digit(i as u32, 10).unwrap()
                }
            })
            .collect()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Missing arguments. Usage [1|2] [filename]");
        return;
    }

    let part = &args[1];
    let filename = &args[2];
    let content = fs::read_to_string(filename).unwrap();
    let moves = parse_input(&content);

    let mut state = match part.as_str() {
        "1" => State::new(2),
        "2" => State::new(10),
        _ => unreachable!(),
    };

    run_simulation(&mut state, &moves);
}

fn run_simulation(state: &mut State, moves: &Vec<(Direction, i32)>) {
    for (direction, count) in moves {
        let dir = match direction {
            Direction::Right => (1, 0),
            Direction::Up => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Down => (0, -1),
        };

        for _ in 0..*count {
            make_move(state, dir);
        }
    }

    println!("{:?}", count_unique_positions(&state));
}

fn is_neighbor(a: &(i32, i32), b: &(i32, i32)) -> bool {
    (a.0 - b.0).abs() <= 1 && (a.1 - b.1).abs() <= 1
}

fn make_move(state: &mut State, dir: (i32, i32)) {
    state.rope[0] = (state.rope[0].0 + dir.0, state.rope[0].1 + dir.1);

    for i in 0..state.rope.len() - 1 {
        let segment = &mut state.rope[i..i + 2];
        let head = segment[0];
        let tail = segment[1];

        if !is_neighbor(&head, &tail) {
            let diff = (head.0 - tail.0, head.1 - tail.1);
            segment[1] = (tail.0 + diff.0.signum(), tail.1 + diff.1.signum());
        }
    }

    state.tail_positions.push(*state.rope.last().unwrap());
}

fn count_unique_positions(state: &State) -> usize {
    let set: HashSet<&(i32, i32)> = HashSet::from_iter(state.tail_positions.iter());

    set.len()
}

fn parse_input(content: &String) -> Vec<(Direction, i32)> {
    let mut moves = vec![];
    for line in content.lines() {
        match line.split_once(' ') {
            Some(("R", count)) => moves.push((Direction::Right, count.parse().unwrap())),
            Some(("U", count)) => moves.push((Direction::Up, count.parse().unwrap())),
            Some(("L", count)) => moves.push((Direction::Left, count.parse().unwrap())),
            Some(("D", count)) => moves.push((Direction::Down, count.parse().unwrap())),
            _ => unreachable!(),
        }
    }

    moves
}
