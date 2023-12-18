#![allow(unused)]

use nom::branch::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    None,
    Round,
    Square,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::None,
            'O' => Self::Round,
            '#' => Self::Square,
            _ => panic!("Invalid terrain"),
        }
    }
}

fn main() {
    let input = include_str!("../../input1.txt");
    println!("{}", solve(input));
}
fn transpose(pattern: Terrain) -> Terrain {
    (0..pattern[0].len())
        .map(|col| (0..pattern.len()).map(|row| pattern[row][col]).collect())
        .collect()
}

type Terrain = Vec<Vec<Cell>>;
fn parse_input(input: &str) -> IResult<&str, Terrain> {
    map(
        separated_list1(newline, many1(map(one_of(".O#"), Cell::from))),
        transpose,
    )(input)
}

fn tilt_column(cells: &mut [Cell]) {
    let mut free_len = 0;
    for i in 0..cells.len() {
        match cells[i] {
            Cell::None => {
                free_len += 1;
            }
            Cell::Round => {
                if free_len > 0 {
                    cells.swap(i, i - free_len);
                }
            }
            Cell::Square => {
                free_len = 0;
            }
        }
    }
}

fn compute_col_load(cells: &[Cell]) -> usize {
    cells
        .iter()
        .enumerate()
        .filter_map(|(row, cell)| match cell {
            Cell::Round => Some(cells.len() - row),
            _ => None,
        })
        .sum()
}

fn solve(input: &str) -> String {
    let (_, mut terrain) = parse_input(input).unwrap();
    terrain
        .iter_mut()
        .map(Vec::as_mut_slice)
        .for_each(tilt_column);

    terrain.iter().map(Vec::as_slice)
        .map(compute_col_load)
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";
        assert_eq!(solve(input), "136");
    }
}
