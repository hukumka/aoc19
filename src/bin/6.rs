use aoc19::get_input;
use std::collections::HashMap;

fn main() {
    let input = get_input(6);
    let parent = build_orbits(&input);

    println!("part1: {}", part1(&parent));
    println!("part2: {}", part2(&parent, "YOU", "SAN"));
}

fn part2(parent: &HashMap<&str, &str>, one: &str, other: &str) -> usize {
    let p1 = parents(&parent, one);
    let p2 = parents(&parent, other);
    let common = p1.iter().zip(&p2).filter(|(a, b)| a == b).count();
    p1.len() + p2.len() - common * 2
}

fn part1(parent: &HashMap<&str, &str>) -> usize {
    let mut count = 0;
    let mut cache = HashMap::new();
    for i in parent.keys() {
        count += parent_count(&mut cache, &parent, *i);
    }
    count
}

fn build_orbits(input: &str) -> HashMap<&str, &str> {
    input
        .trim()
        .lines()
        .map(|s| {
            let mut i = s.split(')');
            let parent = i.next().unwrap();
            let child = i.next().unwrap();
            (child, parent)
        })
        .collect()
}

fn parents<'a>(parent: &HashMap<&'a str, &'a str>, mut node: &'a str) -> Vec<&'a str> {
    let mut res = vec![];
    while let Some(&x) = parent.get(node) {
        res.push(x);
        node = x;
    }
    res.reverse();
    res
}

fn parent_count<'a>(
    cache: &mut HashMap<&'a str, usize>,
    parent: &HashMap<&'a str, &'a str>,
    node: &'a str,
) -> usize {
    if let Some(x) = cache.get(node) {
        return *x;
    }
    let res = if let Some(x) = parent.get(node) {
        parent_count(cache, parent, x) + 1
    } else {
        0
    };
    cache.insert(node, res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../../tests/6/input");
        let parent = build_orbits(&input);

        assert_eq!(part1(&parent), 271151);
        assert_eq!(part2(&parent, "YOU", "SAN"), 388);
    }
}
