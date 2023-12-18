use crate::custom_error::AocError;

use nom::bytes::complete::{tag, take_till};
use nom::character::complete::{self, alpha1, line_ending};
use nom::multi::separated_list1;
use nom::sequence::{pair, terminated, tuple};
use nom::{IResult, Parser};

enum MoveDirection {
    Up,
    Down,
    Right,
    Left,
}

impl From<char> for MoveDirection {
    fn from(ch: char) -> Self {
        match ch {
            'U' => Self::Up,
            'D' => Self::Down,
            'L' => Self::Left,
            'R' => Self::Right,
            _ => Self::Down, // should not be possible
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<(MoveDirection, i64, &str)>> {
    let (input, map) = separated_list1(
        pair(tag(")"), line_ending),
        tuple((
            terminated(alpha1, tag(" "))
                .map(|k: &str| MoveDirection::from(k.parse::<char>().expect("Shoud be valid"))),
            terminated(complete::i64, tag(" (")),
            take_till(|c| c == ')'),
        )),
    )
    .parse(input)?;

    Ok((input, map))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, steps_map) = parse_input(input).expect("Should be valid");
    let mut res = 0;
    let mut length = 0;
    let mut y = 0;
    for (direction, step, _) in steps_map {
        length += step;
        match direction {
            MoveDirection::Down => y += step,
            MoveDirection::Up => y -= step,
            MoveDirection::Left => res += step * y,
            MoveDirection::Right => res -= step * y,
        }
    }
    Ok((res.abs() + length / 2 + 1).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!("62", process(input)?);
        Ok(())
    }
}
