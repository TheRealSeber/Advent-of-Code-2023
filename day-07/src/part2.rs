use std::collections::HashMap;

use itertools::Itertools;

use crate::custom_error::AocError;
const HAND_TYPE: [&str; 7] = ["11111", "1112", "122", "113", "23", "14", "5"];

#[derive(Debug)]
struct CamelCard {
    card_type: u8,
    card: (u8, u8, u8, u8, u8),
}

impl CamelCard {
    fn new(card: &str) -> Self {
        let mut my_hash: HashMap<u8, i32> = HashMap::new();
        let card_strength = card
            .chars()
            .map(|card| {
                let new_value = match card {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 1,
                    'T' => 10,
                    value => value.to_digit(10).unwrap() as u8,
                };
                my_hash
                    .entry(new_value)
                    .and_modify(|k| *k += 1)
                    .or_insert(1);
                new_value
            })
            .collect_tuple::<(u8, u8, u8, u8, u8)>()
            .unwrap();
        if let Some(apperances) = my_hash.remove(&1) {
            if let Some(max_ammount) = my_hash.values().max() {
                let mut max_key = 0;
                my_hash.iter().for_each(|(key, value)| {
                    if *key > max_key && max_ammount == value {
                        max_key = *key;
                    }
                });
                my_hash.entry(max_key).and_modify(|k| *k += apperances);
            } else {
                my_hash.insert(14, 5);
            }
        }
        Self {
            card_type: Self::calculate_card_type(my_hash),
            card: card_strength,
        }
    }

    fn calculate_card_type(input: HashMap<u8, i32>) -> u8 {
        let hand = input.values().sorted_unstable().join("");
        for (idx, k) in HAND_TYPE.iter().enumerate() {
            if k == &hand.as_str() {
                return idx as u8;
            }
        }
        unreachable!()
    }
}

impl Ord for CamelCard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.card_type.cmp(&other.card_type) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => self.card.cmp(&other.card),
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        }
    }
}

impl PartialOrd for CamelCard {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CamelCard {
    fn eq(&self, other: &Self) -> bool {
        self.card_type == other.card_type && self.card == other.card
    }
}

impl Eq for CamelCard {}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut hands: Vec<(CamelCard, u32)> = input
        .lines()
        .map(|line| {
            let (card, bid) = line.split_once(' ').unwrap();
            let camel_card = CamelCard::new(card);
            (camel_card, bid.parse::<u32>().unwrap())
        })
        .collect();

    hands.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    let mut count = 1;
    let mut res = 0;
    for (_, bid) in hands {
        res += bid * count;
        count += 1;
    }
    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!("5905", process(input)?);
        Ok(())
    }
}
