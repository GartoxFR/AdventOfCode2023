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
type Groups = Vec<usize>;

fn parse_input(input: &str) -> IResult<&str, Vec<(Parts, Groups)>> {
    separated_list0(
        newline,
        separated_pair(
            many1(map(one_of(".#?"), PartState::from_char)),
            space1,
            separated_list1(tag(","), map(u64, |n| n as usize)),
        ),
    )(input)
}

fn count_possibilities(parts: &[PartState], groups: &[usize]) -> u64 {
    let mut dp: Vec<Vec<_>> = (0..=groups.len())
        .map(|_| (0..=parts.len()).map(|_| 0).collect())
        .collect();

    for l in &mut dp {
        l[0] = 0; // No parts = not solution
    }

    for p in 1..dp[0].len() {
        dp[0][p] = if parts[..p].iter().any(|p| matches!(p, PartState::Broken)) {
            0
        } else {
            1
        }
    }

    dp[0][0] = 1;

    for g in 1..dp.len() {
        for p in 1..dp[0].len() {
            let mut count = 0;
            let group_len = groups[g - 1];
            let parts = &parts[..p];

            if group_len <= p
                && !parts[parts.len() - group_len..]
                    .iter()
                    .any(|p| matches!(p, PartState::Working))
            {
                if p == group_len {
                    count += dp[g - 1][0]
                } else {
                    match parts[p - group_len - 1] {
                        PartState::Broken => {}
                        _ => {
                            count += dp[g - 1][p - group_len - 1]
                        }
                    }
                }
            }

            if !matches!(parts.last().unwrap(), PartState::Broken) {
                count += dp[g][p - 1];
            }

            dp[g][p] = count;
        }
    }
    // println!("----------");
    // for line in &dp {
    //     for val in line {
    //         print!("{:2} ", val);
    //     }
    //     println!()
    // }
    dp[groups.len()][parts.len()]
}

fn solve(input: &str) -> String {
    let (_, lines) = parse_input(input).unwrap();

    let mut progress = 0;
    let len = lines.len();
    lines
        .into_iter()
        .map(|(mut parts, groups)| {
            let mut new_parts = parts.clone();
            for i in 0..4 {
                new_parts.push(PartState::Unknow);
                new_parts.extend(parts.iter().copied());
            }

            let new_groups = groups
                .iter()
                .copied()
                .cycle()
                .take(groups.len() * 5)
                .collect::<Vec<_>>();
            (new_parts, new_groups)
        })
        .map(|(parts, groups)| count_possibilities(&parts[..], &groups[..]))
        .inspect(|_| {
            progress += 1;
            println!("{}/{}", progress, len);
        })
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(solve(input), "525152");
    }
}
