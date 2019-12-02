static INPUT: &str = include_str!("../inputs/day02");

const ADD: usize = 1;
const MUL: usize = 2;
const HALT: usize = 99;

pub fn part1() {
    let mut index = 0;
    let mut sequence = INPUT
        .trim()
        .split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    while let &[opcode, lhs, rhs, dest] = &sequence[index..index + 4] {
        index += 4;
        match opcode {
            ADD  => sequence[dest] = sequence[lhs] + sequence[rhs],
            MUL  => sequence[dest] = sequence[lhs] * sequence[rhs],
            HALT => break,
            _ => panic!("invalid opcode"),
        }
    }

    println!("{:?}", sequence[0]);
}

pub fn part2() {
    println!();
}
