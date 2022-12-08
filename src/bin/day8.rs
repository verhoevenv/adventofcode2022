use std::io;
use std::io::Read;

use ndarray::{Array, s};

pub fn count_trees(input: &str) -> i32 {
    let lines: Vec<_> = input.lines().collect();
    let shape = (lines[0].len(), lines.len());

    let all_heights: Vec<i32> = lines.join("").chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
    let heights = Array::from_shape_vec(shape, all_heights).unwrap();
    
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

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    println!("{}", count_trees(&input));
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
        assert_eq!(count_trees(INPUT), 21);
    }

}