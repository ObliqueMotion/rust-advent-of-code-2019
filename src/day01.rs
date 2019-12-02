use std::iter::successors;

static INPUT: &str = include_str!("../inputs/day01");

fn required_fuel(&mass: &u64) -> Option<u64> {
    (mass / 3).checked_sub(2)
}

fn compound_fuel(mass: u64) -> u64 {
    successors(required_fuel(&mass), required_fuel).sum::<u64>()
}

pub fn part1() {
    let solution = INPUT
        .lines()
        .map(|s| s.parse::<u64>().ok().as_ref().and_then(required_fuel).unwrap())
        .sum::<u64>();
    println!("{}", solution);
}

pub fn part2() {
    let solution = INPUT
        .lines()
        .map(|input| input.parse::<u64>().ok().map(compound_fuel).unwrap())
        .sum::<u64>();
    println!("{}", solution);
}
