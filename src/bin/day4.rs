use std::io;
use std::io::Read;

type Range = (i32, i32);

pub fn how_many_overlap(input: Vec<(Range,Range)>) -> usize {
    fn contains((s1,e1): Range, (s2,e2): Range) -> bool {
        return (s1 >= s2) && (e1 <= e2);
    }
    return input.iter()
        .filter(|(r1, r2)| contains(*r1, *r2) || contains(*r2, *r1))
        .count();
}

pub fn parse(input: &str) -> Vec<(Range,Range)> {
    fn as_range(input: &str) -> Range {
        let (s1, s2) = input.split_once("-").unwrap();
        return (s1.parse().unwrap(), s2.parse().unwrap());
    }

    return input.lines()
        .map(|l: &str| l.split_once(",").unwrap())
        .map(|(s1, s2): (&str, &str)| (as_range(s1), as_range(s2)))
        .collect();
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");


    let result = how_many_overlap(parse(&input));
    println!("{}", result);
}


#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    "};

    #[test]
    fn test_how_many_overlap() {
        assert_eq!(how_many_overlap(parse(INPUT)), 2);
    }
}