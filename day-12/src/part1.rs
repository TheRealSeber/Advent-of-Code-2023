use crate::custom_error::AocError;

use nom::{
    bytes::complete::{is_a, tag},
    character::complete::{self, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

#[derive(Debug, PartialEq)]
enum LineCharacter {
    Questionmark,
    Dot,
    Hashtag,
}
#[derive(Debug)]
struct Puzzle {
    spring: Vec<LineCharacter>,
    batches: Vec<u32>,
}

fn parse_line(input: &str) -> IResult<&str, Puzzle> {
    let (input, (line, batches)) = separated_pair(
        is_a("#.?").map(|s: &str| s.trim_end_matches('.')),
        space1,
        separated_list1(tag(","), complete::u32),
    )
    .parse(input)?;
    let spring = line
        .chars()
        .map(|ch| match ch {
            '#' => LineCharacter::Hashtag,
            '.' => LineCharacter::Dot,
            _ => LineCharacter::Questionmark,
        })
        .collect::<Vec<LineCharacter>>();
    Ok((input, Puzzle { spring, batches }))
}

fn is_valid_permutatuion(puzzle: &Vec<LineCharacter>, goal_batches: &[u32]) -> bool {
    let mut counter = 0_u32;
    let mut batches_iter = goal_batches.iter();
    for i in puzzle {
        match *i {
            LineCharacter::Hashtag => {
                counter += 1;
            }
            _ => {
                if counter > 0 {
                    match batches_iter.next() {
                        Some(v) => match *v == counter {
                            true => counter = 0,
                            false => return false,
                        },
                        None => return false,
                    }
                }
            }
        }
    }
    match counter.cmp(&0) {
        std::cmp::Ordering::Greater => match batches_iter.next() {
            Some(v) => *v == counter && batches_iter.next().is_none(),
            None => false,
        },
        _ => batches_iter.next().is_none(),
    }
}

fn generate_all_permutations(
    remaining_puzzle: &[LineCharacter],
    current: &mut Vec<LineCharacter>,
    goal_batches: &Vec<u32>,
    result: &mut u32,
) {
    if remaining_puzzle.is_empty() {
        if is_valid_permutatuion(current, goal_batches) {
            *result += 1;
        }
        return;
    }

    match remaining_puzzle[0] {
        LineCharacter::Questionmark => {
            current.push(LineCharacter::Hashtag);
            generate_all_permutations(&remaining_puzzle[1..], current, goal_batches, result);
            current.pop();
            current.push(LineCharacter::Dot);
            generate_all_permutations(&remaining_puzzle[1..], current, goal_batches, result);
            current.pop();
        }
        LineCharacter::Dot => {
            current.push(LineCharacter::Dot);
            generate_all_permutations(&remaining_puzzle[1..], current, goal_batches, result);
            current.pop();
        }
        LineCharacter::Hashtag => {
            current.push(LineCharacter::Hashtag);
            generate_all_permutations(&remaining_puzzle[1..], current, goal_batches, result);
            current.pop();
        }
    }
}

fn process_line(input: &str) -> IResult<&str, u32> {
    let (input, puzzle) = parse_line(input)?;
    let mut result = 0_u32;

    generate_all_permutations(
        &puzzle.spring,
        &mut Vec::with_capacity(puzzle.spring.len()),
        &puzzle.batches,
        &mut result,
    );

    Ok((input, result))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let result = input.lines().fold(0_u32, |acc, line| {
        let (_, combinations) = process_line(line).expect("Should be valid");
        acc + combinations
    });
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 4)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 1)]
    #[case("????.######..#####. 1,6,5", 4)]
    #[case("?###???????? 3,2,1", 10)]
    #[test_log::test]
    fn test_line(#[case] input: &str, #[case] output: u32) -> miette::Result<()> {
        assert_eq!(output.to_string(), process(input)?);
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!("21", process(input)?);
        Ok(())
    }
}
