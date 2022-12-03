use std::collections::HashSet;
use std::io;
use std::io::Read;

fn find_double((str1, str2) : (&str, &str)) -> i32 {
    let s1: HashSet<_> = str1.chars().collect();
    let s2: HashSet<_> = str2.chars().collect();

    let doubles: Vec<_> = s1.intersection(&s2).collect();
    assert!(doubles.len() == 1);
    return priority(doubles[0]);
}

fn priority(item: &char) -> i32 {
    let val = match *item {
        c @ 'A'..='Z' => c as u8 - 'A' as u8 + 27,
        c @ 'a'..='z' => c as u8 - 'a' as u8 + 1,
        c @ _ => panic!("Unexpected character {}", c)
    };
    return val as i32;
}

fn shared_item(input: &str) -> i32 {
    return input.lines()
        .map(|l: &str| l.split_at(l.len() / 2) )
        .map(find_double)
        .sum();
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");


    let result = shared_item(&input);
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

}