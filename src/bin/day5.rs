use std::collections::VecDeque;
use std::io;
use std::io::Read;
use std::str::FromStr;

use regex::Regex;

struct Crane {
    stacks: Vec<Stack>,
    instructions: Vec<Instruction>
}

type Stack = VecDeque<u8>;

struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl Crane {
    fn move_all(&mut self) {
        for instr in &self.instructions {
            for _ in 1..=instr.amount {
                let c = self.stacks[instr.from - 1].pop_back().unwrap();
                self.stacks[instr.to - 1].push_back(c);
            }
        }
    }

    fn top(&self) -> String {
        let mut result = String::new();
        for s in &self.stacks {
            let c = s.back().unwrap();
            result.push(*c as char);
        }
        return result;
    }
}


impl FromStr for Crane {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bottom_crates_re = Regex::new(r"^( \d  ?)*$").unwrap();
        let instruction_re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

        let num_stacks = (s.lines().nth(0).unwrap().len() + 1) / 4;
        let mut lines = s.lines();
        
        let mut stacks = vec![VecDeque::new(); num_stacks];
        while let Some(line) = lines.next() {
            if bottom_crates_re.is_match(line) {
                break;
            }
            for i in 0..num_stacks {
                let c = line.as_bytes()[i*4 + 1];
                if c != ' ' as u8 {
                    stacks[i].push_front(c)
                }
            }
        }

        // empty line
        lines.next();

        let mut instructions = Vec::new();
        while let Some(line) = lines.next() {
            let caps = instruction_re.captures(line).unwrap();
            let amount = caps[1].parse().unwrap();
            let from = caps[2].parse().unwrap();
            let to = caps[3].parse().unwrap();
            instructions.push(Instruction { amount, from, to });
        }

        return Ok(Crane {
            stacks,
            instructions
        });
    }
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let mut c: Crane = input.parse().unwrap();
    c.move_all();
    println!("{}", c.top());
}


#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
            [D]    
        [N] [C]    
        [Z] [M] [P]
         1   2   3 

        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2
    "};

    #[test]
    fn test_move() {
        let mut c: Crane = INPUT.parse().unwrap();
        c.move_all();
        assert_eq!(c.top(), "CMZ");
    }

}