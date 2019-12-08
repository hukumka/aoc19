use aoc19::get_input;

use aoc19::intcode::IntCode;

fn main() {
    let input = get_input(5);
    let input: Vec<_> = input
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    println!("part1: {}", exec(input.clone(), 1));
    println!("part2: {}", exec(input, 5));
}

fn exec(input: Vec<i32>, x: i32) -> i32 {
    let mut intcode = IntCode::new(input);
    intcode.input().push_back(x);
    while intcode.step() {}
    let res = intcode.output().pop_back().unwrap();
    while let Some(x) = intcode.output().pop_back() {
        assert_eq!(x, 0);
    }
    res
}
