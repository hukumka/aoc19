use aoc19::get_input;

fn main() {
    let data: Vec<_> = get_input(2)
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    println!("part1: {}", part1(data.clone()));
    println!("part2: {}", part2(data));
}

fn part1(mut data: Vec<usize>) -> usize {
    data[1] = 53;
    data[2] = 35;
    run_program(&mut data);
    data[0]
}

fn part2(input: Vec<usize>) -> usize {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut inp = input.clone();
            inp[1] = noun;
            inp[2] = verb;
            run_program(&mut inp);
            if inp[0] == 19_690_720 {
                return noun * 100 + verb;
            }
        }
    }
    panic!();
}

fn run_program(data: &mut [usize]) {
    let mut i = 0;
    loop {
        if data[i] == 99 {
            break;
        }
        let a = data[data[i + 1]];
        let b = data[data[i + 2]];
        let res = data[i + 3];
        data[res] = match data[i] {
            1 => a + b,
            2 => a * b,
            _ => panic!(),
        };
        i += 4;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let mut input = vec![1, 0, 0, 0, 99];
        run_program(&mut input);
        assert_eq!(input, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_input() {
        let data: Vec<_> = include_str!("../../tests/2/input")
            .trim()
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        assert_eq!(part1(data.clone()), 19690720);
        assert_eq!(part2(data), 5335);
    }
}
