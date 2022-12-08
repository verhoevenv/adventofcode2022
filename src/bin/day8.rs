use std::io;
use std::io::Read;

use ndarray::{Array, s, ArrayBase, OwnedRepr, Dim};

type HeightMap = ArrayBase<OwnedRepr<i32>, Dim<[usize; 2]>>;

pub fn count_trees(heights: HeightMap) -> i32 {
    let mut total = 0;

    for ((x, y), elem) in heights.indexed_iter() {
        if heights.slice(s![..x, y]).iter().all(|e| e < elem)
            || heights.slice(s![(x+1)..;-1, y]).iter().all(|e| e < elem)
            || heights.slice(s![x, ..y]).iter().all(|e| e < elem)
            || heights.slice(s![x, (y+1)..;-1]).iter().all(|e| e < elem) {
                total += 1;
        }
    }
    
    return total;
}

pub fn viewing_distance<'a, T: IntoIterator<Item=&'a i32>>(trees: T, height: &i32) -> i32 {
    let mut count = 0;
    for h in trees {
        count += 1;
        if h >= height {
            break;
        }
    }
    return count;
}

pub fn scenic_score(heights: &HeightMap, (x, y): (usize, usize)) -> i32 {
    let elem = heights.get((x, y)).unwrap();

    return viewing_distance(heights.slice(s![..x;-1, y]), elem)
        * viewing_distance(heights.slice(s![(x+1).., y]), elem)
        * viewing_distance(heights.slice(s![x, ..y;-1]), elem)
        * viewing_distance(heights.slice(s![x, (y+1)..]), elem);
}

pub fn highest_scenic_score(heights: HeightMap) -> i32 {
    heights.indexed_iter()
        .map(|(idx, _)| scenic_score(&heights, idx))
        .max().unwrap()
}

pub fn parse(input: &str) -> HeightMap {
    let lines: Vec<_> = input.lines().collect();
    let shape = (lines[0].len(), lines.len());

    let all_heights: Vec<i32> = lines.join("").chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
    let heights = Array::from_shape_vec(shape, all_heights).unwrap();
    
    return heights;
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    println!("{}", highest_scenic_score(parse(&input)));
}


#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        30373
        25512
        65332
        33549
        35390
    "};

    #[test]
    fn test_trees() {
        assert_eq!(count_trees(parse(INPUT)), 21);
    }


    #[test]
    fn test_scenic_score() {
        assert_eq!(scenic_score(&parse(INPUT), (1, 2)), 4);
        assert_eq!(scenic_score(&parse(INPUT), (3, 2)), 8);
        assert_eq!(highest_scenic_score(parse(INPUT)), 8);
    }

}