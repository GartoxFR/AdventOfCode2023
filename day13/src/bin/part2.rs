#![allow(unused)]

use nom::branch::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Terrain {
    Rock,
    Ash,
}

impl Terrain {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Self::Rock,
            '.' => Self::Ash,
            _ => panic!("Unknown terrain character"),
        }
    }
}

type Pattern = Vec<Vec<Terrain>>;
fn parse_input(input: &str) -> IResult<&str, Vec<Pattern>> {
    separated_list1(
        count(newline, 2),
        separated_list1(newline, many1(map(one_of("#."), Terrain::from_char))),
    )(input)
}

fn main() {
    let input = include_str!("../../input1.txt");
    println!("{}", solve(input));
}

fn find_vertical_reflection(pattern: &Pattern) -> usize {
    let rows = pattern.len();
    for i in 1..rows {
        let reflection_size = i.min(rows - i);
        let left = &pattern[i - reflection_size..i];
        let right = &pattern[i..i + reflection_size];

        if left
            .iter()
            .zip(right.iter().rev())
            .map(|(a, b)| a.iter().zip(b).filter(|(a, b)| a != b).count())
            .sum::<usize>()
            == 1
        {
            return i;
        }
    }

    0
}
fn transpose(pattern: &Pattern) -> Pattern {
    (0..pattern[0].len())
        .map(|col| (0..pattern.len()).map(|row| pattern[row][col]).collect())
        .collect()
}

fn solve(input: &str) -> String {
    let (_, patterns) = parse_input(input).unwrap();
    patterns
        .into_iter()
        .map(|pattern| {
            100 * find_vertical_reflection(&pattern)
                + find_vertical_reflection(&transpose(&pattern))
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(solve(input), "400");
    }
}
