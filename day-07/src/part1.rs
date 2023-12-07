use std::collections::{BTreeMap, HashMap};

use itertools::Itertools;

use crate::custom_error::AocError;
const HAND_TYPE: [&str; 7] = ["11111", "1112", "122", "113", "23", "14", "5"];

#[derive(Debug)]
struct CamelCard {
    card_type: u8,
    card: (u8, u8, u8, u8, u8),
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

fn calculate_card_type(input: &str) -> u8 {
    let mut my_hash: HashMap<u8, i32> = HashMap::new();
    input.as_bytes().iter().for_each(|v| {
        my_hash.entry(*v).and_modify(|k| *k += 1).or_insert(1);
    });
    let hand = my_hash.values().sorted_unstable().join("");
    for (idx, k) in HAND_TYPE.iter().enumerate() {
        if k == &hand.as_str() {
            return idx as u8;
        }
    }
    unreachable!()
}

fn generate_tuple(input: &str) -> (u8, u8, u8, u8, u8) {
    input
        .chars()
        .map(|card| match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            value => value.to_digit(10).unwrap() as u8,
        })
        .collect_tuple::<(u8, u8, u8, u8, u8)>()
        .unwrap()
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut map = input.lines().fold(
        BTreeMap::new(),
        |mut acc: BTreeMap<CamelCard, u32>, line| {
            let (card, bid) = line.split_once(' ').unwrap();
            let camel_card = CamelCard {
                card_type: calculate_card_type(card),
                card: generate_tuple(card),
            };
            acc.insert(camel_card, bid.parse::<u32>().unwrap());
            acc
        },
    );
    let mut count = 1;
    let mut res = 0;
    while let Some((_, bid)) = map.pop_first() {
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
        assert_eq!("6440", process(input)?);
        Ok(())
    }
}
