#![allow(unused)]

use std::collections::HashSet;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn all() -> &'static [Self] {
        &[
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
    }

    fn inverse(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug)]
struct Pipe(Direction, Direction);

#[derive(Debug)]
enum Tile {
    Pipe(Pipe),
    Ground,
    Start,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(usize, usize);

impl Position {
    fn follow_dir(self, dir: &Direction) -> Option<Position> {
        let Position(row, col) = self;
        match dir {
            Direction::North => Some(Position(row.checked_sub(1)?, col)),
            Direction::South => Some(Position(row.checked_add(1)?, col)),
            Direction::West => Some(Position(row, col.checked_sub(1)?)),
            Direction::East => Some(Position(row, col.checked_add(1)?)),
        }
    }
}

#[derive(Debug)]
struct Map(Vec<Vec<Tile>>);

impl Map {
    fn get_tile(&self, &Position(row, col): &Position) -> Option<&Tile> {
        self.0.get(row)?.get(col)
    }

    fn find_start(&self) -> Option<Position> {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(row, cols)| {
                std::iter::repeat(row).zip(cols.iter().enumerate().map(|(col, tile)| (col, tile)))
            })
            .map(|(row, (col, tile))| (Position(row, col), tile))
            .find_map(|(pos, tile)| match tile {
                Tile::Start => Some(pos),
                _ => None,
            })
    }
}

impl Pipe {
    fn from_char(c: char) -> Self {
        match c {
            '|' => Pipe(Direction::North, Direction::South),
            '-' => Pipe(Direction::East, Direction::West),
            'L' => Pipe(Direction::North, Direction::East),
            'J' => Pipe(Direction::North, Direction::West),
            '7' => Pipe(Direction::South, Direction::West),
            'F' => Pipe(Direction::South, Direction::East),
            _ => unreachable!(),
        }
    }

    fn traverse(&self, entrance: Direction) -> Option<Direction> {
        match (self.0, self.1) {
            (first, second) if first == entrance => Some(second),
            (first, second) if second == entrance => Some(first),
            _ => None,
        }
    }

    fn contains(&self, dir: &Direction) -> bool {
        self.0 == *dir || self.1 == *dir
    }
}

fn tile(input: &str) -> IResult<&str, Tile> {
    alt((
        map(one_of("|-LJ7F"), |c| Tile::Pipe(Pipe::from_char(c))),
        map(tag("."), |_| Tile::Ground),
        map(tag("S"), |_| Tile::Start),
    ))(input)
}
fn solve(input: &str) -> String {
    let (_, mut map) = map(separated_list1(newline, many1(tile)), Map)(input).unwrap();
    let start = map.find_start().unwrap();
    let (path, start_dir1, start_dir2) = Direction::all()
        .iter()
        .find_map(|start_dir| {
            let mut path = HashSet::new();
            path.insert(start);
            let mut dir = *start_dir;
            let mut current_pos = start.follow_dir(start_dir)?;
            let mut current_tile = map.get_tile(&current_pos)?;
            while let Tile::Pipe(pipe) = current_tile {
                path.insert(current_pos);
                dir = pipe.traverse(dir.inverse())?;
                current_pos = current_pos.follow_dir(&dir)?;
                current_tile = map.get_tile(&current_pos)?;
            }

            if let Tile::Start = current_tile {
                Some((path, *start_dir, dir.inverse()))
            } else {
                None
            }
        })
        .unwrap();

    map.0[start.0][start.1] = Tile::Pipe(Pipe(start_dir1, start_dir2));

    (0..map.0.len())
        .cartesian_product(0..map.0[0].len())
        .map(|(row, col)| Position(row, col))
        .filter(|pos| !path.contains(pos))
        .filter(|Position(row, col)| {
            let v_crossings = (0..*row)
                .zip(std::iter::repeat(*col))
                .filter(|(row, col)| path.contains(&Position(*row, *col)))
                .fold((None, 0), |(last_dir, count), (row, col)| {
                    let Tile::Pipe(pipe) = map.get_tile(&Position(row, col)).unwrap() else {
                        unreachable!()
                    };
                    if !pipe.contains(&Direction::North) && !pipe.contains(&Direction::South) {
                        (None, count + 1)
                    } else if pipe.contains(&Direction::South) && last_dir.is_none() {
                        (pipe.traverse(Direction::South), count)
                    } else if pipe.contains(&Direction::North) && !pipe.contains(&Direction::South) {
                        let dir = pipe.traverse(Direction::North);
                        if dir != last_dir {
                            (None, count + 1)
                        } else {
                            (None, count)
                        }
                    } else {
                        (last_dir, count)
                    }
                }).1;

            let h_crossings = std::iter::repeat(*row).zip(0..*col)
                .filter(|(row, col)| path.contains(&Position(*row, *col)))
                .fold((None, 0), |(last_dir, count), (row, col)| {
                    let Tile::Pipe(pipe) = map.get_tile(&Position(row, col)).unwrap() else {
                        unreachable!()
                    };
                    if !pipe.contains(&Direction::West) && !pipe.contains(&Direction::East) {
                        (None, count + 1)
                    } else if pipe.contains(&Direction::East) && last_dir.is_none() {
                        (pipe.traverse(Direction::East), count)
                    } else if pipe.contains(&Direction::West) && !pipe.contains(&Direction::East) {
                        let dir = pipe.traverse(Direction::West);
                        if dir != last_dir {
                            (None, count + 1)
                        } else {
                            (None, count)
                        }
                    } else {
                        (last_dir, count)
                    }
                }).1;

            v_crossings % 2 == 1 && h_crossings % 2 == 1
        }).inspect(|el| println!("{:?}", el)).count().to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(solve(input), "10");
    }
}
