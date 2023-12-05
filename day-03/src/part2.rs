use nom::AsChar;

use crate::custom_error::AocError;

fn construct_num(v: &[u8], idx: i32) -> u32 {
    let mut res: (i32, i32) = (idx, idx);
    if idx != 0 {
        while let Some(k) = v.get((res.0 - 1) as usize) {
            match k.is_dec_digit() {
                true => {
                    res.0 -= 1;
                    if res.0 == 0 {
                        break;
                    }
                }
                false => break,
            }
        }
    }
    while let Some(k) = v.get((res.1 + 1) as usize) {
        match k.is_dec_digit() {
            true => {
                res.1 += 1;
            }
            false => break,
        }
    }
    unsafe {
        String::from_utf8_unchecked(v[res.0 as usize..(res.1 + 1) as usize].to_vec())
            .parse::<u32>()
            .expect("Should be valid")
    }
}

fn crawler(v: &[&[u8]], idx: (i32, i32)) -> u32 {
    let neighbour_up_down = [-1, 0, 1];
    let neighbout_line = [-1, 1];
    let mut numbers = vec![];
    for x in neighbour_up_down {
        if let Some(row) = v.get((idx.1 + 1) as usize) {
            if let Some(num) = row.get((idx.0 + x).min(0) as usize) {
                if num.is_dec_digit() {
                    numbers.push(construct_num(row, (idx.0 + x).min(0)));
                    break;
                }
            }
        }
    }
    for x in neighbout_line {
        if let Some(row) = v.get(idx.1 as usize) {
            if idx.0 + x == -1 {
                continue;
            }
            if let Some(num) = row.get((idx.0 + x) as usize) {
                if num.is_dec_digit() {
                    numbers.push(construct_num(row, (idx.0 + x).min(0)));
                    break;
                }
            }
        }
    }
    for x in neighbour_up_down {
        if idx.1 - 1 == -1 {
            continue;
        }
        if let Some(row) = v.get((idx.1 - 1) as usize) {
            if let Some(num) = row.get((idx.0 + x).min(0) as usize) {
                if num.is_dec_digit() {
                    numbers.push(construct_num(row, (idx.0 + x).min(0)));
                    break;
                }
            }
        }
    }
    dbg!(&numbers);
    match numbers.len().cmp(&2) {
        std::cmp::Ordering::Less => 0,
        _ => numbers.into_iter().product::<u32>(),
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let to_vec = input.lines().map(|a| a.as_bytes()).collect::<Vec<&[u8]>>();
    let mut res = 0;
    for i in 0..to_vec.len() {
        for j in 0..to_vec.len() {
            if to_vec[i][j] == b'*' {
                res += crawler(&to_vec, (j as i32, i as i32))
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
...$.*....
.664.598..";
        assert_eq!("467835", process(input)?);
        Ok(())
    }
}
