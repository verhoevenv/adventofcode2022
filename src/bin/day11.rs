use std::collections::VecDeque;
use std::io;
use std::io::Read;
use std::str::FromStr;

use regex::Regex;

pub struct Monkey {
    items: VecDeque<i32>,
    inspections: i32,
    operation: Operation,
    test: Test,
    if_true: usize,
    if_false: usize,
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items_re = Regex::new(r"(?m)Starting items: ([0-9, ]+)").unwrap();
        let operation_re = Regex::new(r"(?m)Operation: (.+)$").unwrap();
        let test_re = Regex::new(r"(?m)Test: (.+)$").unwrap();
        let true_re = Regex::new(r"(?m)If true: throw to monkey (\d+)$").unwrap();
        let false_re = Regex::new(r"(?m)If false: throw to monkey (\d+)$").unwrap();

        let items = items_re.captures(s).unwrap().get(1).unwrap().as_str()
                                    .split(", ").map(|n| n.parse().unwrap()).collect();
    
        let operation = operation_re.captures(s).unwrap().get(1).unwrap().as_str().parse()?;
        let test = test_re.captures(s).unwrap().get(1).unwrap().as_str().parse()?;
        let if_true = true_re.captures(s).unwrap().get(1).unwrap().as_str().parse().unwrap();
        let if_false = false_re.captures(s).unwrap().get(1).unwrap().as_str().parse().unwrap();

        return Ok(Monkey {
            items,
            inspections: 0,
            operation,
            test,
            if_true,
            if_false
         })
    }
}

pub struct Operation {
    op: Operator,
    arg: Argument,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let op_re = Regex::new(r"new = old (.) (.+)$").unwrap();
        let caps = op_re.captures(s).unwrap();

        let op = match caps.get(1).unwrap().as_str() {
            "*" => Operator::Mul,
            "+" => Operator::Add,
            _ => unreachable!(),
        };

        let arg = match caps.get(2).unwrap().as_str() {
            "old" => Argument::Old,
            v @ _ => {
                let val = v.parse().unwrap();
                Argument::Const(val)
            },
        };

        return Ok(Operation {
            op,
            arg
        });
    }
}

pub enum Operator {
    Add, Mul
}

pub enum Argument {
    Old, Const(i32)
}

pub struct Test {
    divisor: i32
}

impl FromStr for Test {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"divisible by (\d+)").unwrap();
        let x = re.captures(s).unwrap();
        return Ok(Self { 
            divisor: x.get(1).unwrap().as_str().parse().unwrap()
         })
    }
}

pub fn monkey_business(input: &str) -> i32 {
    let mut monkeys : Vec<Monkey> = input.split("\n\n").map(|m| m.parse().unwrap()).collect();

    for _round in 1..=20 {
        for monkey in 0..monkeys.len() {
            let m = &mut monkeys[monkey];
            let mut inserts: Vec<(usize, i32)> = vec![];
            while let Some(item) = m.items.pop_front() {
                m.inspections += 1;
                let val = match m.operation.arg {
                    Argument::Old => item,
                    Argument::Const(c) => c,
                };
                let new_worry = match m.operation.op {
                    Operator::Add => item + val,
                    Operator::Mul => item * val,
                };
                let new_worry = new_worry / 3;

                let new_monkey = match new_worry % m.test.divisor {
                    0 => m.if_true,
                    _ => m.if_false,
                };

                inserts.push((new_monkey, new_worry));
            }
            // splitting this out of the above loop to reassure the borrow checker,
            // it can't know we aren't changing the monkey we are handling
            for (new_monkey, new_worry) in inserts {
                monkeys[new_monkey].items.push_back(new_worry);
            }
        }
    }

    
    let mut inspections: Vec<_> = monkeys.iter().map(|m| m.inspections).collect();
    inspections.sort();
    inspections.reverse();
    return inspections[0] * inspections[1];
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    println!("{}", monkey_business(&input));
}


#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_mokney_business() {
        assert_eq!(monkey_business(INPUT), 10605);
    }

    const INPUT: &str = indoc! {"
        Monkey 0:
            Starting items: 79, 98
            Operation: new = old * 19
            Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3
        
        Monkey 1:
            Starting items: 54, 65, 75, 74
            Operation: new = old + 6
            Test: divisible by 19
            If true: throw to monkey 2
            If false: throw to monkey 0
        
        Monkey 2:
            Starting items: 79, 60, 97
            Operation: new = old * old
            Test: divisible by 13
            If true: throw to monkey 1
            If false: throw to monkey 3
        
        Monkey 3:
            Starting items: 74
            Operation: new = old + 3
            Test: divisible by 17
            If true: throw to monkey 0
            If false: throw to monkey 1
    "};

}