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


pub fn how_many_visited(movements: &str) -> usize {
    let mut visited: HashSet<XY> = HashSet::new();
    let mut head = xy!(0, 0);
    let mut tail = xy!(0, 0);
    visited.insert(tail);

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
            head = head + dir;
            tail.step_to(&head);
            visited.insert(tail);
        }
    }

    return visited.len();
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    println!("{}", how_many_visited(&input));
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
    fn test_trees() {
        assert_eq!(how_many_visited(INPUT), 13);
    }

}