use std::collections::HashSet;
use std::io;
use std::io::Read;

use regex::Regex;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
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
    #[inline]
    pub fn dist(&self, other: &XY) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sensor {
    position: XY,
    beacon: XY,
    distance: i32,
}

impl Sensor {
    pub fn new(position: XY, beacon: XY) -> Self {
        let distance = position.dist(&beacon);
        Sensor { 
            position,
            beacon,
            distance,
         }
    }
}

fn parse(input: &str) -> Vec<Sensor> {
    let mut result = Vec::new();
    let sensor_re = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();

    for line in input.lines() {
        let sensor_cap = sensor_re.captures(line).unwrap();
        result.push(Sensor::new(
            xy!(sensor_cap[1].parse().unwrap(), sensor_cap[2].parse().unwrap()),
            xy!(sensor_cap[3].parse().unwrap(), sensor_cap[4].parse().unwrap())
        ))
    }

    return result;
}

pub fn no_beacon(y: i32, sensors: Vec<Sensor>) -> usize {
    let mut positions_covered = HashSet::new();
    for sensor in sensors {
        let chord_length = sensor.distance - (sensor.position.y - y).abs();
        if chord_length > 0 {
            let min_x = sensor.position.x - chord_length;
            let max_x = sensor.position.x + chord_length;
            for x in min_x..max_x {
                positions_covered.insert(x);
            }
        }
    }
    return positions_covered.len();
}


pub fn tuning_frequency(max_xy: i32, sensors: Vec<Sensor>) -> u64 {
    for x in 0..=max_xy {
        let mut y = 0;
        let mut changed = true;
        while changed && y <= max_xy {
            changed = false;
            for s in &sensors {
                let chord_length = s.distance - s.position.dist(&xy!(x, y));
                if chord_length >= 0 {
                    // we are in range of a sensor, step out of range
                    y = y + chord_length + 1;
                    changed = true;    
                }
            }
        }
        if y <= max_xy {
            return x as u64 * 4000000 + y as u64;
        }
    }
    unreachable!();
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let result = tuning_frequency(4000000, parse(&input));

    println!("{}", result);
}


#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_no_beacon_on_row() {
        assert_eq!(no_beacon(10, parse(INPUT)), 26);
    } 
    
    #[test]
    fn test_tuning_frequency() {
        assert_eq!(tuning_frequency(20, parse(INPUT)), 56000011);
    }     

    const INPUT: &str = indoc! {"
        Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3
    "};

}