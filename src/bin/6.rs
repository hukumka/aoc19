use aoc19::get_input;
use reformation::Reformation;
use std::collections::HashMap;

#[derive(Reformation)]
#[reformation(r"{parent}\){child}")]
struct Input<'a> {
    parent: &'a str,
    child: &'a str,
}

fn main() {
    let input = get_input(6);
    let orbits: Vec<_> = input
        .trim()
        .lines()
        .map(|s| Input::parse(s).unwrap())
        .collect();

    let mut parent = HashMap::new();
    for i in &orbits {
        parent.insert(i.child, i.parent);
    }

    // part1
    let mut count = 0;
    let mut cache = HashMap::new();
    for i in &orbits {
        count += parent_count(&mut cache, &parent, i.child);
    }
    println!("part1: {}", count);
    
    // part2
    let p1 = parents(&parent, "YOU");
    let p2 = parents(&parent, "SAN");
    let common = p1.iter().zip(&p2)
        .filter(|(a, b)| a == b)
        .count();
    println!("part2: {}", p1.len() + p2.len() - common * 2);
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

fn parent_count<'a>(cache: &mut HashMap<&'a str, usize>, parent: &HashMap<&'a str, &'a str>, node: &'a str) -> usize {
    if let Some(x) = cache.get(node){
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
