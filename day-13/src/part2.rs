use crate::custom_error::AocError;
use nom::{
    bytes::complete::is_a, character::complete::line_ending, multi::separated_list1,
    sequence::pair, IResult,
};

#[derive(Debug, Clone, PartialEq)]
enum Character {
    Hashtag,
    Dot,
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<&str>>> {
    let (input, lava) = separated_list1(
        pair(line_ending, line_ending),
        separated_list1(line_ending, is_a("#.")),
    )(input)?;
    Ok((input, lava))
}
#[warn(unreachable_code)]
fn calculate_single_lava(lava: Vec<&[u8]>) -> u32 {
    let mut columns_vectors: Vec<Vec<Character>> =
        vec![Vec::with_capacity(lava.len()).clone(); lava[0].len()];
    let row_vectors = lava
        .into_iter()
        .map(|k| {
            k.iter()
                .enumerate()
                .map(|(idx, v)| match v {
                    b'.' => {
                        columns_vectors[idx].push(Character::Dot);
                        Character::Dot
                    }
                    _ => {
                        columns_vectors[idx].push(Character::Hashtag);
                        Character::Hashtag
                    }
                })
                .collect()
        })
        .collect::<Vec<Vec<Character>>>();
    for x in 1..row_vectors.len().max(columns_vectors.len()) {
        if x < row_vectors.len() {
            let mut differences = 0_u32;
            for i in 0..row_vectors.len() - x {
                if x > i {
                    count_differences(x, i, &row_vectors, &mut differences);
                    if differences > 1 {
                        break;
                    }
                }
            }
            if differences == 1 {
                return x as u32 * 100;
            }
        }
        if x < columns_vectors.len() {
            let mut differences = 0_u32;
            for i in 0..columns_vectors.len() - x {
                if x > i {
                    count_differences(x, i, &columns_vectors, &mut differences);
                    if differences > 1 {
                        break;
                    }
                }
            }
            if differences == 1 {
                return x as u32;
            }
        }
    }
    unreachable!();
}

fn count_differences(
    x: usize,
    i: usize,
    characters: &Vec<Vec<Character>>,
    count_differences: &mut u32,
) {
    for (idx, k) in (*characters[x + i]).iter().enumerate() {
        if (*characters[x - i - 1])[idx] != *k {
            *count_differences += 1;
            if *count_differences > 1 {
                return;
            }
        }
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, lavas) = parse_input(input).expect("Should be valid!");
    let res = lavas.into_iter().fold(0_u32, |acc, lava| {
        acc + calculate_single_lava(lava.into_iter().map(|s| s.as_bytes()).collect())
    });
    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!("400", process(input)?);
        Ok(())
    }
}
