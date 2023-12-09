use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let result = input
        .lines()
        .map(|k| {
            k.split_whitespace()
                .map(|v| v.parse::<i32>().expect("Should be valid"))
                .collect::<Vec<i32>>()
        })
        .fold(0, |sum, x| {
            sum + x.last().expect("Must exist") + vectorize(&x)
        });
    Ok(result.to_string())
}

fn vectorize(slice: &[i32]) -> i32 {
    let new_vec = slice.windows(2).map(|k| k[1] - k[0]).collect::<Vec<i32>>();
    match new_vec.iter().all(|v| *v == 0) {
        true => 0,
        false => new_vec.last().expect("Should exist") + vectorize(&new_vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!("114", process(input)?);
        Ok(())
    }
}
