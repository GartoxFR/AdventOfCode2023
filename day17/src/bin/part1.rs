#![allow(unused)]

use std::collections::BinaryHeap;
use std::collections::VecDeque;

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

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl Direction {
    fn turns(&self) -> [Direction; 2] {
        match self {
            Direction::Up => [Direction::Right, Direction::Left],
            Direction::Down => [Direction::Right, Direction::Left],
            Direction::Left => [Direction::Down, Direction::Up],
            Direction::Right => [Direction::Down, Direction::Up],
        }
    }
    fn advance(&self, row: isize, col: isize) -> (isize, isize) {
        match self {
            Direction::Up => (row - 1, col),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1),
        }
    }
}

#[derive(Debug)]
struct State {
    row: isize,
    col: isize,
    dir: Direction,
    dist: usize,
    lost_heat: usize,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.lost_heat == other.lost_heat
    }
}

impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.lost_heat.cmp(&self.lost_heat)
    }
}
fn solve(input: &str) -> String {
    let map: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let mut explored = map
        .iter()
        .map(|line| {
            line.iter()
                .map(|_| vec![vec![usize::MAX; 4]; 4])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut queue = BinaryHeap::default();
    queue.push(State {
        row: 0,
        col: 0,
        dist: 0,
        dir: Direction::Right,
        lost_heat: 0,
    });

    while let Some(state) = queue.pop() {
        let best =
            &mut explored[state.row as usize][state.col as usize][state.dir as usize][state.dist];
        if (state.lost_heat >= *best) {
            continue;
        }

        *best = state.lost_heat;

        if state.dist < 2 {
            let (row, col) = state.dir.advance(state.row, state.col);
            if row >= 0 && row < map.len() as isize && col >= 0 && col < map[0].len() as isize {
                let cost = map[row as usize][col as usize];
                queue.push(State {
                    row,
                    col,
                    dir: state.dir,
                    dist: state.dist + 1,
                    lost_heat: state.lost_heat + cost,
                });
            }
        }

        for dir in state.dir.turns() {
            let (row, col) = dir.advance(state.row, state.col);
            if row >= 0 && row < map.len() as isize && col >= 0 && col < map[0].len() as isize {
                let cost = map[row as usize][col as usize];
                queue.push(State {
                    row,
                    col,
                    dir,
                    dist: 0,
                    lost_heat: state.lost_heat + cost,
                });
            }
        }
    }

    explored
        .last()
        .unwrap()
        .last()
        .unwrap()
        .iter()
        .flatten()
        .min()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        assert_eq!(solve(input), "102");
    }
}
