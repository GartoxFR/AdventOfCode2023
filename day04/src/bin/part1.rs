use nom::bytes::complete::tag;
use nom::character::complete::{self, newline};
use nom::character::complete::{space0, space1};
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::{delimited, terminated, tuple};
use nom::IResult;

fn main() {
    let input = include_str!("../../input1.txt");
    println!("{}", solve(input));
}

#[derive(Debug)]
struct Card {
    _id: u32,
    obtained: Vec<u32>,
    winning: Vec<u32>,
}

fn card(input: &str) -> IResult<&str, Card> {
    map(
        tuple((
            delimited(
                terminated(tag("Card"), space1),
                complete::u32,
                terminated(tag(":"), space1),
            ),
            terminated(
                separated_list0(space1, complete::u32),
                delimited(space0, tag("|"), space0),
            ),
            separated_list0(space1, complete::u32),
        )),
        |(id, obtained, winning)| Card {
            _id: id,
            obtained,
            winning,
        },
    )(input)
}

fn solve(input: &str) -> String {
    let (_, cards) = separated_list0(newline, card)(input).unwrap();

    cards
        .into_iter()
        .map(|card| {
            let winning_count = card
                .obtained
                .into_iter()
                .filter(|num| card.winning.contains(num))
                .count() as u32;
            if winning_count == 0 {
                0
            } else {
                u32::pow(2, winning_count - 1)
            }
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(solve(input), "13");
    }
}
