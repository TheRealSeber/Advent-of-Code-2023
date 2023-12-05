use std::collections::{HashMap, HashSet};

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let dataset_start_point = 6 + input.len().ilog10() as usize;
    Ok(input
        .lines()
        .enumerate()
        .fold(
            (0_u32, HashMap::new()),
            |(acc, mut my_hashmap), (idx, line)| {
                my_hashmap
                    .entry(idx + 1)
                    .and_modify(|k| *k += 1)
                    .or_insert(1);
                let matched = line[dataset_start_point..]
                    .split(' ')
                    .filter(|a| !a.is_empty())
                    .fold(
                        (0_u32, HashSet::new(), false),
                        |(found, mut my_hashset, passed_pipe), v| {
                            if v == "|" {
                                (found, my_hashset, true)
                            } else {
                                match passed_pipe {
                                    true => {
                                        if my_hashset
                                            .contains(&v.parse::<u32>().expect("Should be valid"))
                                        {
                                            (found + 1, my_hashset, passed_pipe)
                                        } else {
                                            (found, my_hashset, passed_pipe)
                                        }
                                    }
                                    false => {
                                        my_hashset
                                            .insert(v.parse::<u32>().expect("Should be valid"));
                                        (found, my_hashset, passed_pipe)
                                    }
                                }
                            }
                        },
                    )
                    .0;
                let card_num_entries = &my_hashmap.remove(&(idx + 1)).expect("Should exist");
                (idx + 2..idx + 2 + matched as usize).for_each(|v| {
                    my_hashmap
                        .entry(v)
                        .and_modify(|k| *k += *card_num_entries)
                        .or_insert(*card_num_entries);
                });
                (acc + card_num_entries, my_hashmap)
            },
        )
        .0
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("30", process(input)?);
        Ok(())
    }
}
