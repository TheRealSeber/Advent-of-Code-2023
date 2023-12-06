use crate::custom_error::AocError;

use nom::{
    bytes::complete::is_not,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use nom_supreme::ParserExt;

fn nums(input: &str) -> IResult<&str, Vec<u32>> {
    is_not("0123456789")
        .precedes(separated_list1(space1, complete::u32))
        .parse(input)
}

fn parse(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    separated_pair(nums, line_ending, nums).parse(input)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, (time, distance)) = parse(input).expect("Should be valid");
    let res = time.into_iter().zip(distance).fold(1, |acc, (x, y)| {
        acc * (1..x).fold(0, |poss, k| match ((x - k) * k).cmp(&y) {
            std::cmp::Ordering::Greater => poss + 1,
            _ => poss,
        })
    });
    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("288", process(input)?);
        Ok(())
    }
}
