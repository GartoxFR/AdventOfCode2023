#![allow(unused)]

use nom::branch::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

#[derive(Debug, Clone, Copy)]
enum SplitterDirection {
    Horizontal,
    Vertical,
}

impl SplitterDirection {
    fn split(&self, dir: &Direction) -> (Direction, Option<Direction>) {
        match (self, dir) {
            (Self::Horizontal, Direction::Up | Direction::Down) => {
                (Direction::Left, Some(Direction::Right))
            }
            (Self::Vertical, Direction::Left | Direction::Right) => {
                (Direction::Up, Some(Direction::Down))
            }
            _ => (*dir, None),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum MirrorDirection {
    UpLeft,
    UpRight,
}

impl MirrorDirection {
    fn bounce(&self, dir: &Direction) -> Direction {
        match (self, dir) {
            (MirrorDirection::UpLeft, Direction::Up) => Direction::Left,
            (MirrorDirection::UpLeft, Direction::Down) => Direction::Right,
            (MirrorDirection::UpLeft, Direction::Left) => Direction::Up,
            (MirrorDirection::UpLeft, Direction::Right) => Direction::Down,
            (MirrorDirection::UpRight, Direction::Up) => Direction::Right,
            (MirrorDirection::UpRight, Direction::Down) => Direction::Left,
            (MirrorDirection::UpRight, Direction::Left) => Direction::Down,
            (MirrorDirection::UpRight, Direction::Right) => Direction::Up,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn advance(&self, row: isize, col: isize) -> (isize, isize) {
        match self {
            Direction::Up => (row - 1, col),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    Mirror(MirrorDirection),
    Splitter(SplitterDirection),
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '-' => Tile::Splitter(SplitterDirection::Horizontal),
            '|' => Tile::Splitter(SplitterDirection::Vertical),
            '/' => Tile::Mirror(MirrorDirection::UpRight),
            '\\' => Tile::Mirror(MirrorDirection::UpLeft),
            _ => panic!("Unknown tile"),
        }
    }
}

fn main() {
    let input = include_str!("../../input1.txt");
    println!("{}", solve(input));
}

#[derive(Debug, Clone, Copy)]
struct Beam {
    row: isize,
    col: isize,
    dir: Direction,
}

fn explore(map: &Vec<Vec<Tile>>, start_beam: Beam) -> usize {
    let mut explored: Vec<Vec<Vec<Direction>>> = map
        .iter()
        .map(|line| line.iter().map(|_| vec![]).collect())
        .collect();

    let mut beams = vec![start_beam];

    while let Some(beam) = beams.pop() {
        if beam.row < 0
            || beam.row >= map.len() as isize
            || beam.col < 0
            || beam.col >= map[0].len() as isize
        {
            continue;
        }

        let explored = &mut explored[beam.row as usize][beam.col as usize];
        if (explored.contains(&beam.dir)) {
            continue;
        }

        explored.push(beam.dir);

        let tile = map[beam.row as usize][beam.col as usize];
        let (dir1, dir2) = match tile {
            Tile::Empty => (beam.dir, None),
            Tile::Mirror(mirror) => (mirror.bounce(&beam.dir), None),
            Tile::Splitter(splitter) => splitter.split(&beam.dir),
        };

        let (row, col) = dir1.advance(beam.row, beam.col);
        beams.push(Beam {
            row,
            col,
            dir: dir1,
        });

        if let Some(dir) = dir2 {
            let (row, col) = dir.advance(beam.row, beam.col);
            beams.push(Beam { row, col, dir });
        }
    }

    explored.iter().flatten().filter(|v| !v.is_empty()).count()
}

fn solve(input: &str) -> String {
    let map: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| line.chars().map(Tile::from).collect())
        .collect();

    (0..map.len() as isize)
        .flat_map(|row| {
            [
                Beam {
                    row,
                    col: 0,
                    dir: Direction::Right,
                },
                Beam {
                    row,
                    col: map[0].len() as isize - 1,
                    dir: Direction::Left,
                },
            ]
        })
        .chain((0..map[0].len() as isize).flat_map(|col| {
            [
                Beam {
                    row: 0,
                    col,
                    dir: Direction::Down,
                },
                Beam {
                    row: map.len() as isize - 1,
                    col,
                    dir: Direction::Up,
                },
            ]
        }))
        .map(|beam| explore(&map, beam))
        .max()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
        let input = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";
        assert_eq!(solve(input), "51");
    }
}
