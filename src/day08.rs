static INPUT: &str = include_str!("../inputs/day08");

pub fn part1() {
    let (_, ones, twos) = INPUT
        .as_bytes()
        .chunks_exact(25 * 6)
        .map(|chunk| {
            (
                chunk.iter().filter(|&&b| b == b'0').count(),
                chunk.iter().filter(|&&b| b == b'1').count(),
                chunk.iter().filter(|&&b| b == b'2').count(),
            )
        })
        .min_by_key(|&(zeros, _, _)| zeros)
        .unwrap();
    println!("Day08 Part1: {}", ones * twos);
}

pub fn part2() {
    println!("Day08 Part2:");
    let layers = INPUT.as_bytes().chunks_exact(25 * 6).collect::<Vec<_>>();
    let mut image = Vec::with_capacity(25 * 6);
    
    for i in 0..25 * 6 {
        let mut next = b'2';
        for layer in &layers {
            if layer[i] < next {
                next = layer[i];
                break;
            }
        }
        image.push(next);
    }
    
    for i in 0..6 {
        for j in 0..25 {
            match image[i * 25 + j] {
                48 => print!(" "),
                49 => print!("■"),
                _ => panic!(),
            }
        }
        println!();
    }
}
