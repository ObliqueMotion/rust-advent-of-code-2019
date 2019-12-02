static INPUT: &str = include_str!("../inputs/day02");

const OUTPUT: usize = 0;
const NOUN:   usize = 1;
const VERB:   usize = 2;

const ADD:    usize = 1;
const MUL:    usize = 2;
const HALT:   usize = 99;

const TARGET_OUTPUT: usize = 19_690_720;

fn load_instructions() -> Vec<usize> {
    INPUT.trim().split(',').map(|n| n.parse().unwrap()).collect()
}

fn run(mut instructions: Vec<usize>, noun: usize, verb: usize) -> usize {
    let mut index = 0;
    instructions[NOUN] = noun;
    instructions[VERB] = verb;
    
    while let [opcode, lhs, rhs, dest] = instructions[index..index + 4] {
        index += 4;
        match opcode {
            ADD  => instructions[dest] = instructions[lhs] + instructions[rhs],
            MUL  => instructions[dest] = instructions[lhs] * instructions[rhs],
            HALT => break,
            _    => panic!("invalid opcode {}", opcode),
        }
    }
    
    instructions[OUTPUT] 
}

pub fn part1() {
    let instructions = load_instructions();
    println!("Day02 Part1: {}", run(instructions, 12, 2));
}

pub fn part2() {
    let instructions = load_instructions();
    
    for noun in 0..100 {
        for verb in 0..100 {
            if run(instructions.clone(), noun, verb) == TARGET_OUTPUT {
                return println!("Day02 Part2: {}", 100 * noun + verb);
            }
        }
    }
}