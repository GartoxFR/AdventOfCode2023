#![allow(unused)]

use std::collections::HashMap;
use std::rc::Rc;

use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

fn main() {
    let input = include_str!("../../input1.txt");
    println!("{}", solve(input));
}

#[derive(Debug)]
enum Direction {
    Right,
    Left,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Wrong direction char"),
        }
    }
}

type Map<'a> = HashMap<&'a str, (&'a str, &'a str)>;

fn instructions(input: &str) -> IResult<&str, Vec<Direction>> {
    many1(map(one_of("LR"), Direction::from_char))(input)
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let line = separated_pair(
        alpha1,
        tuple((space0, tag("="), space0)),
        delimited(
            tag("("),
            separated_pair(alpha1, tag(", "), alpha1),
            tag(")"),
        ),
    );

    map(separated_list1(newline, line), |lines| {
        lines.into_iter().collect()
    })(input)
}

fn solve(input: &str) -> String {
    let (_, (instructions, map)) = separated_pair(instructions, count(newline, 2), parse_map)(input).unwrap();
    let mut i = 0;
    let mut current = "AAA";

    while current != "ZZZ" {
        current = match instructions[i % instructions.len()] {
            Direction::Left => map.get(current).unwrap().0,
            Direction::Right => map.get(current).unwrap().1,
        };
        i += 1;
    }

    i.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(solve(input), "6");
    }
}
