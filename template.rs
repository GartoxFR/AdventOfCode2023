#![allow(unused)]

use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::branch::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

fn main() {
    let input = include_str!("../../input1.txt");
    println!("{}", solve(input));
}

fn solve(input: &str) -> String {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "";
        assert_eq!(solve(input), "");
    }
}
