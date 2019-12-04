use itertools::Itertools;
use std::ops::RangeInclusive;

const INPUT: RangeInclusive<u32> = (138307..=654504);

/// Returns an iterator over the digits of x
fn digits_of(x: u32) -> impl Iterator<Item = u32> {
    let digit_count = (x as f32).log10() as u32 + 1;
    (0..digit_count).map(move |exp| x / 10u32.pow(exp) % 10).rev()
}

/// Returns true if x contains any adjacent double digit
fn contains_double_digit(&x: &u32) -> bool {
    digits_of(x).tuple_windows().any(|(d1, d2)| d1 == d2)
}

/// Returns true if the digits (from left to right) never decrease.
fn digits_never_decrease(&x: &u32) -> bool {
    digits_of(x).tuple_windows().all(|(d1, d2)| d1 <= d2)
}

/// Returns the unique count of adjacent double digits
fn doubles_count(x: u32) -> usize {
    digits_of(x)
        .tuple_windows()
        .filter_map(|(d1, d2)| if d1 == d2 { Some(d1) } else { None })
        .unique()
        .count()
}

/// Returns the unique count of adjacent triple digits.
fn triples_count(x: u32) -> usize {
    digits_of(x)
        .tuple_windows()
        .filter_map(|(d1, d2, d3)| if d1 == d2 && d1 == d3 { Some(d1) } else { None })
        .unique()
        .count()
}

/// Contains a strict double digit that is not 
/// part of a larger group of the same digit.
fn contains_strict_double_digit(&x: &u32) -> bool {
    doubles_count(x) > triples_count(x)
}

pub fn part1() {
    let count = INPUT
        .filter(digits_never_decrease)
        .filter(contains_double_digit)
        .count();
    println!("Day04 Part1: {}", count);
}

pub fn part2() {
    let count = INPUT
        .filter(digits_never_decrease)
        .filter(contains_strict_double_digit)
        .count();
    println!("Day04 Part2: {}", count);
}    
