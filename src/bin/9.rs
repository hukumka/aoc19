use aoc19::get_input;
use aoc19::intcode::IntCode;

fn main() {
    let input = get_input(9);

    let input: Vec<_> = input
        .trim()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    println!("part1: {}", exec(input.clone(), 1));
    println!("part2: {}", exec(input, 2));
}

fn exec(input: Vec<i64>, x: i64) -> i64 {
    let mut intcode = IntCode::new(input);
    intcode.input().push_back(x);
    while intcode.step() {}

    assert_eq!(intcode.output().len(), 1);
    intcode.output().pop_front().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let input = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut intcode = IntCode::new(input.clone());
        while intcode.step() {}
        assert_eq!(intcode.output().iter().cloned().collect::<Vec<_>>(), input);
    }
}
