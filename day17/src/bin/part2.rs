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

    fn reverse(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
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
    prev: Option<Previous>,
}

#[derive(Debug, Clone, Copy)]
struct Previous {
    row: isize,
    col: isize,
    dist: usize,
    dir: Direction,
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
                .map(|_| vec![vec![usize::MAX; 10]; 4])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut prevs = map
        .iter()
        .map(|line| {
            line.iter()
                .map(|_| vec![vec![None; 10]; 4])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut queue = BinaryHeap::default();
    queue.push(State {
        row: 0,
        col: 1,
        dist: 0,
        dir: Direction::Right,
        lost_heat: map[0][1],
        prev: None,
    });

    queue.push(State {
        row: 1,
        col: 0,
        dist: 0,
        dir: Direction::Down,
        lost_heat: map[1][0],
        prev: None,
    });

    while let Some(state) = queue.pop() {
        let best =
            &mut explored[state.row as usize][state.col as usize][state.dir as usize][state.dist];

        if (state.lost_heat >= *best) {
            continue;
        }

        *best = state.lost_heat;
        prevs[state.row as usize][state.col as usize][state.dir as usize][state.dist] = state.prev;

        if state.dist < 9 {
            let (row, col) = state.dir.advance(state.row, state.col);
            if row >= 0 && row < map.len() as isize && col >= 0 && col < map[0].len() as isize {
                let cost = map[row as usize][col as usize];
                queue.push(State {
                    row,
                    col,
                    dir: state.dir,
                    dist: state.dist + 1,
                    lost_heat: state.lost_heat + cost,
                    prev: Some(Previous {
                        row: state.row,
                        col: state.col,
                        dist: state.dist,
                        dir: state.dir,
                    }),
                });
            }
        }

        if state.dist > 2 {
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
                        prev: Some(Previous {
                            row: state.row,
                            col: state.col,
                            dist: state.dist,
                            dir: state.dir,
                        }),
                    });
                }
            }
        }
    }

    let (dir, (dist, _)) = explored
        .last()
        .unwrap()
        .last()
        .unwrap()
        .iter()
        .enumerate()
        .flat_map(|(dir, dists)| std::iter::repeat(dir).zip(dists.iter().enumerate().skip(3)))
        .min_by_key(|(_, (_, dist))| **dist)
        .unwrap();

    let mut prev_opt = prevs.last().unwrap().last().unwrap()[dir][dist];
    let mut map = map.iter().map(|line| line.iter().map(|_| '.').collect::<Vec<_>>()).collect::<Vec<_>>();
    while let Some(prev) = prev_opt {
        // println!("{} {} {}", prev.row, prev.col, prev.dist);
        map[prev.row as usize][prev.col as usize] = match prev.dir {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        };
        prev_opt = prevs[prev.row as usize][prev.col as usize][prev.dir as usize][prev.dist];
    }

    for line in map {
        for c in line {
            print!("{c}");
        }
        println!()
    }

    explored
        .last()
        .unwrap()
        .last()
        .unwrap()
        .iter()
        .flat_map(|dists| dists.iter().skip(3))
        .min()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
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
        assert_eq!(solve(input), "94");
    }
    #[test]
    fn test_part2_2() {
        let input = "111111111111
999999999991
999999999991
999999999991
999999999991";
        assert_eq!(solve(input), "71");
    }
    #[test]
    fn test_part2_3() {
        let input = "111111111119999
999999999919999
999999999919999
999999999919999
999999999911111";
        assert_eq!(solve(input), "50");
    }
}
