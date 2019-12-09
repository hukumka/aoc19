use aoc19::get_input;
use aoc19::intcode::IntCode;
use itertools::Itertools;

fn main() {
    let input = get_input(7);
    let program = input
        .trim()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    println!("part1: {}", part1(&program));
    println!("part2: {}", part2(&program));
}

fn part1(program: &[i64]) -> i64 {
    (0..=4)
        .permutations(5)
        .map(|input| get_thrust(program, &input))
        .max()
        .unwrap()
}

fn get_thrust(program: &[i64], input: &[i64]) -> i64 {
    let mut intcodes: Vec<_> = (0..input.len())
        .map(|_| IntCode::new(Vec::from(program)))
        .collect();

    let mut old_input = 0;
    for (cpu, inp) in intcodes.iter_mut().zip(input) {
        cpu.input().push_back(*inp);
        cpu.input().push_back(old_input);
        while cpu.step() {}
        old_input = cpu.output().pop_front().unwrap();
    }
    old_input
}

fn part2(program: &[i64]) -> i64 {
    (5..=9)
        .permutations(5)
        .map(|input| get_thrust2(program, &input))
        .max()
        .unwrap()
}

fn get_thrust2(program: &[i64], input: &[i64]) -> i64 {
    let mut intcodes: Vec<_> = input
        .iter()
        .map(|i| {
            let mut cpu = IntCode::new(Vec::from(program));
            cpu.input().push_back(*i);
            cpu
        })
        .collect();

    let mut old_input = 0;
    let mut res = 0;
    loop {
        for cpu in intcodes.iter_mut() {
            cpu.input().push_back(old_input);
            if let Some(x) = run_until_output(cpu) {
                old_input = x;
            } else {
                return res;
            }
        }
        res = old_input;
    }
}

fn run_until_output(cpu: &mut IntCode) -> Option<i64> {
    while cpu.output().is_empty() {
        if !cpu.step() {
            return None;
        }
    }
    cpu.output().pop_front()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&[
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0
            ]),
            54321
        );
        assert_eq!(
            part1(&[3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0]),
            43210
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&[
                3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28,
                -1, 28, 1005, 28, 6, 99, 0, 0, 5
            ]),
            139629729
        );
        assert_eq!(
            part2(&[
                3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001,
                54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53,
                55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
            ]),
            18216
        );
    }
}
