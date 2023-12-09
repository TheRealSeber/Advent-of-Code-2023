use std::collections::HashMap;

use crate::custom_error::AocError;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending, multispace1},
    multi::{fold_many1, many1},
    sequence::{separated_pair, terminated},
    IResult, Parser,
};

#[derive(Debug)]
enum Move {
    Left,
    Right,
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Move>> {
    let (input, instuctions) = many1(alt((
        complete::char('L').map(|_| Move::Left),
        complete::char('R').map(|_| Move::Right),
    )))
    .parse(input)?;
    Ok((input, instuctions))
}

fn parse_maps(input: &str) -> IResult<&str, HashMap<&str, (&str, &str)>> {
    let (input, _) = multispace1(input)?;
    let (input, maps) = fold_many1(
        terminated(
            terminated(
                separated_pair(
                    alpha1,
                    tag(" = ("),
                    separated_pair(alpha1, tag(", "), alpha1),
                ),
                tag(")"),
            ),
            line_ending,
        ),
        HashMap::new,
        |mut acc: HashMap<&str, (&str, &str)>, (key, map)| {
            acc.insert(key, map);
            acc
        },
    )
    .parse(input)?;
    Ok((input, maps))
}

#[tracing::instrument]
#[warn(unreachable_code)]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (input, instructions) = parse_instructions(input).expect("Should be valid!");
    let (_, map) = parse_maps(input).expect("Should be valid!");
    let mut counter = 0;
    let mut current_place = map.get("AAA").expect("Should exist");
    for next_move in instructions.iter().cycle() {
        counter += 1;
        current_place = match next_move {
            Move::Left => {
                if current_place.0 == "ZZZ" {
                    return Ok((counter).to_string());
                }
                map.get(current_place.0).expect("Should exist")
            }
            Move::Right => {
                if current_place.1 == "ZZZ" {
                    return Ok((counter).to_string());
                }
                map.get(current_place.1).expect("Should exist")
            }
        };
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
