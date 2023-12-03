#![allow(unused)]

use std::collections::BTreeSet;

use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{newline, one_of};
use nom::error::{convert_error, ParseError, VerboseError};
use nom::multi::{fold_many0, fold_many1, many0, many1};
use nom::sequence::terminated;
use nom::IResult;
use nom::{character::complete::digit1, combinator::map};

fn main() {
    let input = include_str!("../../input2.txt");
    println!("{}", solve(input));
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Gear,
    Number(u32),
    Other,
}

#[derive(Debug)]
struct Gear {
    row: usize,
    col: usize,
}

#[derive(Debug, Default)]
struct Plan {
    gears: Vec<Gear>,
    cells: Vec<Vec<Cell>>,
}

#[derive(Debug, Default)]
struct LineAccumulator {
    gears: Vec<Gear>,
    cells: Vec<Cell>,
}

#[derive(Debug)]
enum ParsedCells {
    Number(u32, usize), // value, len
    Gear,
    Other,
}

fn solve(input: &str) -> String {
    let mut number_parser = map(digit1, |digits: &str| {
        ParsedCells::Number(digits.parse().unwrap(), digits.len())
    });
    let mut dot_parser = map(tag("*"), |_| ParsedCells::Gear);
    let mut symbol_parser = map(one_of("[]$%^&.'{()}=+-<>\"~`|_/@\\@#!?"), |_| {
        ParsedCells::Other
    });

    let mut line_parser = fold_many1(
        alt((number_parser, dot_parser, symbol_parser)),
        LineAccumulator::default,
        |mut acc, val| {
            match val {
                ParsedCells::Number(val, len) => {
                    acc.cells
                        .extend(std::iter::repeat(Cell::Number(val)).take(len));
                }
                ParsedCells::Gear => {
                    acc.gears.push(Gear {
                        row: 0,
                        col: acc.cells.len(),
                    });
                    acc.cells.push(Cell::Gear);
                }
                ParsedCells::Other => {
                    acc.cells.push(Cell::Other);
                }
            }
            acc
        },
    );

    let mut board_parser = fold_many1(
        terminated(line_parser, many0(newline)),
        Plan::default,
        |mut plan, line| {
            let row = plan.cells.len();
            plan.cells.push(line.cells);
            plan.gears.extend(line.gears.into_iter().map(|mut gear| {
                gear.row = row;
                gear
            }));
            plan
        },
    );

    let plan: IResult<_, _> = board_parser(input);
    let plan = plan.unwrap().1;

    plan.gears
        .into_iter()
        .filter_map(|gear| {
            let adjacent_nums: BTreeSet<_> = (-1..=1)
                .into_iter()
                .cartesian_product(-1..=1)
                .map(|(drow, dcol)| (gear.row as i32 + drow, gear.col as i32 + dcol))
                .filter_map(|(row, col)| {
                    if row < 0 || col < 0 {
                        None
                    } else {
                        Some((row as usize, col as usize))
                    }
                })
                .filter_map(|(row, col)| plan.cells.get(row).and_then(|line| line.get(col)))
                .filter_map(|cell| match cell {
                    Cell::Number(n) => Some(n),
                    _ => None,
                })
                .copied()
                .collect();

            if adjacent_nums.len() != 2 {
                None
            } else {
                adjacent_nums.into_iter().product1::<u32>()
            }
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(solve(input), "467835");
    }
}
