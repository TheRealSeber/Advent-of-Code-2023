use std::collections::HashMap;

use crate::custom_error::AocError;

fn hash_label(string: &str) -> u8 {
    string
        .bytes()
        .fold(0, |acc, x| acc.wrapping_add(x).wrapping_mul(17))
}
#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let hashed_input = input.split(',').fold(
        HashMap::new(),
        |mut my_hash: HashMap<u8, Vec<(&str, u8)>>, word| {
            match word.ends_with('-') {
                true => {
                    let label = word.trim_end_matches('-');
                    if let Some(values) = my_hash.get_mut(&hash_label(label)) {
                        if let Some(idx) = values.iter().position(|(keyword, _)| *keyword == label)
                        {
                            values.remove(idx);
                        }
                    }
                }
                false => {
                    let (label, value) = word
                        .split_once('=')
                        .map(|k| (k.0, k.1.parse::<u8>().expect("Should be parsable")))
                        .expect("Should be splitable");
                    my_hash
                        .entry(hash_label(label))
                        .and_modify(|v| {
                            if let Some((_, focal)) =
                                v.iter_mut().find(|(keyword, _)| *keyword == label)
                            {
                                *focal = value;
                            } else {
                                v.push((label, value));
                            }
                        })
                        .or_insert(vec![(label, value)]);
                }
            }
            my_hash
        },
    );
    let result = hashed_input.into_iter().fold(0_u32, |acc, (box_num, v)| {
        acc + v
            .into_iter()
            .enumerate()
            .fold(0_u32, |sum, (idx, (_, focal_length))| {
                sum + ((box_num as u32 + 1) * (idx as u32 + 1) * focal_length as u32)
            })
    });
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!("145", process(input)?);
        Ok(())
    }
}
