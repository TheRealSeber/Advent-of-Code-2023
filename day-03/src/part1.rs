use nom::AsChar;

use crate::custom_error::AocError;

macro_rules! is_special {
    ($byte:expr) => {
        !($byte == b'.' || $byte.is_dec_digit())
    };
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut res = 0;
    let to_vec = input.lines().map(|a| a.as_bytes()).collect::<Vec<&[u8]>>();
    let mut in_num = false;
    let mut start_index = 0;
    let line_len = to_vec[0].len();
    for i in 0..line_len {
        let mut special_index = i32::MIN;
        for j in 0..line_len {
            if (i > 0 && is_special!(to_vec[i - 1][j]))
                || (i < line_len - 1 && is_special!(to_vec[i + 1][j]))
                || is_special!(to_vec[i][j])
            {
                special_index = j as i32;
            }
            if to_vec[i][j].is_dec_digit() && !in_num {
                start_index = j;
                in_num = true;
            }
            if in_num && (!to_vec[i][j].is_dec_digit() || j + 1 == line_len) {
                if special_index >= start_index as i32 - 1 {
                    if !to_vec[i][j].is_dec_digit() {
                        res += String::from_utf8_lossy(&to_vec[i][start_index..j])
                            .parse::<i32>()
                            .expect("Valid number");
                    } else if j + 1 == line_len {
                        res += String::from_utf8_lossy(&to_vec[i][start_index..])
                            .parse::<i32>()
                            .expect("Valid number");
                    }
                }
                in_num = false;
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
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*.*..
.664...598";
        assert_eq!("4361", process(input)?);
        Ok(())
    }
}
