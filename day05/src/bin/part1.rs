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
struct Map(Vec<(u64, u64, u64)>);

impl Map {
    fn lookup(&self, val: u64) -> u64 {
        self.0
            .iter()
            .find_map(|(dest_start, source_start, range_len)| {
                if (*source_start..source_start + range_len).contains(&val) {
                    Some(dest_start + (val - source_start))
                } else {
                    None
                }
            })
            .unwrap_or(val)
    }
}

fn map_parser(input: &str) -> IResult<&str, Map> {
    map(
        preceded(
            tuple((is_not("\r\n"), newline)),
            separated_list0(
                newline,
                tuple((
                    preceded(space0, u64),
                    preceded(space0, u64),
                    preceded(space0, u64),
                )),
            ),
        ),
        Map,
    )(input)
}

fn solve(input: &str) -> String {
    let (input, seeds) = delimited(
        tuple((tag::<_, _, nom::error::Error<_>>("seeds:"), space0)),
        separated_list1(space1, u64),
        count(newline, 2),
    )(input)
    .unwrap();

    let (_, maps) = separated_list1(count(newline, 2), map_parser)(input).unwrap();

    seeds
        .into_iter()
        .map(|seed| maps.iter().fold(seed, |val, map| map.lookup(val)))
        .min()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(solve(input), "35");
    }
}
