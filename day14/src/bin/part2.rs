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

type Terrain = Vec<Vec<Cell>>;
fn parse_input(input: &str) -> IResult<&str, Terrain> {
    separated_list1(newline, many1(map(one_of(".O#"), Cell::from)))(input)
}

fn tilt_west(rows: &mut Terrain) {
    for row in rows {
        let mut free_len = 0;
        for i in 0..row.len() {
            match row[i] {
                Cell::None => {
                    free_len += 1;
                }
                Cell::Round => {
                    if free_len > 0 {
                        row.swap(i, i - free_len);
                    }
                }
                Cell::Square => {
                    free_len = 0;
                }
            }
        }
    }
}

fn tilt_east(rows: &mut Terrain) {
    for row in rows {
        let mut free_len = 0;
        for i in (0..row.len()).rev() {
            match row[i] {
                Cell::None => {
                    free_len += 1;
                }
                Cell::Round => {
                    if free_len > 0 {
                        row.swap(i, i + free_len);
                    }
                }
                Cell::Square => {
                    free_len = 0;
                }
            }
        }
    }
}

fn tilt_north(rows: &mut Terrain) {
    for col_num in 0..rows[0].len() {
        let mut free_len = 0;
        for row_num in 0..rows.len() {
            match rows[row_num][col_num] {
                Cell::None => {
                    free_len += 1;
                }
                Cell::Round => {
                    if free_len > 0 {
                        let (before, after) = rows.split_at_mut(row_num);
                        std::mem::swap(
                            &mut before[row_num - free_len][col_num],
                            &mut after[0][col_num],
                        );
                    }
                }
                Cell::Square => {
                    free_len = 0;
                }
            }
        }
    }
}

fn tilt_south(rows: &mut Terrain) {
    for col_num in 0..rows[0].len() {
        let mut free_len = 0;
        for row_num in (0..rows.len()).rev() {
            match rows[row_num][col_num] {
                Cell::None => {
                    free_len += 1;
                }
                Cell::Round => {
                    if free_len > 0 {
                        let (before, after) = rows.split_at_mut(row_num + free_len);
                        std::mem::swap(&mut before[row_num][col_num], &mut after[0][col_num]);
                    }
                }
                Cell::Square => {
                    free_len = 0;
                }
            }
        }
    }
}

fn cycle(rows: &mut Terrain) {
    tilt_north(rows);
    tilt_west(rows);
    tilt_south(rows);
    tilt_east(rows);
}

fn compute_load(rows: &Terrain) -> usize {
    let mut count = 0;
    for (i, row) in rows.into_iter().enumerate() {
        for cell in row {
            if let Cell::Round = cell {
                count += rows.len() - i;
            }
        }
    }
    count
}

fn solve(input: &str) -> String {
    let (_, mut terrain) = parse_input(input).unwrap();
    let cycle_to_do = 1000000000;
    let mut prev_terrains = vec![terrain.clone()];
    let mut cycle_info = None;
    for i in 1..=1000000000 {
        cycle(&mut terrain);
        if let Some(index) = prev_terrains
            .iter()
            .enumerate()
            .find_map(|(i, prev_terrain)| (terrain == *prev_terrain).then_some(i))
        {
            dbg!((i, index));
            cycle_info = Some((index, i - index));
            break;
        }
        prev_terrains.push(terrain.clone());
    }

    if let Some((cycle_start, cycle_len)) = cycle_info {
        let remaining_cycles = cycle_to_do - cycle_start;
        let position_in_cycle = remaining_cycles % cycle_len;
        terrain = prev_terrains[cycle_start + position_in_cycle].clone();
    }
    compute_load(&terrain).to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
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
        assert_eq!(solve(input), "64");
    }
}
