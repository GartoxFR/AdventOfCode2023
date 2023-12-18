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

#[derive(Debug)]
enum Intruction<'a> {
    Add(&'a str, u8),
    Remove(&'a str),
}

#[derive(Debug)]
struct Step<'a> {
    hash: u8,
    instruction: Intruction<'a>,
}

fn hash(s: &str) -> u32 {
    s.chars()
        .fold(0, |hash, current| ((hash + current as u32) * 17) % 256)
}

fn parse_line(line: &str) -> Result<Vec<Step>, ()> {
    line.split(',')
        .map(|line| {
            let instruction = alt((
                map(
                    separated_pair(alpha1::<&str, nom::error::Error<_>>, tag("="), u8),
                    |(label, focal)| Intruction::Add(label, focal),
                ),
                map(terminated(alpha1, tag("-")), Intruction::Remove),
            ))(line)
            .map_err(|_| ())?
            .1;
            let label = match instruction {
                Intruction::Add(label, _) | Intruction::Remove(label) => label,
            };
            Ok(Step {
                hash: hash(label) as u8,
                instruction,
            })
        })
        .collect()
}

type Box<'a> = Vec<(&'a str, u8)>;
fn solve(input: &str) -> String {
    let steps = parse_line(input.lines().next().unwrap()).unwrap();
    let mut boxes = vec![Box::default(); 256];

    for step in steps {
        let b = &mut boxes[step.hash as usize];
        match step.instruction {
            Intruction::Add(label, focal) => {
                if let Some(i) = b
                    .iter()
                    .enumerate()
                    .find_map(|(i, (l, _))| (*l == label).then_some(i))
                {
                    b[i].1 = focal;
                } else {
                    b.push((label, focal));
                }
            }
            Intruction::Remove(label) => {
                b.retain(|(l, _)| *l != label);
            }
        }
    }

    boxes
        .into_iter()
        .enumerate()
        .map(|(bi, b)| {
            (bi + 1)
                * b.into_iter()
                    .enumerate()
                    .map(|(li, (_, focal))| (li + 1) * focal as usize)
                    .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(solve(input), "145");
    }
}
