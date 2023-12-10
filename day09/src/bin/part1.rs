#![allow(unused)]

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

fn solve(input: &str) -> String {
    let (_, histories) = separated_list1(
        newline,
        separated_list1(space1::<_, nom::error::Error<_>>, i32),
    )(input)
    .unwrap();

    histories
        .into_iter()
        .map(|history| {
            let mut diffs = vec![history];
            while !diffs.last().unwrap().iter().all(|val| *val == 0) {
                let actual = diffs.last().unwrap();
                let mut diff = vec![];
                for i in 0..actual.len() - 1 {
                    diff.push(actual[i + 1] - actual[i]);
                }
                diffs.push(diff);
            }

            diffs
                .into_iter()
                .filter_map(|v| v.last().copied())
                .sum::<i32>()
        })
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(solve(input), "114");
    }
}
