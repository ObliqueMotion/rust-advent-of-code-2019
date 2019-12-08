use std::collections::HashMap;

static INPUT: &str = include_str!("../inputs/day06");

type Object = &'static str;
type OrbitMap = HashMap<Object, Object>;
type CountMap = HashMap<Object, usize>;

fn map_orbits() -> OrbitMap {
    INPUT.lines().map(|line| (&line[4..], &line[..3])).collect()
}

fn orbit_count(object: Object, orbits: &OrbitMap, counts: &mut CountMap) -> usize {
    if !orbits.contains_key(object) {
        counts.insert(object, 0);
    } else if !counts.contains_key(object) {
        let count = 1 + orbit_count(orbits[object], orbits, counts);
        counts.insert(object, count);
    }
    counts[object]
}

fn map_counts(orbits: &OrbitMap) -> CountMap {
    let mut counts = HashMap::new();
    for k in orbits.keys() {
        orbit_count(k, orbits, &mut counts);
    }
    counts
}

fn orbital_jumps(object1: Object, object2: Object, orbits: &OrbitMap) -> usize {
    let mut path1 = HashMap::new();
    let mut path2 = HashMap::new();

    orbit_count(object1, orbits, &mut path1);
    orbit_count(object2, orbits, &mut path2);

    let mut current = object1;

    while !path2.contains_key(current) {
        current = orbits[current];
    }

    let intersection = path1[current];
    let jumps1 = path1[object1] - intersection - 1;
    let jumps2 = path2[object2] - intersection - 1;

    jumps1 + jumps2
}

pub fn part1() {
    let orbits = map_counts(&map_orbits()).values().sum::<usize>();
    println!("Day06 Part1 {}", orbits);
}

pub fn part2() {
    let jumps = orbital_jumps("YOU", "SAN", &map_orbits());
    println!("Day06 Part2 {}", jumps);
}
