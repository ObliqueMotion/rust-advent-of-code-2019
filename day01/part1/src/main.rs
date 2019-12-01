static INPUT: &str = include_str!("../../input");

fn main() {
    let part1 = INPUT
        .lines()
        .map(|s| s.parse::<u64>().unwrap() / 3 - 2)
        .sum::<u64>();
    println!("{}", part1);
}
