use std::cmp::Ordering;
use std::io;
use std::io::Read;

use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::{take_while, tag};
use nom::combinator::{map, map_res};
use nom::multi::separated_list0;
use nom::sequence::{delimited, pair, terminated};

#[derive(Debug, Clone, PartialEq, Eq)]
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
        separated_list0(
            tag("\n"),
            pair(
                terminated(packet, tag("\n")),
                terminated(packet, tag("\n")),
                ),
        )(input)
    }

    let (rest, result) = all(input).unwrap();
    assert!(rest.is_empty());
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

fn pair_in_order(packets: (&Packet, &Packet)) -> TFU {
    use TFU::*;
    match packets {
        (Packet::Elem(l), Packet::Elem(r)) => {
            match l.cmp(&r) {
                Ordering::Less => return True,
                Ordering::Greater => return False,
                Ordering::Equal => return Unknown,
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
            return pair_in_order((&Packet::List(l.to_owned()), &Packet::List(vec![Packet::Elem(r.to_owned())])))
        },
        (Packet::Elem(l), Packet::List(r)) => {
            return pair_in_order((&Packet::List(vec![Packet::Elem(l.to_owned())]), &Packet::List(r.to_owned())))
        },
    }
}


pub fn put_in_order(packets: Vec<(Packet, Packet)>) -> usize {
    let divider1 = Packet::List(vec![Packet::List(vec![Packet::Elem(2)])]);
    let divider2 = Packet::List(vec![Packet::List(vec![Packet::Elem(6)])]);
    let mut all_packets: Vec<_> = packets.into_iter()
        .flat_map(|(a, b)| vec![a, b])
        .chain(vec![divider1.clone(), divider2.clone()].into_iter())
        .collect();

    all_packets.sort_by(|p1, p2| {
        if pair_in_order((p1, p2)).as_bool() {Ordering::Less} else {Ordering::Greater}
    });
    
    let pos1 = all_packets.iter().position(|d| d == &divider1).unwrap() + 1;
    let pos2 = all_packets.iter().position(|d| d == &divider2).unwrap() + 1;

    return pos1 * pos2;
}

pub fn sum_in_order(packets: Vec<(Packet, Packet)>) -> usize {
    packets.into_iter()
        .enumerate()
        .filter_map(|(i, (p1, p2))| {
            if pair_in_order((&p1, &p2)).as_bool() {Some(i + 1)} else {None}
        })
        .sum()
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    println!("{}", put_in_order(parse(&input)));
}


#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_sum_in_order() {
        assert_eq!(sum_in_order(parse(INPUT)), 13);
    }   

    #[test]
    fn test_put_in_order() {
        assert_eq!(put_in_order(parse(INPUT)), 140);
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