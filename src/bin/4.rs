use aoc19::get_input;
use reformation::Reformation;

#[derive(Reformation)]
#[reformation("{from}-{to}")]
struct Input {
    from: u32,
    to: u32,
}

fn main() {
    let input = get_input(4);
    let Input { from, to } = Input::parse(input.trim()).unwrap();
    println!("part1: {}", part1(from, to));
    println!("part1: {}", part2(from, to));
}

fn part1(from: u32, to: u32) -> usize {
    (from..=to).filter(|x| can_be_password(*x)).count()
}

fn part2(from: u32, to: u32) -> usize {
    (from..=to).filter(|x| can_be_password2(*x)).count()
}

fn can_be_password2(x: u32) -> bool {
    let s = format!("|{}|", x);
    let inc = s.as_bytes().windows(4).all(|x| x[1] <= x[2]);
    let duo = s
        .as_bytes()
        .windows(4)
        .any(|x| x[1] == x[2] && x[0] != x[1] && x[2] != x[3]);
    inc && duo
}

fn can_be_password(x: u32) -> bool {
    let s = x.to_string();
    let inc = s.as_bytes().windows(2).all(|x| x[0] <= x[1]);
    let duo = s.as_bytes().windows(2).any(|x| x[0] == x[1]);
    inc && duo
}
