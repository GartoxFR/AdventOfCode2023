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
    let (input, times) = delimited(
        tuple((tag("Time:"), space0)),
        separated_list1(space1::<_, nom::error::Error<_>>, i32),
        newline,
    )(input)
    .unwrap();
    let (input, records) = preceded(
        tuple((tag("Distance:"), space0)),
        separated_list1(space1::<_, nom::error::Error<_>>, i32),
    )(input)
    .unwrap();

    times
        .into_iter()
        .zip(records)
        .map(|(time, record)| {
            let delta = time.pow(2) - 4 * record;
            if delta <= 0 {
                0
            } else {
                let h1 = (time as f32 - f32::sqrt(delta as f32)) / 2.0;
                let h2 = (time as f32 + f32::sqrt(delta as f32)) / 2.0;
                let mut first_possibility = h1.ceil() as i32;
                let mut last_possibility = h2.floor() as i32;

                if first_possibility * (time - first_possibility) == record {
                    first_possibility += 1;
                }

                if last_possibility * (time - last_possibility) == record {
                    last_possibility -= 1;
                }

                last_possibility - first_possibility + 1
            }
        })
        .product::<i32>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(solve(input), "288");
    }
}
