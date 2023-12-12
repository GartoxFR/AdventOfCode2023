#![allow(unused)]

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
enum PartState {
    Broken,
    Working,
    Unknow,
}

impl PartState {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Working,
            '#' => Self::Broken,
            '?' => Self::Unknow,
            _ => panic!("Wrong part char"),
        }
    }
}

type Parts = Vec<PartState>;
type Groups = Vec<u32>;

fn parse_input(input: &str) -> IResult<&str, Vec<(Parts, Groups)>> {
    separated_list0(
        newline,
        separated_pair(
            many1(map(one_of(".#?"), PartState::from_char)),
            space1,
            separated_list1(tag(","), u32),
        ),
    )(input)
}

fn count_possibilities(parts: &[PartState], groups: &[u32]) -> u64 {
    match groups.first() {
        None => {
            if parts.iter().any(|c| matches!(c, PartState::Broken)) {
                0
            } else {
                1
            }
        }
        Some(len) if *len as usize <= parts.len() => {
            (0..=(parts.len() - *len as usize))
                .filter(|start_i| {
                    // dbg!((parts, groups));
                    if (parts[..*start_i]
                        .iter()
                        .any(|part| matches!(part, PartState::Broken)))
                    {
                        return false;
                    }
                    if let Some(next) = parts.get(*start_i + *len as usize) {
                        if matches!(next, PartState::Broken) {
                            return false;
                        }
                    }
                    !parts[*start_i..*start_i + *len as usize]
                        .iter()
                        .any(|part| matches!(part, PartState::Working))
                })
                .map(|start_i| {
                    let new_start = start_i + *len as usize + 1;
                    if new_start >= parts.len() {
                        count_possibilities(&[], &groups[1..])
                    } else {
                        count_possibilities(&parts[new_start..], &groups[1..])
                    }
                })
                .sum()
        }
        _ => 0,
    }
}

fn solve(input: &str) -> String {
    let (_, lines) = parse_input(input).unwrap();

    lines
        .iter()
        .map(|(parts, groups)| count_possibilities(parts, groups))
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(solve(input), "21");
    }
}
