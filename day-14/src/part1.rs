use crate::custom_error::AocError;
use nom::{
    bytes::complete::is_a, character::complete::line_ending, multi::separated_list1, IResult,
};

#[derive(Debug)]
enum Character {
    Dot,
    RoundedRock,
    CubedRock,
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Character>>> {
    let (input, lines) = separated_list1(line_ending, is_a("#.O"))(input)?;
    let charachter_lines = lines
        .into_iter()
        .map(|k| {
            k.chars()
                .map(|ch| match ch {
                    'O' => Character::RoundedRock,
                    '#' => Character::CubedRock,
                    _ => Character::Dot,
                })
                .collect()
        })
        .collect::<Vec<Vec<Character>>>();
    Ok((input, charachter_lines))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, lines) = parse_input(input).expect("Should be valid");
    let mut res = 0;
    for x in 0..lines[0].len() {
        let mut current_weight = lines.len();
        for y in 0..lines.len() {
            match lines[y][x] {
                Character::CubedRock => current_weight = lines.len() - y - 1,
                Character::RoundedRock => {
                    res += current_weight;
                    current_weight -= 1;
                }
                _ => {}
            }
        }
    }
    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!("136", process(input)?);
        Ok(())
    }
}
