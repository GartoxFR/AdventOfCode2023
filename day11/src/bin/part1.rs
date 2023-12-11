#![allow(unused)]

use std::collections::BTreeSet;
use std::collections::HashSet;
use std::iter::repeat;

use itertools::Itertools;
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

fn solve(input: &str) -> String {
    let galaxies: Vec<_> = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            repeat(row)
                .zip(line.chars().enumerate())
                .filter_map(|(row, (col, c))| if c == '#' { Some((row, col)) } else { None })
        })
        .collect();

    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let used_rows: HashSet<_> = galaxies.iter().map(|(row, _)| row).copied().collect();
    let used_cols: HashSet<_> = galaxies.iter().map(|(_, col)| col).copied().collect();

    let expanded_rows: Vec<_> = (0..height).filter(|row| !used_rows.contains(row)).collect();
    let expanded_cols: Vec<_> = (0..width).filter(|col| !used_cols.contains(col)).collect();

    galaxies
        .into_iter()
        .tuple_combinations()
        .map(|((row1, col1), (row2, col2))| {
            let expanded_rows_between = usize::abs_diff(
                expanded_rows.binary_search(&row1).unwrap_or_else(|err| err),
                expanded_rows.binary_search(&row2).unwrap_or_else(|err| err),
            );
            let expanded_cols_between = usize::abs_diff(
                expanded_cols.binary_search(&col1).unwrap_or_else(|err| err),
                expanded_cols.binary_search(&col2).unwrap_or_else(|err| err),
            );

            usize::abs_diff(row1, row2)
                + usize::abs_diff(col1, col2)
                + expanded_rows_between
                + expanded_cols_between
        })
        .sum::<usize>()
        .to_string()
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(solve(input), "374");
    }
}
