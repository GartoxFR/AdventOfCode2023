#![allow(unused)]

use nom::branch::*;
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

fn hash(s: &str) -> u32 {
    s.chars()
        .fold(0, |hash, current| ((hash + current as u32) * 17) % 256)
}

fn solve(input: &str) -> String {
    input
        .lines().next().unwrap()
        .split(',')
        .map(hash)
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(solve(input), "1320");
    }
}
