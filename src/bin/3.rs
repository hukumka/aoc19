#![feature(test)]
use aoc19::get_input;
use reformation::Reformation;
use rpds::RedBlackTreeMap;

fn main() {
    let input = get_input(3);
    let (w1, w2) = parse_input(&input, std::usize::MAX);
    println!("part1: {}", part1(&w1, &w2));
    println!("part1: {}", part1_smort(&w1, &w2));
    println!("part2: {}", part2(&w1, &w2));
}

fn parse_input(input: &str, limit: usize) -> (Wire, Wire) {
    let mut wires = input.trim().lines().map(|s| {
        let paths = s.split(',').take(limit).map(|s| Path::parse(s).unwrap());
        Wire::new(paths)
    });
    (wires.next().unwrap(), wires.next().unwrap())
}

fn part1(w1: &Wire, w2: &Wire) -> i32 {
    w1.intersections(w2)
        .into_iter()
        .map(|((x, y), _)| x.abs() + y.abs())
        .min()
        .unwrap()
}

fn part1_smort(w1: &Wire, w2: &Wire) -> i32 {
    let vert1 = VerticalSegments::new(&w1.vertical);
    let min1 = w2
        .horizontal
        .iter()
        .flat_map(|s| vert1.intersections(s).map(move |s2| (s2.pos, s.pos)))
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap_or(std::i32::MAX);
    let vert2 = VerticalSegments::new(&w2.vertical);
    let min2 = w1
        .horizontal
        .iter()
        .flat_map(|s| vert2.intersections(s).map(move |s2| (s2.pos, s.pos)))
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap_or(std::i32::MAX);
    min1.min(min2)
}

fn part2(w1: &Wire, w2: &Wire) -> i32 {
    w1.intersections(w2)
        .into_iter()
        .map(|(_, v)| v)
        .min()
        .unwrap()
}

#[derive(Reformation)]
#[reformation("{dir}{len}")]
struct Path {
    len: i32,
    dir: char,
}

#[derive(Clone)]
struct Segment {
    pos: i32,
    range: (i32, i32),
    value: Linear,
}

/// Linear polynom f(x) = k*x + b
#[derive(Clone)]
struct Linear {
    k: i32,
    b: i32,
}

struct Wire {
    vertical: Vec<Segment>,
    horizontal: Vec<Segment>,
}

impl Wire {
    fn new(paths: impl Iterator<Item = Path>) -> Self {
        let mut vertical = vec![];
        let mut horizontal = vec![];
        let mut pos = (0, 0);
        let mut value = 0;
        for p in paths {
            match p.dir {
                'U' => {
                    let seg = Segment {
                        pos: pos.0,
                        range: (pos.1 - p.len, pos.1),
                        value: Linear {
                            k: -1,
                            b: value + p.len,
                        },
                    };
                    pos = (pos.0, pos.1 - p.len);
                    vertical.push(seg);
                }
                'D' => {
                    let seg = Segment {
                        pos: pos.0,
                        range: (pos.1, pos.1 + p.len),
                        value: Linear { k: 1, b: value },
                    };
                    pos = (pos.0, pos.1 + p.len);
                    vertical.push(seg);
                }
                'L' => {
                    let seg = Segment {
                        pos: pos.1,
                        range: (pos.0 - p.len, pos.0),
                        value: Linear {
                            k: -1,
                            b: value + p.len,
                        },
                    };
                    pos = (pos.0 - p.len, pos.1);
                    horizontal.push(seg);
                }
                'R' => {
                    let seg = Segment {
                        pos: pos.1,
                        range: (pos.0, pos.0 + p.len),
                        value: Linear { k: 1, b: value },
                    };
                    pos = (pos.0 + p.len, pos.1);
                    horizontal.push(seg);
                }
                _ => panic!(),
            }
            value += p.len;
        }
        Self {
            vertical,
            horizontal,
        }
    }

    fn intersections(&self, other: &Self) -> Vec<((i32, i32), i32)> {
        let mut res = vec![];
        for hor in &self.horizontal {
            for vert in &other.vertical {
                if let Some(x) = Self::intersects(vert, hor) {
                    res.push(x);
                }
            }
        }
        for vert in &self.vertical {
            for hor in &other.horizontal {
                if let Some(x) = Self::intersects(vert, hor) {
                    res.push(x);
                }
            }
        }
        res
    }

    fn intersects(vert: &Segment, hor: &Segment) -> Option<((i32, i32), i32)> {
        if vert.pos > hor.range.0
            && vert.pos < hor.range.1
            && hor.pos > vert.range.0
            && hor.pos < vert.range.1
        {
            let pos = (vert.pos, hor.pos);
            let v1 = vert.value.k * (hor.pos - vert.range.0) + vert.value.b;
            let v2 = hor.value.k * (vert.pos - hor.range.0) + hor.value.b;
            Some((pos, v1 + v2))
        } else {
            None
        }
    }
}

/// Datastructure for fast search of intersections
/// with horizontal segment
struct VerticalSegments {
    // Each lane holds set of all segments intersected by lane.
    // If sweepline lies in a lane it intersects all segments in this set,
    // but no other segments.
    lanes: Vec<RedBlackTreeMap<i32, Segment>>,
    lane_boundaries: Vec<i32>,
}

impl VerticalSegments {
    fn new(segments: &[Segment]) -> Self {
        struct Event<'a> {
            segment: &'a Segment,
            pos: i32,
            is_add: bool,
        }

        let mut events = Vec::with_capacity(segments.len() * 2);
        for s in segments {
            events.push(Event {
                pos: s.range.0,
                is_add: true,
                segment: s,
            });
            events.push(Event {
                pos: s.range.1,
                is_add: false,
                segment: s,
            });
        }
        events.sort_by_key(|e| e.pos);

        let mut lanes = vec![RedBlackTreeMap::new()];
        let mut lane_boundaries = vec![];

        for e in &events {
            let tree = lanes.last().unwrap();
            let new_tree = if e.is_add {
                tree.insert(e.segment.pos, e.segment.clone())
            } else {
                tree.remove(&e.segment.pos)
            };
            lanes.push(new_tree);
            lane_boundaries.push(e.pos);
        }
        Self {
            lanes,
            lane_boundaries,
        }
    }

    fn intersections(&self, hor: &Segment) -> impl Iterator<Item = &Segment> {
        let lane = match self.lane_boundaries.binary_search(&hor.pos) {
            Ok(l) => l,
            Err(l) => l,
        };
        self.lanes[lane]
            .range(hor.range.0..=hor.range.1)
            .map(|(_k, v)| v)
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;

    macro_rules! test_input {
        ($test_name: ident, $test_fn: ident, $in: expr, $out: expr) => {
            #[test]
            fn $test_name() {
                let input = $in;
                let (w1, w2) = parse_input(&input, 1000);
                assert_eq!($test_fn(&w1, &w2), $out);
            }
        };
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        let input = get_input(3);
        let (w1, w2) = parse_input(&input, 1000);
        b.iter(|| {
            let x = test::black_box(part1(&w1, &w2));
            assert_eq!(x, 399);
        })
    }

    #[bench]
    fn bench_smort(b: &mut test::Bencher) {
        let input = get_input(3);
        let (w1, w2) = parse_input(&input, 1000);
        b.iter(|| {
            let x = test::black_box(part1_smort(&w1, &w2));
        })
    }

    test_input! {test_part1_1, part1, include_str!("../../tests/3/1_1"), 159}
    test_input! {test_part1_2, part1, include_str!("../../tests/3/1_2"), 6}
    test_input! {test_part1_3, part1, include_str!("../../tests/3/1_3"), 135}

    test_input! {test_part2_1, part2, include_str!("../../tests/3/1_1"), 610}
    test_input! {test_part2_2, part2, include_str!("../../tests/3/1_2"), 30}
    test_input! {test_part2_3, part2, include_str!("../../tests/3/1_3"), 410}
}
