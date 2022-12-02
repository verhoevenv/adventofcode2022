use std::io;
use std::io::Read;

#[derive(PartialEq, Eq)]
pub enum RPS {
    Rock, Paper, Scissors
}

pub enum Result {
    P1Win, P2Win, Draw
}

pub fn play(p1: &RPS, p2: &RPS) -> Result {
    return match (p1, p2) {
        (RPS::Paper, RPS::Rock) => Result::P1Win,
        (RPS::Rock, RPS::Paper) => Result::P2Win,
        (RPS::Rock, RPS::Scissors) => Result::P1Win,
        (RPS::Scissors, RPS::Rock) => Result::P2Win,
        (RPS::Scissors, RPS::Paper) => Result::P1Win,
        (RPS::Paper, RPS::Scissors) => Result::P2Win,
        _ => Result::Draw,
    };
}

pub fn pick_result<'a>(p1: &'a RPS, res: &'a Result) -> &'a RPS {
    return match (p1, res) {
        (RPS::Rock, Result::P1Win) => &RPS::Scissors,
        (RPS::Rock, Result::P2Win) => &RPS::Paper,
        (RPS::Paper, Result::P1Win) => &RPS::Rock,
        (RPS::Paper, Result::P2Win) => &RPS::Scissors,
        (RPS::Scissors, Result::P1Win) => &RPS::Paper,
        (RPS::Scissors, Result::P2Win) => &RPS::Rock,
        (x, Result::Draw) => x,
    };
}

pub fn score(p1: &RPS, p2: &RPS) -> i32 {
    let shape = match p2 {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    };

    let outcome = match play(p1, p2) {
        Result::P1Win => 0,
        Result::Draw => 3,
        Result::P2Win => 6,
    };

    return shape + outcome;
}

pub fn follow_guide1(list: Vec<(RPS, RPS)>) -> i32 {
    return list.iter()
        .map(|(p1, p2)| score(p1, p2))
        .sum()
}

pub fn follow_guide2(list: Vec<(RPS, Result)>) -> i32 {
    return list.iter()
        .map(|(p1, res)| (p1, pick_result(p1, res)))
        .map(|(p1, p2)| score(p1, p2))
        .sum()
}

pub fn parse1(input: &str) -> Vec<(RPS, RPS)> {
    let mut list: Vec<(RPS, RPS)> = Vec::new();
    for line in input.lines() {
        let (p1_s, p2_s) = line.split_once(" ").unwrap();
        let p1 = match p1_s {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => RPS::Scissors,
            _ => panic!("Unexpected string {}", p1_s)
        };
        let p2 = match p2_s {
            "X" => RPS::Rock,
            "Y" => RPS::Paper,
            "Z" => RPS::Scissors,
            _ => panic!("Unexpected string {}", p2_s)
        };
        list.push((p1, p2));
    }
    return list;
}

pub fn parse2(input: &str) -> Vec<(RPS, Result)> {
    let mut list: Vec<(RPS, Result)> = Vec::new();
    for line in input.lines() {
        let (p1_s, p2_s) = line.split_once(" ").unwrap();
        let p1 = match p1_s {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => RPS::Scissors,
            _ => panic!("Unexpected string {}", p1_s)
        };
        let p2 = match p2_s {
            "X" => Result::P1Win,
            "Y" => Result::Draw,
            "Z" => Result::P2Win,
            _ => panic!("Unexpected string {}", p2_s)
        };
        list.push((p1, p2));
    }
    return list;
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");


    let result = follow_guide2(parse2(&input));
    println!("{}", result);
}


#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        A Y
        B X
        C Z
    "};

    #[test]
    fn test_follow_guide1() {
        assert_eq!(follow_guide1(parse1(INPUT)), 15);
    }

    #[test]
    fn test_follow_guide2() {
        assert_eq!(follow_guide2(parse2(INPUT)), 12);
    }

}