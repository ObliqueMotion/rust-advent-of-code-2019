use std::env;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
//mod day06;
//mod day07;
//mod day08;
//mod day09;
//mod day10;
//mod day11;
//mod day12;
//mod day13;
//mod day14;
//mod day15;
//mod day16;
//mod day17;
//mod day18;
//mod day19;
//mod day20;
//mod day21;
//mod day22;
//mod day23;
//mod day24;
//mod day25;

fn main() {
    let solution = env::args().nth(1).unwrap_or_default();
    match solution.parse::<usize>() {
        Ok(11) => day01::part1(),
        Ok(12) => day01::part2(),

        Ok(21) => day02::part1(),
        Ok(22) => day02::part2(),

        Ok(31) => day03::part1(),
        Ok(32) => day03::part2(),

        Ok(41) => day04::part1(),
        Ok(42) => day04::part2(),

        Ok(51) => day05::part1(),
        Ok(52) => day05::part2(),

        _ => panic!("the disco"),
    }
}
