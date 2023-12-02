#![allow(unused)]

use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::{self, tag};
use nom::character::complete::newline;
use nom::character::streaming::space1;
use nom::combinator::map;
use nom::multi::{fold_many1, many0, separated_list0};
use nom::sequence::{delimited, separated_pair, terminated, pair};
use nom::{branch, character, IResult, Parser};

fn main() {
    let input = include_str!("../../input1.txt");
    println!("{}", solve(input));
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    const fn limit(&self) -> u32 {
        match *self {
            Color::Red => 12,
            Color::Green => 13,
            Color::Blue => 14,
        }
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl Game {
    fn is_valid(&self) -> bool {
        self.draws.iter().all(Draw::is_valid)
    }
}

#[derive(Debug, Default)]
struct Draw {
    colors: HashMap<Color, u32>,
}

impl Draw {
    fn is_valid(&self) -> bool {
        self.colors.iter().all(|(color, count)| *count <= color.limit())
    }
}

fn color(input: &str) -> IResult<&str, Color> {
    alt((
        map(tag("red"), |_| Color::Red),
        map(tag("green"), |_| Color::Green),
        map(tag("blue"), |_| Color::Blue),
    ))(input)
}

fn draw(input: &str) -> IResult<&str, Draw> {
    let color_count = terminated(
        separated_pair(character::complete::u32, space1, color),
        many0(tag(", ")),
    );

    fold_many1(
        color_count,
        Draw::default,
        |mut draw, (count, color)| {
            *draw.colors.entry(color).or_default() += count;
            draw
        },
    )(input)
}

fn game(input: &str) -> IResult<&str, Game> {
    let game_id = delimited(tag("Game "), character::complete::u32, tag(": "));
    let draws = separated_list0(tag("; "), draw);

    map(pair(game_id, draws), |(id, draws)| Game {id, draws})(input)
}

fn solve(input: &str) -> String {
    let games = separated_list0(newline, game)(input).unwrap().1;
    games.into_iter().filter(Game::is_valid).map(|g| g.id).sum::<u32>().to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(solve(input), "8");
    }
}
