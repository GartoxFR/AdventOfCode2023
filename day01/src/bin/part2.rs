use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::one_of;
use nom::combinator::map;
use nom::IResult;

fn main() {
    let input = include_str!("../../input2.txt");
    println!("{}", solve(input));
}

fn number(input: &str) -> IResult<&str, u32> {
    alt((
        map(one_of("0123456789"), |c| c.to_digit(10).unwrap()),
        map(tag("one"), |_| 1),
        map(tag("two"), |_| 2),
        map(tag("three"), |_| 3),
        map(tag("four"), |_| 4),
        map(tag("five"), |_| 5),
        map(tag("six"), |_| 6),
        map(tag("seven"), |_| 7),
        map(tag("eight"), |_| 8),
        map(tag("nine"), |_| 9),
    ))(input)
}

fn digits(mut input: &str) -> Vec<u32> {
    let mut digits = vec![];
    while !input.is_empty() {
        if let Ok((_, digit)) = number(input) {
            digits.push(digit);
        }
        input = &input[1..];
    }
    digits
}

fn solve(input: &str) -> String {
    println!("{:?}", digits("3jsdxk"));
    input
        .lines()
        .map(|line| {
            let digits = digits(line);
            digits.first().unwrap() * 10 + digits.last().unwrap()
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(solve(input), "281");
    }
}
