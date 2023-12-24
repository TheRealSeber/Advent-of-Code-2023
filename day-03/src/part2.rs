use std::collections::HashSet;

use nom::AsChar;

use crate::custom_error::AocError;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Entry {
    Default,
    Number(u32),
    Star,
}

fn neighbours_product((y, x): &(usize, usize), map: &Vec<Vec<Entry>>) -> u32 {
    let mut neighbours = HashSet::new();
    if *y > 0 {
        if let Entry::Number(num) = map[*y - 1][*x] {
            neighbours.insert(num);
        }
        if *x > 0 {
            if let Entry::Number(num) = map[*y - 1][*x - 1] {
                neighbours.insert(num);
            }
        }
        if *x + 1 < map[0].len() {
            if let Entry::Number(num) = map[*y - 1][*x + 1] {
                neighbours.insert(num);
            }
        }
    }
    if *y + 1 < map.len() {
        if let Entry::Number(num) = map[*y + 1][*x] {
            neighbours.insert(num);
        }
        if *x > 0 {
            if let Entry::Number(num) = map[*y + 1][*x - 1] {
                neighbours.insert(num);
            }
        }
        if *x + 1 < map[0].len() {
            if let Entry::Number(num) = map[*y + 1][*x + 1] {
                neighbours.insert(num);
            }
        }
    }
    if *x > 0 {
        if let Entry::Number(num) = map[*y][*x - 1] {
            neighbours.insert(num);
        }
    }
    if *x + 1 < map[0].len() {
        if let Entry::Number(num) = map[*y][*x + 1] {
            neighbours.insert(num);
        }
    }
    if neighbours.len() == 2 {
        neighbours.into_iter().fold(1, |acc, x| acc * x)
    } else {
        0
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut map = vec![vec![Entry::Default; lines[0].len()]; lines.len()];
    for (y, line) in lines.into_iter().enumerate() {
        let mut num_starting_point = 0;
        let mut in_num = false;
        for (x, ch) in line.chars().enumerate() {
            if ch == '*' {
                map[y][x] = Entry::Star;
            }
            if ch.is_dec_digit() {
                if !in_num {
                    num_starting_point = x;
                    in_num = true;
                }
                if x + 1 == line.len() && in_num {
                    for i in num_starting_point..x + 1 {
                        map[y][i] = Entry::Number(
                            line[num_starting_point..x + 1]
                                .parse::<u32>()
                                .expect("Should be valid"),
                        );
                    }
                }
            }
            if !ch.is_dec_digit() {
                if in_num {
                    for i in num_starting_point..x {
                        map[y][i] = Entry::Number(
                            line[num_starting_point..x]
                                .parse::<u32>()
                                .expect("Should be valid"),
                        );
                    }
                }
                in_num = false;
            }
        }
    }
    let mut res = 0_u32;
    for (y, line) in map.iter().enumerate() {
        for (x, entry) in line.iter().enumerate() {
            if *entry == Entry::Star {
                res += neighbours_product(&(y, x), &map);
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
        assert_eq!("467835", process(input)?);
        Ok(())
    }
}
