use std::io;
use std::io::Read;

pub fn most_calories(list: Vec<Vec<i32>>) -> i32 {
    return list.iter()
        .map(|l| l.iter().sum())
        .max()
        .unwrap()
}

pub fn top_three_calories(list: Vec<Vec<i32>>) -> i32 {
    let mut calories: Vec<i32> = list.iter()
        .map(|l| l.iter().sum())
        .collect();

    calories.sort();

    return calories.iter().rev()
        .take(3)
        .sum();
}

pub fn parse(input: &str) -> Vec<Vec<i32>> {
    let mut list: Vec<Vec<i32>> = Vec::new();
    let mut current_elf: Vec<i32> = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            list.push(current_elf);
            current_elf = Vec::new();
        } else {
            let number: i32 = line.trim().parse().expect("Failed to parse input");
            current_elf.push(number);    
        }
    }

    return list
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");


    let result = top_three_calories(parse(&input));
    println!("{}", result);
}


#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        1000
        2000
        3000
        
        4000
        
        5000
        6000
        
        7000
        8000
        9000
        
        10000

    "};

    #[test]
    fn test_most_calories() {
        assert_eq!(most_calories(parse(INPUT)), 24000);
    }

    #[test]
    fn test_top_three_calories() {
        assert_eq!(top_three_calories(parse(INPUT)), 45000);
    }
}