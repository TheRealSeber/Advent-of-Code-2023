use crate::custom_error::AocError;
use nom::{
    bytes::complete::take_until,
    character::complete::{self, line_ending, space1},
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult, Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};
use std::ops::Range;

#[derive(Debug)]
struct SeedMap {
    mappings: Vec<(Range<u64>, Range<u64>)>,
}

impl SeedMap {
    fn get_map(&self, source: u64) -> u64 {
        let valid_mapping = self
            .mappings
            .iter()
            .find(|(source_map, _)| source_map.contains(&source));
        match valid_mapping {
            Some((source_map, destination_map)) => {
                destination_map.start + (source - source_map.start)
            }
            None => source,
        }
    }
}

fn create_map(input: &str) -> IResult<&str, (Range<u64>, Range<u64>)> {
    let (input, (destination, source, range)) = tuple((
        complete::u64,
        complete::u64.preceded_by(tag(" ")),
        complete::u64.preceded_by(tag(" ")),
    ))(input)?;

    Ok((
        input,
        (
            (source..source + range),
            (destination..source + destination),
        ),
    ))
}

#[tracing::instrument]
fn seed_map(input: &str) -> IResult<&str, SeedMap> {
    take_until("map:")
        .precedes(tag("map:"))
        .precedes(many1(line_ending.precedes(create_map)).map(|mappings| SeedMap { mappings }))
        .parse(input)
}

#[tracing::instrument]
fn parse_seeds(input: &str) -> IResult<&str, (Vec<SeedMap>, Vec<u64>)> {
    let (input, seeds) = tag("seeds: ")
        .precedes(separated_list1(space1, complete::u64))
        .parse(input)?;
    let (input, maps) = many1(seed_map)(input)?;
    Ok((input, (maps, seeds)))
}
#[tracing::instrument(skip(input), fields(input_first_line = input.split('\n').next().unwrap()))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, (seedmaps, seeds)) = parse_seeds(input).expect("Should be valid");
    let min_loc = seeds
        .iter()
        .map(|seed| {
            seedmaps
                .iter()
                .fold(*seed, |seed, seed_map| seed_map.get_map(seed))
        })
        .min()
        .expect("Should exist");
    Ok(min_loc.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
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
        assert_eq!("35", process(input)?);
        Ok(())
    }
}
