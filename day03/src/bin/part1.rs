#![allow(unused)]

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
    let input = include_str!("../../input1.txt");
    println!("{}", solve(input));
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Dot,
    Digit,
    Symbol,
}

#[derive(Debug)]
struct Number {
    row: usize,
    col: usize,
    value: u32,
    len: usize,
}

#[derive(Debug, Default)]
struct Plan {
    numbers: Vec<Number>,
    cells: Vec<Vec<Cell>>,
}

#[derive(Debug, Default)]
struct LineAccumulator {
    numbers: Vec<Number>,
    cells: Vec<Cell>,
}

#[derive(Debug)]
enum ParsedCells {
    Number(u32, usize), // value, len
    Dot,
    Symbol,
}

fn solve(input: &str) -> String {
    let mut number_parser = map(digit1, |digits: &str| {
        ParsedCells::Number(digits.parse().unwrap(), digits.len())
    });
    let mut dot_parser = map(tag("."), |_| ParsedCells::Dot);
    let mut symbol_parser = map(one_of("[]$%^&*'{()}=+-<>\"~`|_/@\\@#!?"), |_| ParsedCells::Symbol);

    let mut line_parser = fold_many1(
        alt((number_parser, dot_parser, symbol_parser)),
        LineAccumulator::default,
        |mut acc, val| {
            match val {
                ParsedCells::Number(val, len) => {
                    acc.numbers.push(Number {
                        row: 0,
                        col: acc.cells.len(),
                        value: val,
                        len,
                    });
                    acc.cells.extend(std::iter::repeat(Cell::Digit).take(len));
                }
                ParsedCells::Dot => {
                    acc.cells.push(Cell::Dot);
                }
                ParsedCells::Symbol => {
                    acc.cells.push(Cell::Symbol);
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
            plan.numbers.extend(line.numbers.into_iter().map(|mut num| {
                num.row = row;
                num
            }));
            plan
        },
    );

    let plan: IResult<_, _> = board_parser(input);
    let plan = plan.unwrap().1;

    plan.numbers
        .into_iter()
        .filter(|num| {
            ([-1, 1])
                .into_iter()
                .cartesian_product(-1..=num.len as i32)
                .chain([(0, -1), (0, num.len as i32)])
                .map(|(drow, dcol)| (num.row as i32 + drow, num.col as i32 + dcol))
                .filter_map(|(row, col)| {
                    if row < 0 || col < 0 {
                        None
                    } else {
                        Some((row as usize, col as usize))
                    }
                })
                .filter_map(|(row, col)| plan.cells.get(row).and_then(|line| line.get(col)))
                .any(|cell| matches!(cell, Cell::Symbol))
        })
        .map(|num| num.value as u64)
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
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
        assert_eq!(solve(input), "4361");
    }
}
