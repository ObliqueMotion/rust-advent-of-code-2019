use std::collections::HashMap;
use Direction::*;

static INPUT: &str = include_str!("../inputs/day03");

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn update(&self, x: &mut i64, y: &mut i64) {
        match self {
            Up    => *y += 1,
            Right => *x += 1,
            Down  => *y -= 1,
            Left  => *x -= 1,
        }
    }
}

fn parse_instruction(instruction: &str) -> (Direction, i64) {
    match (instruction.chars().next(), instruction[1..].parse::<i64>()) {
        (Some(direction), Ok(distance)) => match direction {
            'U' => (Up,    distance),
            'R' => (Right, distance),
            'D' => (Down,  distance),
            'L' => (Left,  distance),
            _ => panic!("Invalid Instruction: {}", instruction),
        },
        _ => panic!("Invalid Instruction: {}", instruction),
    }
}

fn map_coords_to_steps(input: &str) -> HashMap<(i64, i64), usize> {
    let mut x = 0;
    let mut y = 0;
    let mut step = 0;
    let mut coordinates = HashMap::new();
    let instructions = input.trim().split(',').map(parse_instruction);
    
    let mut add_entries_for = |direction: &Direction| {
        step += 1;
        direction.update(&mut x, &mut y);
        coordinates.entry((x, y)).or_insert(step);
    };

    for (direction, distance) in instructions {
        for _ in 0..distance {
            add_entries_for(&direction)
        }
    }
    
    coordinates
}

pub fn part1() {
    let mut lines = INPUT.lines();
    let input1 = lines.next().expect("Failed to read puzzle input.");
    let input2 = lines.next().expect("Failed to read puzzle input.");

    let cts1 = map_coords_to_steps(input1);
    let cts2 = map_coords_to_steps(input2);

    let min_distance = cts1.keys().filter(|k| cts2.contains_key(k))
        .map(|(x, y)| x.abs() + y.abs())
        .min();

    println!("Day03 Part1: {:?}", min_distance);
}

pub fn part2() {
    let mut lines = INPUT.lines();
    let input1 = lines.next().expect("Failed to read puzzle input.");
    let input2 = lines.next().expect("Failed to read puzzle input.");

    let cts1 = map_coords_to_steps(input1);
    let cts2 = map_coords_to_steps(input2);

    let min_steps = cts1.keys().filter(|k| cts2.contains_key(k))
        .map(|k| cts1[k] + cts2[k])
        .min();

    println!("Day03 Part2: {:?}", min_steps);
}
