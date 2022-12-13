use std::io;
use std::io::Read;

use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::{take_while, tag};
use nom::combinator::{map, map_res};
use nom::multi::{many1, separated_list0};
use nom::sequence::{delimited, pair, terminated};

#[derive(Debug)]
pub enum Packet {
    List(Vec<Packet>),
    Elem(i32)
}

fn parse(input: &str) -> Vec<(Packet, Packet)> {
    fn list(input: &str) -> IResult<&str, Packet> {
        map(
            delimited(tag("["), separated_list0(tag(","), packet), tag("]")),
            |l| Packet::List(l)
        )(input)
    }
    fn elem(input: &str) -> IResult<&str, Packet> {
        map(
            map_res(
                take_while(|c: char| c.is_digit(10)),
                |n: &str| n.parse::<i32>()
            ),
            |val| Packet::Elem(val)
        )(input)
    }
    fn packet(input: &str) -> IResult<&str, Packet> {
        alt((list, elem))(input)
    }
    fn all(input: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
        many1(
            terminated(
                pair(
                    terminated(packet, tag("\n")),
                    terminated(packet, tag("\n")),
                    ),
                tag("\n"))
        )(input)
    }

    let (_rest_, result) = all(input).unwrap();
    return result;
}

#[derive(PartialEq, Debug)]
enum TFU {
    True, False, Unknown
}

impl TFU {
    pub fn as_bool(&self) -> bool {
        match self {
            TFU::True => return true,
            TFU::False => return false,
            TFU::Unknown => panic!("Unknown"),
        }
    }
}


pub fn in_order(packets: Vec<(Packet, Packet)>) -> usize {
    fn pair_in_order(packets: (Packet, Packet)) -> TFU {
        use TFU::*;
        match packets {
            (Packet::Elem(l), Packet::Elem(r)) => {
                match l.cmp(&r) {
                    std::cmp::Ordering::Less => return True,
                    std::cmp::Ordering::Greater => return False,
                    std::cmp::Ordering::Equal => return Unknown,
                }
            },
            (Packet::List(l), Packet::List(r)) => {
                let mut l = l.into_iter();
                let mut r = r.into_iter();
                loop {
                    let el = l.next();
                    let er = r.next();
                    match (el, er) {
                        (None, None) => return Unknown,
                        (None, Some(_)) => return True,
                        (Some(_), None) => return False,
                        (Some(pl), Some(pr)) => {
                            let sub_pair = pair_in_order((pl, pr));
                            if sub_pair != Unknown {
                                return sub_pair;
                            }
                        },
                    }
                }
            },
            (Packet::List(l), Packet::Elem(r)) => {
                return pair_in_order((Packet::List(l), Packet::List(vec![Packet::Elem(r)])))
            },
            (Packet::Elem(l), Packet::List(r)) => {
                return pair_in_order((Packet::List(vec![Packet::Elem(l)]), Packet::List(r)))
            },
        }
    }
    
    packets.into_iter()
        .enumerate()
        .filter_map(|(i, p)| {
            if pair_in_order(p).as_bool() {Some(i + 1)} else {None}
        })
        .sum()
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    println!("{}", in_order(parse(&input)));
}


#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_in_order() {
        assert_eq!(in_order(parse(INPUT)), 13);
    }   

    const INPUT: &str = indoc! {"
        [1,1,3,1,1]
        [1,1,5,1,1]
        
        [[1],[2,3,4]]
        [[1],4]
        
        [9]
        [[8,7,6]]
        
        [[4,4],4,4]
        [[4,4],4,4,4]
        
        [7,7,7,7]
        [7,7,7]
        
        []
        [3]
        
        [[[]]]
        [[]]
        
        [1,[2,[3,[4,[5,6,7]]]],8,9]
        [1,[2,[3,[4,[5,6,0]]]],8,9]
    "};

}