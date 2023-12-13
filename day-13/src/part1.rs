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
    'outer: for x in 1..row_vectors.len().max(columns_vectors.len()) {
        if x < row_vectors.len() && row_vectors[x] == row_vectors[x - 1] {
            for i in 1..row_vectors.len() - x {
                if x > i && row_vectors[x - i - 1] != row_vectors[x + i] {
                    // here is just a repetition of the lines 57-63
                    // because we can miss a possible solution
                    if x < columns_vectors.len() && columns_vectors[x] == columns_vectors[x - 1] {
                        for j in 1..columns_vectors.len() - x {
                            if x > j && columns_vectors[x - j - 1] != columns_vectors[x + j] {
                                continue 'outer;
                            }
                        }
                        return x as u32;
                    }
                    continue 'outer;
                    //
                    //
                }
            }
            return x as u32 * 100;
        }
        if x < columns_vectors.len() && columns_vectors[x] == columns_vectors[x - 1] {
            for i in 1..columns_vectors.len() - x {
                if x > i && columns_vectors[x - i - 1] != columns_vectors[x + i] {
                    continue 'outer;
                }
            }
            return x as u32;
        }
    }
    unreachable!();
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
        assert_eq!("405", process(input)?);
        Ok(())
    }
}
