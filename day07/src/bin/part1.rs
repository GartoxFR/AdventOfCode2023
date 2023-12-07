#![allow(unused)]

use std::cmp::Ordering;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(c: char) -> Self {
        match c {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!("Wrong card char: {}", c),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    typ: HandType,
    cards: [Card; 5],
}

fn hands(input: &str) -> IResult<&str, Vec<(Hand, u32)>> {
    separated_list1(
        newline,
        map(
            separated_pair(
                count(map(one_of("23456789TJQKA"), Card::from_char), 5),
                space0,
                u32,
            ),
            |(cards, value)| {
                let cards = cards.try_into().unwrap();
                (
                    Hand {
                        typ: hand_type(&cards),
                        cards,
                    },
                    value,
                )
            },
        ),
    )(input)
}

fn streak_end(streak: u32, typ: HandType) -> HandType {
    match streak {
        1 => typ,
        2 => match typ {
            HandType::HighCard => HandType::OnePair,
            HandType::OnePair => HandType::TwoPair,
            HandType::ThreeOfKind => HandType::FullHouse,
            _ => unreachable!(),
        },
        3 => match typ {
            HandType::HighCard => HandType::ThreeOfKind,
            HandType::OnePair => HandType::FullHouse,
            _ => unreachable!(),
        },
        4 => HandType::FourOfKind,
        5 => HandType::FiveOfKind,
        _ => unreachable!(),
    }
}

fn hand_type(cards: &[Card; 5]) -> HandType {
    let mut cards = *cards;
    cards.sort_unstable();

    let mut last_card = cards[0];
    let mut streak = 1;
    let mut typ = HandType::HighCard;

    for card in cards.into_iter().skip(1) {
        if (card == last_card) {
            streak += 1;
        } else {
            typ = streak_end(streak, typ);
            streak = 1;
            last_card = card;
        }
    }

    streak_end(streak, typ)
}

fn solve(input: &str) -> String {
    let (_, mut hands) = hands(input).unwrap();
    hands.sort_by(|a, b| a.0.cmp(&b.0));
    hands
        .into_iter()
        .map(|(_hand, bid)| bid)
        .enumerate()
        .map(|(i, bid)| (i as u32 + 1) * bid)
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(solve(input), "6440");
    }
}
