use std::collections::HashSet;
use std::io;
use std::io::Read;

pub fn find_double(strs: &[&str]) -> char {
    let doubles = strs.iter()
        .map(|s| s.chars().collect::<HashSet<char>>())
        .reduce(|acc, item| acc.intersection(&item).copied().collect())
        .unwrap();

    assert!(doubles.len() == 1);
    return *doubles.iter().next().unwrap();
}

pub fn priority(item: char) -> i32 {
    let val = match item {
        c @ 'A'..='Z' => c as u8 - 'A' as u8 + 27,
        c @ 'a'..='z' => c as u8 - 'a' as u8 + 1,
        c @ _ => panic!("Unexpected character {}", c)
    };
    return val as i32;
}

pub fn shared_item(input: &str) -> i32 {
    return input.lines()
        .map(|l: &str| l.split_at(l.len() / 2) )
        .map(|(a, b)| [a,b])
        .map(|x| find_double(&x))
        .map(priority)
        .sum();
}

pub fn shared_item2(input: &str) -> i32 {
    let all_lines: Vec<_> = input.lines().collect();
    return all_lines.chunks(3)
        .map(find_double)
        .map(priority)
        .sum();
}


fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");


    let result = shared_item2(&input);
    println!("{}", result);
}


#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
    "};

    #[test]
    fn test_shared_item() {
        assert_eq!(shared_item(INPUT), 157);
    }

    #[test]
    fn test_shared_item2() {
        assert_eq!(shared_item2(INPUT), 70);
    }
}