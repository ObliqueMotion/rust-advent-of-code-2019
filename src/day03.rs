use std::collections::HashMap;
use std::ops::AddAssign;
use Direction::*;

static INPUT: &str = include_str!("../inputs/day03");

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl AddAssign<&Direction> for (i32, i32) {
    fn add_assign(&mut self, other: &Direction) {
        let (x, y) = self;
        match other {
            Up    => *y += 1,
            Right => *x += 1,
            Down  => *y -= 1,
            Left  => *x -= 1,
        }
    }
}

fn parse_instruction(instruction: &str) -> (Direction, i32) {
    match (instruction.chars().next(), instruction[1..].parse::<i32>()) {
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

fn map_coords_to_steps(input: &str) -> HashMap<(i32, i32), usize> {
    let mut step = 0;
    let mut coord = (0, 0);
    let mut coord_map = HashMap::new();
    let instructions = input.trim().split(',').map(parse_instruction);
    
    for (direction, distance) in instructions {
        for _ in 0..distance {
            step += 1;
            coord += &direction;
            coord_map.entry(coord).or_insert(step);
        }
    }
    
    coord_map
}

pub fn part1() {
    let mut lines = INPUT.lines();
    let input1 = lines.next().expect("Failed to read puzzle input.");
    let input2 = lines.next().expect("Failed to read puzzle input.");

    let wire1 = map_coords_to_steps(input1);
    let wire2 = map_coords_to_steps(input2);

    let min_distance = wire1.keys().filter(|k| wire2.contains_key(k))
        .map(|(x, y)| x.abs() + y.abs())
        .min();

    println!("Day03 Part1: {:?}", min_distance);
}

pub fn part2() {
    let mut lines = INPUT.lines();
    let input1 = lines.next().expect("Failed to read puzzle input.");
    let input2 = lines.next().expect("Failed to read puzzle input.");

    let wire1 = map_coords_to_steps(input1);
    let wire2 = map_coords_to_steps(input2);

    let min_steps = wire1.keys().filter(|k| wire2.contains_key(k))
        .map(|k| wire1[k] + wire2[k])
        .min();

    println!("Day03 Part2: {:?}", min_steps);
}
