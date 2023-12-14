use crate::custom_error::AocError;
use nom::{
    bytes::complete::is_a, character::complete::line_ending, multi::separated_list1, IResult,
};

use std::{collections::HashMap, ptr::swap};

const CYCLES: u32 = 1000000000;

#[derive(Debug, PartialEq)]
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

fn rotate(input: &mut Vec<Vec<Character>>) {
    let vec_size = input.len();
    for i in 0..vec_size / 2 {
        for j in i..vec_size - i - 1 {
            unsafe {
                swap(&mut input[i][j], &mut input[vec_size - j - 1][i]);
                swap(
                    &mut input[vec_size - j - 1][i],
                    &mut input[vec_size - i - 1][vec_size - j - 1],
                );
                swap(
                    &mut input[vec_size - i - 1][vec_size - j - 1],
                    &mut input[j][vec_size - i - 1],
                );
            }
        }
    }
}

fn move_rocks(input: &mut Vec<Vec<Character>>) {
    for x in 0..input[0].len() {
        let mut last_free_place = 0;
        for y in 0..input.len() {
            match input[y][x] {
                Character::CubedRock => last_free_place = y + 1,
                Character::RoundedRock => {
                    unsafe { swap(&mut input[last_free_place][x], &mut input[y][x]) }
                    last_free_place += 1;
                }
                _ => {}
            }
        }
    }
}

fn create_vec_id(vec_chars: &[Vec<Character>]) -> [u64; 157] {
    // Encode the locations of the rounded rocks, assuming grid is 100x100 max.
    //
    // NOTE: `64 * 157 > 100 * 100`
    let mut bits = [0; 157];
    for (idx, obj) in vec_chars.iter().flatten().enumerate() {
        if *obj == Character::RoundedRock {
            bits[idx / 64] |= 1 << (idx % 64);
        }
    }
    bits
}

fn make_cycle(input: &mut Vec<Vec<Character>>) {
    for _ in 0..4 {
        move_rocks(input);
        rotate(input);
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, mut lines) = parse_input(input).expect("Should be valid");
    let mut seen = HashMap::new();
    for cycle in 0..CYCLES {
        make_cycle(&mut lines);
        if let Some(prev_step) = seen.insert(create_vec_id(&lines), cycle) {
            let remaining = CYCLES - 1 - cycle;
            let period = cycle - prev_step;
            for _ in 0..remaining % period {
                make_cycle(&mut lines);
            }
            break;
        }
    }
    let mut res = 0;
    for x in 0..lines[0].len() {
        for y in 0..lines.len() {
            if lines[y][x] == Character::RoundedRock {
                res += lines.len() - y;
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
        assert_eq!("64", process(input)?);
        Ok(())
    }
}
