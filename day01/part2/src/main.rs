use std::iter::successors;

static INPUT: &str = include_str!("../../input");

fn main() {
    let part2 = INPUT
        .lines()
        .map(|input| {
            successors(
                input.parse::<u64>().ok().and_then(|mass| (mass / 3).checked_sub(2)),
                |&mass| (mass / 3).checked_sub(2),
            )
            .sum::<u64>()
        })
        .sum::<u64>();
    println!("{}", part2);
}
