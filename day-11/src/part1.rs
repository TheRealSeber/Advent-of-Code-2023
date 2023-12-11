use std::collections::HashSet;

use crate::custom_error::AocError;

#[derive(Debug)]
enum RowSpaced {
    NotSpaced,
    Row,
}
#[derive(Debug, PartialEq)]
enum Entry {
    Dot,
    ColumnSpacedDot,
    RowSpacedDot,
    ColumnRowSpacedDot,
    Galaxy,
}

fn distances_from_source(space: Vec<Vec<Entry>>) -> Vec<(u32, u32)> {
    let mut res = Vec::new();
    let mut additional_down_steps = 0_u32;
    space
        .into_iter()
        .enumerate()
        .for_each(|(idx_y, row_entries)| {
            let mut additional_right_steps = 0_u32;
            let mut added_down_step = false;
            row_entries
                .into_iter()
                .enumerate()
                .for_each(|(idx_x, entry)| match entry {
                    Entry::ColumnSpacedDot => additional_right_steps += 1,
                    Entry::RowSpacedDot => {
                        if !added_down_step {
                            additional_down_steps += 1;
                            added_down_step = true;
                        }
                    }
                    Entry::Galaxy => res.push((
                        idx_x as u32 + additional_right_steps,
                        idx_y as u32 + additional_down_steps,
                    )),
                    _ => {}
                })
        });
    res
}

fn parse_input(input: &str) -> Vec<Vec<Entry>> {
    let mut filled_columns = HashSet::new();
    let first_translation = input
        .lines()
        .map(|k| {
            let mut passed_galaxy = false;
            let entries = k
                .as_bytes()
                .iter()
                .enumerate()
                .map(|(idx, k)| match k {
                    b'.' => Entry::Dot,
                    _ => {
                        passed_galaxy = true;
                        filled_columns.insert(idx);
                        Entry::Galaxy
                    }
                })
                .collect::<Vec<Entry>>();
            let spaced = match passed_galaxy {
                true => RowSpaced::NotSpaced,
                false => RowSpaced::Row,
            };
            (spaced, entries)
        })
        .collect::<Vec<(RowSpaced, Vec<Entry>)>>();

    first_translation
        .into_iter()
        .map(|(rowspaced, k)| {
            k.into_iter()
                .enumerate()
                .map(
                    |(idx, v)| match (&rowspaced, filled_columns.contains(&idx), &v) {
                        (RowSpaced::NotSpaced, false, Entry::Dot) => Entry::ColumnSpacedDot,
                        (RowSpaced::Row, false, Entry::Dot) => Entry::ColumnRowSpacedDot,
                        (RowSpaced::Row, true, Entry::Dot) => Entry::RowSpacedDot,
                        _ => v,
                    },
                )
                .collect::<Vec<Entry>>()
        })
        .collect::<Vec<Vec<Entry>>>()
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let translation = parse_input(input);
    let mut res = 0;
    let dist_from_source = distances_from_source(translation);
    for (i, from) in dist_from_source.iter().enumerate() {
        for to in dist_from_source.iter().skip(i) {
            res += to.0.abs_diff(from.0) + to.1.abs_diff(from.1)
        }
    }
    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!("374", process(input)?);
        Ok(())
    }
}
