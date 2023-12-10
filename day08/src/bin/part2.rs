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
        alphanumeric1,
        tuple((space0, tag("="), space0)),
        delimited(
            tag("("),
            separated_pair(alphanumeric1, tag(", "), alphanumeric1),
            tag(")"),
        ),
    );

    map(separated_list1(newline, line), |lines| {
        lines.into_iter().collect()
    })(input)
}

fn gcd(mut n: usize, mut m: usize) -> usize {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if (m < n) {
            std::mem::swap(&mut n, &mut m);
        }
        m %= n;
    }
    n
}

fn lcm(n: usize, m: usize) -> usize {
    (n * m).div_euclid(gcd(n, m))
}

fn solve(input: &str) -> String {
    let (_, (instructions, map)) =
        separated_pair(instructions, count(newline, 2), parse_map)(input).unwrap();
    let mut starts: Vec<_> = map
        .keys()
        .copied()
        .filter(|node| node.ends_with('A'))
        .collect();

    starts
        .into_iter()
        .map(|mut current| {
            let mut i = 0;
            while !current.ends_with('Z') {
                current = match instructions[i % instructions.len()] {
                    Direction::Left => map.get(current).unwrap().0,
                    Direction::Right => map.get(current).unwrap().1,
                };
                i += 1;
            }
            i
        })
        .reduce(lcm)
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!(solve(input), "6");
    }
}
