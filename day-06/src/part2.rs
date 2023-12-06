use crate::custom_error::AocError;

use nom::{
    bytes::complete::is_not,
    character::complete::{digit1, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use nom_supreme::ParserExt;

fn nums(input: &str) -> IResult<&str, u64> {
    is_not("0123456789")
        .precedes(separated_list1(space1, digit1))
        .map(|list| list.join("").parse::<u64>().expect("Should be valid"))
        .parse(input)
}

fn parse(input: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(nums, line_ending, nums).parse(input)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, (test, distance)) = parse(input).expect("Should be valid");
    let mut k = 1_u64;
    let mut res = 0_u64;
    while k < test {
        if ((test - k) * k) > distance {
            res += 1;
        }
        k += 1;
    }
    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("71503", process(input)?);
        Ok(())
    }
}
