use std::collections::{HashMap, HashSet};
use std::io;
use std::io::Read;
use std::str::FromStr;

type XY = (i8, i8);
type Height = u8;

pub struct HeightMap {
    heights: HashMap<XY, Height>,
    pub start: XY,
    pub end: XY,
}

impl FromStr for HeightMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut x: i8 = 0;
        let mut y: i8 = 0;
        let mut start = (-1, -1);
        let mut end = (-1, -1);
        let mut heights = HashMap::new();
        for c in s.chars() {
            match c {
                '\n' => {
                    x = 0;
                    y += 1;
                },
                'S' => {
                    heights.insert((x, y), 'a' as u8);
                    start = (x, y);
                    x += 1;
                }
                'E' => {
                    heights.insert((x, y), 'z' as u8);
                    end = (x, y);
                    x += 1;
                }
                _ => {
                    heights.insert((x, y), c as u8);
                    x += 1;
                }
            }
        }

        return Ok(HeightMap {heights, start, end})
    }
}

impl HeightMap {
    pub fn get(&self, xy: &XY) -> Height {
        return *self.heights.get(xy).unwrap();
    }
    
    pub fn all(&self) -> HashSet<XY> {
        return self.heights.keys().cloned().collect();
    }

    pub fn neighbours(&self, xy: &XY) -> Vec<XY> {
        let mut result = Vec::with_capacity(4);
        for dir in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let new_xy = (xy.0 + dir.0, xy.1 + dir.1);
            if self.heights.contains_key(&new_xy) {
                result.push(new_xy);
            }
        }
        return result;
    }
    
    pub fn exits(&self, xy: &XY) -> Vec<XY> {
        let height = self.get(xy);
        return self.neighbours(xy).iter()
                .filter(|n| self.get(n) >= height - 1)
                .cloned()
                .collect();
    }

    pub fn path<F>(&self, from: XY, is_end: F) -> Vec<XY> 
        where F: Fn(XY) -> bool {
        //Dijkstra
        let mut distances: HashMap<XY, u64> = HashMap::new();
        for xy in self.all() {
            distances.insert(xy, u64::MAX);
        }
        let mut unvisited = self.all();
        let mut current = from;
        let mut end = (-1, -1);
        let mut path_found = false;
        distances.insert(current, 0);

        while !path_found {
            let dist = distances.get(&current).unwrap().clone();
            let exits = self.exits(&current);
            for exit in exits {
                if unvisited.contains(&exit) {
                    let curr_dist = distances.get_mut(&exit).unwrap();
                    if *curr_dist > dist {
                        *curr_dist = dist + 1;
                    }
                }
            }
            unvisited.remove(&current);
            if is_end(current) {
                path_found = true;
                end = current;
            } else {
                current = *unvisited.iter().min_by_key(|e| distances.get(e).unwrap()).unwrap();
            }
        }

        let mut path = Vec::new();
        let mut current = end;
        while current != from {
            path.push(current);
            current = *self.neighbours(&current).iter()
                .filter(|n| *distances.get(&n).unwrap() == (distances.get(&current).unwrap() - 1))
                .nth(0)
                .unwrap();
        }
        path.reverse();
        return path;
    }
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let hm: HeightMap = input.parse().unwrap();
    println!("{}", hm.path(hm.end, |xy| hm.get(&xy) == ('a' as u8)).len());
}


#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_path() {
        let hm: HeightMap = INPUT.parse().unwrap();
        assert_eq!(hm.path(hm.end, |xy| xy == hm.start).len(), 31);
    }    

    #[test]
    fn test_any_path() {
        let hm: HeightMap = INPUT.parse().unwrap();
        assert_eq!(hm.path(hm.end, |xy| hm.get(&xy) == ('a' as u8)).len(), 29);
    }   

    const INPUT: &str = indoc! {"
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi
    "};

}