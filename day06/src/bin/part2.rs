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
    let (input, time) = map(
        delimited(
            tuple((tag("Time:"), space0)),
            separated_list1(space1::<_, nom::error::Error<_>>, digit1),
            newline,
        ),
        |strs| strs.join("").parse::<i64>().unwrap(),
    )(input)
    .unwrap();
    let (input, record) = map(
        preceded(
            tuple((tag("Distance:"), space0)),
            separated_list1(space1::<_, nom::error::Error<_>>, digit1),
        ),
        |strs| strs.join("").parse::<i64>().unwrap(),
    )(input)
    .unwrap();

    let delta = time.pow(2) - 4 * record;
    if delta <= 0 {
        "0".into()
    } else {
        let h1 = (time as f64 - f64::sqrt(delta as f64)) / 2.0;
        let h2 = (time as f64 + f64::sqrt(delta as f64)) / 2.0;
        let mut first_possibility = h1.ceil() as i64;
        let mut last_possibility = h2.floor() as i64;

        if first_possibility * (time - first_possibility) == record {
            first_possibility += 1;
        }

        if last_possibility * (time - last_possibility) == record {
            last_possibility -= 1;
        }

        (last_possibility - first_possibility + 1).to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(solve(input), "71503");
    }
}
