use aoc19::get_input;

fn main() {
    let s = get_input(1);
    println!("{}", part1(&s));
    println!("{}", part2(&s));
}

fn part1(input: &str) -> i32 {
    input
        .trim()
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .map(|s| s / 3 - 2)
        .sum()
}

fn part2(input: &str) -> i32 {
    fn fuel(x: i32) -> i32 {
        (x / 3 - 2).max(0)
    }
    fn fuel_sum(x: i32) -> i32 {
        let mut total = 0;
        let mut f = fuel(x);
        while f > 0 {
            total += f;
            f = fuel(f);
        }
        total
    }
    input
        .trim()
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .map(fuel_sum)
        .sum()
}
