use std::collections::HashSet;
use std::io;
use std::io::Read;

use derive_more::{Add, Sub};


#[derive(Debug, Add, Sub, Copy, Clone, Hash, Eq, PartialEq)]
pub struct XY {
    x: i32,
    y: i32,
}

macro_rules! xy {
    ($x:expr, $y:expr) => {
        XY{x: $x, y:$y}
    };
}

impl XY {
    pub fn adjacent(&self, b: &XY) -> bool {
        let diff = *self - *b;
        return diff.x.abs() <= 1 && diff.y.abs() <= 1;
    }

    pub fn step_to(&mut self, head: &XY) {
        if self.adjacent(head) {
            return;
        }
        let diff = *head - *self;
        let delta = match diff {
            XY{x, y: 0} => xy!(x.signum(), 0),
            XY{x: 0, y} => xy!(0, y.signum()),
            XY{x, y} => xy!(x.signum(), y.signum()),
        };
        *self = *self + delta;
    }
}


pub fn how_many_visited(movements: &str, rope_length: usize) -> usize {
    let mut visited: HashSet<XY> = HashSet::new();
    let mut rope = vec![];
    rope.resize(rope_length, xy!(0,0));
    let tail = rope.last().unwrap();
    visited.insert(*tail);

    for line in movements.lines() {
        let (dir, n) = line.split_once(" ").unwrap();
        let dir = dir.chars().next().unwrap();
        let n: i32 = n.parse().unwrap();

        let dir = match dir {
            'U' => xy!(0, 1),
            'D' => xy!(0, -1),
            'R' => xy!(1, 0),
            'L' => xy!(-1, 0),
            _ => panic!("unknown direction {}", dir)
        };

        for _ in 0..n {
            let head = rope.get_mut(0).unwrap();
            *head = *head + dir;
            for rope_segment in 1..rope_length {
                let prev_seg = rope.get(rope_segment - 1).unwrap().to_owned();
                let seg = rope.get_mut(rope_segment).unwrap();
                seg.step_to(&prev_seg);
            }
            let tail = rope.last().unwrap();
            visited.insert(*tail);
        }
    }

    return visited.len();
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    println!("{}", how_many_visited(&input, 10));
}


#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2
    "};

    #[test]
    fn test_rope_2() {
        assert_eq!(how_many_visited(INPUT, 2), 13);
    }


    #[test]
    fn test_rope_10_ex_1() {
        assert_eq!(how_many_visited(INPUT, 10), 1);
    }

    #[test]
    fn test_rope_10_ex_2() {
        const INPUT: &str = indoc! {"
            R 5
            U 8
            L 8
            D 3
            R 17
            D 10
            L 25
            U 20
        "};
        assert_eq!(how_many_visited(INPUT, 10), 36);
    }
}