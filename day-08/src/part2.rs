use std::collections::HashMap;

use crate::custom_error::AocError;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alphanumeric1, line_ending, multispace1},
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
                    alphanumeric1,
                    tag(" = ("),
                    separated_pair(alphanumeric1, tag(", "), alphanumeric1),
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
    let current_places = {
        map.iter()
            .filter(|(key, _)| key.ends_with('A'))
            .map(|(_, places)| places)
            .collect::<Vec<&(&str, &str)>>()
    };
    let min_steps_each_map = current_places
        .iter()
        .map(|node| {
            let mut current_node = *node;
            instructions
                .iter()
                .cycle()
                .enumerate()
                .find_map(|(index, instruction)| match instruction {
                    Move::Left => {
                        if current_node.0.ends_with('Z') {
                            return Some(index + 1);
                        }
                        current_node = map.get(current_node.0).expect("Should exist");
                        None
                    }
                    Move::Right => {
                        if current_node.1.ends_with('Z') {
                            return Some(index + 1);
                        }
                        current_node = map.get(current_node.1).expect("Should exist");
                        None
                    }
                })
                .expect("Should exist")
        })
        .collect::<Vec<usize>>();
    let min_cycle = lcm(&min_steps_each_map);

    Ok(min_cycle.to_string())
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
