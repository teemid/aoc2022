use std::env;
use std::fs;

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

struct Cpu {
    cycle: usize,
    pc: usize,
    x: i32,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu{ cycle: 0, pc: 0, x: 1 }
    }
}

const CRT_WIDTH: usize = 40;
const CRT_HEIGHT: usize = 6;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Missing arguments. Usage [filename]");
        return;
    }

    let mut instructions = vec![];

    let filename = &args[1];
    let content = fs::read_to_string(filename).unwrap();
    for line in content.lines() {
        match line.split_once(' ') {
            Some(("addx", count)) => { instructions.push(Instruction::AddX(count.parse().unwrap())); },
            Some((_, _)) => unreachable!(),
            None => { instructions.push(Instruction::Noop); },
        }
    }

    let mut screen = vec![".".repeat(CRT_WIDTH); CRT_HEIGHT];

    let mut signal_strength = 0;
    let mut cpu = Cpu::new();
    while cpu.pc < instructions.len() {
        let current_instruction = &instructions[cpu.pc];
        let cycles = match current_instruction {
            Instruction::Noop => 1,
            Instruction::AddX(_) => 2,
        };

        let sprite = (cpu.x - 1, cpu.x, cpu.x + 1);

        for _ in 0..cycles {

            let x = cpu.cycle % CRT_WIDTH;
            let y = cpu.cycle / CRT_WIDTH;

            if sprite.0 == x as i32 || sprite.1 == x as i32 || sprite.2 == x as i32 {
                screen[y].replace_range(x..x+1, "#");
            }

            cpu.cycle += 1;

            if cpu.cycle == 20 || (cpu.cycle > 20 && (cpu.cycle - 20) % 40 == 0) {
                println!("cycle = {}, x = {}", cpu.cycle, cpu.x);
                signal_strength += cpu.cycle as i32 * cpu.x;
            }
        }

        match current_instruction {
            Instruction::AddX(arg) => cpu.x += arg,
            _ => (),
        }

        cpu.pc += 1;
    }

    println!("signal strength = {}", signal_strength);
    for scanline in screen {
        println!("{}", scanline);
    }
}
