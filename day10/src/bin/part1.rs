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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn all() -> &'static [Self] {
        &[Direction::North, Direction::South, Direction::East, Direction::West]
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

#[derive(Debug, Clone, Copy)]
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
}

fn tile(input: &str) -> IResult<&str, Tile> {
    alt((
        map(one_of("|-LJ7F"), |c| Tile::Pipe(Pipe::from_char(c))),
        map(tag("."), |_| Tile::Ground),
        map(tag("S"), |_| Tile::Start),
    ))(input)
}
fn solve(input: &str) -> String {
    let (_, map) = map(separated_list1(newline, many1(tile)), Map)(input).unwrap();
    let start = map.find_start().unwrap();
    let len = Direction::all().iter().find_map(|start_dir| {
        let mut dir = *start_dir;
        let mut current_pos = start.follow_dir(start_dir)?;
        let mut current_tile = map.get_tile(&current_pos)?;
        let mut len = 0;
        while let Tile::Pipe(pipe) = current_tile {
            len += 1;
            dir = pipe.traverse(dir.inverse())?;
            current_pos = current_pos.follow_dir(&dir)?;
            current_tile = map.get_tile(&current_pos)?;
        }

        if let Tile::Start = current_tile {
            let mut len = len / 2;
            if (len % 2 != 0) {
                len += 1;
            }
            Some(len)
        } else {
            None
        }
    }).unwrap();

    len.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(solve(input), "8");
    }
}
