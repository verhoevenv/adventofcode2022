use std::{io, fmt};
use std::io::Read;

use derive_more::{Add, Sub, AddAssign};
use nom::IResult;
use nom::bytes::complete::{take_while, tag};
use nom::combinator::{map, map_res};
use nom::multi::separated_list0;
use nom::sequence::separated_pair;

use ndarray::Array2;


#[derive(Debug, Add, Sub, AddAssign, Copy, Clone, Hash, Eq, PartialEq)]
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
    pub fn as_index(&self, relative_to: &XY) -> [usize; 2] {
        return [(self.x - relative_to.x) as usize, (self.y - relative_to.y) as usize];
    }

    pub fn direction(&self, other: &Self) -> Self {
        let diff = *other - *self;
        return xy!(diff.x.signum(), diff.y.signum());
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Filling {
    Air, Rock, Sand
}
type Map = Array2<Filling>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cave {
    topleft: XY,
    bottomright: XY,
    map: Map,
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut display_topleft = self.bottomright;
        let mut display_bottomright = self.topleft;
        for y in self.topleft.y..=self.bottomright.y {
            for x in self.topleft.x..=self.bottomright.x {  
                let idx = xy!(x,y).as_index(&self.topleft);
                match self.map[idx] {
                    Filling::Air => {},
                    Filling::Rock | Filling::Sand => {
                        display_topleft.x = display_topleft.x.min(x);
                        display_topleft.y = display_topleft.y.min(y);
                        display_bottomright.x = display_bottomright.x.max(x);
                        display_bottomright.y = display_bottomright.y.max(y);
                    },
                };
            }
        }

        for y in display_topleft.y..=display_bottomright.y {
            for x in display_topleft.x..=display_bottomright.x {  
                let idx = xy!(x,y).as_index(&self.topleft);
                let symbol = match self.map[idx] {
                    Filling::Air => '.',
                    Filling::Rock => '#',
                    Filling::Sand => 'o',
                };
                write!(f, "{}", symbol)?
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

pub struct Path {
    points: Vec<XY>
}

fn parse(input: &str) -> Cave {
    fn path(input: &str) -> IResult<&str, Path> {
        map(
            separated_list0(
                tag(" -> "),
                xy
            ),
            |val| Path {points: val}
        )(input)
    }
    fn num(input: &str) -> IResult<&str, i32> {
        map_res(
            take_while(|c: char| c.is_digit(10)),
            |n: &str| n.parse::<i32>()
        )(input)
    }    
    fn xy(input: &str) -> IResult<&str, XY> {
        map(
            separated_pair(num,tag(","),num),
            |(x,y): (i32, i32)| xy!(x, y)
        )(input)
    }
    fn all(input: &str) -> IResult<&str, Cave> {
        let (input, paths) = separated_list0(
            tag("\n"),
            path,
        )(input)?;

        let miny = paths.iter().flat_map(|path| path.points.iter()).map(|xy| xy.y).min().unwrap();
        let maxy = paths.iter().flat_map(|path| path.points.iter()).map(|xy| xy.y).max().unwrap();

        // provision enough room for the worst-case pyramid
        let floor = maxy + 2;
        let height = floor;
        let width_one_side = height;
        let topleft = xy!(500 - width_one_side - 1, miny.min(0) - 1);
        let bottomright = xy!(500 + width_one_side + 1, floor);
        
        let shape = bottomright - topleft + xy!(1, 1);
        let shape = (shape.x.try_into().unwrap(), shape.y.try_into().unwrap());

        let mut map = Array2::from_elem(shape, Filling::Air);
        for path in paths {
            for line in path.points.windows(2) {
                let from = line[0];
                let to = line[1];
                let dir = from.direction(&to);
                let mut cur = from;
                map[cur.as_index(&topleft)] = Filling::Rock;
                while cur != to {
                    cur += dir;
                    map[cur.as_index(&topleft)] = Filling::Rock;
                }
            }
        }

        Ok((input, Cave { topleft, bottomright, map } ))
    }

    let (rest, result) = all(input).unwrap();
    assert!(rest.is_empty());
    return result;
}

pub fn drop_sand(cave: &Cave) -> Option<XY> {
    let mut sand_at = xy!(500, 0);
    if cave.map[sand_at.as_index(&cave.topleft)] == Filling::Sand {
        return None;
    }

    'down: loop {   
        for possible_movement in [xy!(0, 1), xy!(-1, 1), xy!(1, 1)] {
            let next_pos = sand_at + possible_movement;
            if next_pos.y > cave.bottomright.y {
                return None;
            }
            if cave.map[next_pos.as_index(&cave.topleft)] == Filling::Air {
                sand_at = next_pos;
                continue 'down;
            }
        }
        return Some(sand_at);
    }
}

pub fn create_floor(mut cave: Cave) -> Cave {
    for x in cave.topleft.x..=cave.bottomright.x {  
        cave.map[xy!(x, cave.bottomright.y).as_index(&cave.topleft)] = Filling::Rock;
    }
    return cave;
}

pub fn sand(cave: &mut Cave) -> usize {
    let mut count = 0;
    while let Some(new_sand) = drop_sand(cave) {
        count += 1;
        cave.map[new_sand.as_index(&cave.topleft)] = Filling::Sand;
    }
    return count;
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let mut cave = create_floor(parse(&input));
    
    // print!("{}", cave);

    let result = sand(&mut cave);

    // print!("{}", cave);

    println!("{}", result);
}


#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_sand() {
        assert_eq!(sand(&mut parse(INPUT)), 24);
    }   

    #[test]
    fn test_until_blocked() {
        assert_eq!(sand(&mut create_floor(parse(INPUT))), 93);
    }   

    const INPUT: &str = indoc! {"
        498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9
    "};

}