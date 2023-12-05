use std::ops::Range;

use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

fn main() {
    let input = include_str!("../../input2.txt");
    println!("{}", solve(input));
}

#[derive(Debug)]
struct Map(Vec<(u64, u64, u64)>);

impl Map {
    fn lookup(&self, val_range: Range<u64>) -> Vec<Range<u64>> {
        let (remaining_range, mut destination_ranges) = self.0.iter().fold(
            (vec![val_range], vec![]),
            |(remaining_ranges, mut destination_ranges), (dest_start, source_start, range_len)| {
                let mut new_remaining_ranges = vec![];
                let source_end = source_start + range_len;
                let dest_end = dest_start + range_len;
                for remaining_range in remaining_ranges {
                    if remaining_range.start >= *source_start && remaining_range.end <= source_end {
                        // Remaining fully contained in this range
                        let offset = remaining_range.start - source_start;
                        destination_ranges
                            .push((dest_start + offset)..(dest_start + offset + remaining_range.end - remaining_range.start));
                    } else if remaining_range.start < *source_start
                        && remaining_range.end > source_end
                    {
                        // This range is fully contained in the remaining_range
                        destination_ranges.push(*dest_start..dest_end);


                        new_remaining_ranges.push(remaining_range.start..*source_start);
                        new_remaining_ranges.push(source_end..remaining_range.end);
                    } else if remaining_range.end > *source_start
                        && remaining_range.end <= source_end
                    {
                        // Remaining range right side overlap with this range
                        new_remaining_ranges.push(remaining_range.start..*source_start);
                        destination_ranges
                            .push(*dest_start..dest_start + remaining_range.end - source_start);
                    } else if remaining_range.start < source_end
                        && remaining_range.start >= *source_start
                    {
                        // Remaining range left side overlap with this range
                        new_remaining_ranges.push(source_end..remaining_range.end);
                        destination_ranges
                            .push(dest_start + remaining_range.start - source_start..dest_end);
                    } else {
                        new_remaining_ranges.push(remaining_range);
                    }
                }
                (new_remaining_ranges, destination_ranges)
            },
        );
        destination_ranges.extend(remaining_range);
        destination_ranges
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
        separated_list1(space1, separated_pair(u64, space1, u64)),
        count(newline, 2),
    )(input)
    .unwrap();

    let (_, maps) = separated_list1(count(newline, 2), map_parser)(input).unwrap();

    let range_iter = seeds
        .into_iter()
        .flat_map(|(seed_start, seed_len)| {
            #[allow(clippy::single_range_in_vec_init)]
            maps.iter()
                .fold(vec![(seed_start..seed_start + seed_len)], |val, map| {
                    val.into_iter().flat_map(|val| map.lookup(val)).collect()
                })
        });
    // let ranges: Vec<_> = range_iter.collect();
    // dbg!(ranges);
    range_iter.map(|range| range.start).min().unwrap().to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
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
        assert_eq!(solve(input), "46");
    }
}
