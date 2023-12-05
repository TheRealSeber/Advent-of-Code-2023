use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let num_str = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    Ok(input
        .lines()
        .fold(0_i32, |acc, x| {
            let (mut l_dg, mut r_dg) = (0_u8, 0_u8);
            for i in 0..x.len() {
                for (idx, s) in num_str.iter().enumerate() {
                    if x[i..].starts_with(s) {
                        if l_dg != 0 {
                            r_dg = (idx + 1) as u8;
                        } else {
                            (l_dg, r_dg) = ((idx + 1) as u8, (idx + 1) as u8);
                        }
                        break;
                    }
                }
                if let Ok(v) = x[i..i + 1].parse::<u8>() {
                    if l_dg != 0 {
                        r_dg = v;
                    } else {
                        (l_dg, r_dg) = (v, v);
                    }
                }
            }
            acc + (l_dg * 10 + r_dg) as i32
        })
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process(input)?);
        Ok(())
    }
}
