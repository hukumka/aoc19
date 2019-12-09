use grid::Grid;
use aoc19::get_input;
use bytecount::count;

fn main(){
    let input = get_input(8);
    println!("part1: {}", part1(input.trim().as_bytes()));
    part2(input.trim().as_bytes());
}

fn part2(input: &[u8]){
    let (w, h) = (25, 6);
    let mut grid = Grid::new((w, h), b'2');
    for layer in input.chunks(w*h){
        for y in 0..h{
            for x in 0..w{
                let old = grid[(x, y)];
                if old == b'2'{
                    grid[(x, y)] = layer[y*w + x];
                }
            }
        }
    }
    for y in 0..h{
        for x in 0..w{
            if grid[(x, y)] == b'1'{
                print!("#");
            }else{
                print!(" ");
            }
        }
        println!();
    }
}

fn part1(input: &[u8]) -> usize{
    let (w, h) = (25, 6);
    input.chunks(w*h)
        .min_by_key(|x| count(x, b'0'))
        .map(|x|{
            println!("{:?}", x);
            count(x, b'1') * count(x, b'2')
        })
        .unwrap()
}
