static INPUT: &str = include_str!("../inputs/day02");

const OUTPUT: usize = 0;
const NOUN:   usize = 1;
const VERB:   usize = 2;

const ADD:    usize = 1;
const MUL:    usize = 2;
const HALT:   usize = 99;

fn load_instructions() -> Vec<usize> {
    INPUT
        .trim()
        .split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

fn run_instructions(noun: usize, verb: usize) -> usize {
    let mut index = 0;
    let mut instructions = load_instructions();
    
    instructions[NOUN] = noun;
    instructions[VERB] = verb;
    
    while let &[opcode, lhs, rhs, dest] = &instructions[index..index + 4] {
        index += 4;
        match opcode {
            ADD  => instructions[dest] = instructions[lhs] + instructions[rhs],
            MUL  => instructions[dest] = instructions[lhs] * instructions[rhs],
            HALT => break,
            _ => panic!("invalid opcode"),
        }
    }
    
    instructions[OUTPUT] 
}

pub fn part1() {
    println!("{}", run_instructions(12, 2));
}

pub fn part2() {
    const TARGET_OUTPUT: usize = 19690720;
    for noun in 0..100 {
        for verb in 0..100 {
            if run_instructions(noun, verb) == TARGET_OUTPUT {
                return println!("{}", 100 * noun + verb);
            }
        }
    }
}