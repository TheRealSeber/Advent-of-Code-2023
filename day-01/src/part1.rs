use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    Ok(input
        .lines()
        .fold(0_i32, |acc, x| {
            let res = x
                .chars()
                .fold((0_u8, 0_u8), |(x, y), ch| match ch.to_digit(10) {
                    Some(v) => {
                        if x != 0 {
                            (x, v as u8)
                        } else {
                            (v as u8, v as u8)
                        }
                    }
                    None => (x, y),
                });
            acc + (res.0 * 10 + res.1) as i32
        })
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}
