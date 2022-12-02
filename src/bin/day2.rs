use std::io;
use std::io::Read;
use ascent::ascent;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum RPS {
    Rock, Paper, Scissors
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Result {
    P1Win, P2Win, Draw
}

use RPS::*;
use crate::Result::*;

ascent! {
   relation outcome(RPS, RPS, Result);
   relation opposite(Result, Result);

   relation play(RPS, RPS);
   relation play_out(Result);
   relation pick(RPS, Result);
   relation pick_out(RPS);

   opposite(P1Win, P2Win);
   opposite(P2Win, P1Win);
 
   outcome(Rock, Scissors, Result::P1Win);
   outcome(Scissors, Paper, Result::P1Win);
   outcome(Paper, Rock, Result::P1Win);
   outcome(x, x, Result::Draw) <-- for x in vec![Rock, Paper, Scissors];
   outcome(p1, p2, r) <-- opposite(r, r2), outcome(p2, p1, r2);

   play_out(o) <-- play(p1, p2), outcome(p1, p2, o);
   pick_out(p2) <-- pick(p1, o), outcome(p1, p2, o);
}


pub fn play(p1: &RPS, p2: &RPS) -> Result {
    let mut prog = AscentProgram::default();
    prog.play = vec![(*p1, *p2)];
    prog.run();
    return prog.play_out[0].0;
}

pub fn pick_result(p1: &RPS, res: &Result) -> RPS {
    let mut prog = AscentProgram::default();
    prog.pick = vec![(*p1, *res)];
    prog.run();
    return prog.pick_out[0].0;
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
        .map(|(p1, p2)| score(p1, &p2))
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