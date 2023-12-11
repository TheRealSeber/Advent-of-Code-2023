use std::collections::VecDeque;

use crate::custom_error::AocError;

fn extract_around<'a>(
    (y, x): (usize, usize),
    vec: &'a [(usize, Vec<(usize, &'a u8)>)],
    was_visited: &mut Vec<Vec<bool>>,
) -> Option<Vec<((&'a usize, &'a usize), &'a &'a u8)>> {
    let mut res = Vec::new();
    if [b'S', b'|', b'L', b'J'].contains(vec[y].1[x].1)
        && y > 0
        && [b'F', b'7', b'|'].contains(vec[y - 1].1[x].1)
        && !was_visited[y - 1][x]
    {
        res.push(extract_given((y - 1, x), vec));
        was_visited[y - 1][x] = true;
    };
    if [b'S', b'|', b'F', b'7'].contains(vec[y].1[x].1)
        && y + 1 < vec.len()
        && [b'J', b'L', b'|'].contains(vec[y + 1].1[x].1)
        && !was_visited[y + 1][x]
    {
        res.push(extract_given((y + 1, x), vec));
        was_visited[y + 1][x] = true;
    };
    if [b'S', b'-', b'J', b'7'].contains(vec[y].1[x].1)
        && x > 0
        && [b'L', b'-', b'F'].contains(vec[y].1[x - 1].1)
        && !was_visited[y][x - 1]
    {
        res.push(extract_given((y, x - 1), vec));
        was_visited[y][x - 1] = true;
    };
    if [b'S', b'-', b'F', b'L'].contains(vec[y].1[x].1)
        && x + 1 < vec[0].1.len()
        && [b'J', b'7', b'-'].contains(vec[y].1[x + 1].1)
        && !was_visited[y][x + 1]
    {
        res.push(extract_given((y, x + 1), vec));
        was_visited[y][x + 1] = true;
    };
    match res.is_empty() {
        true => None,
        false => Some(res),
    }
}

fn extract_given<'a>(
    (y, x): (usize, usize),
    vec: &'a [(usize, Vec<(usize, &'a u8)>)],
) -> ((&usize, &usize), &&'a u8) {
    let x_entry = &vec[y];
    ((&x_entry.0, &x_entry.1[x].0), &x_entry.1[x].1)
}

#[tracing::instrument]
#[warn(unreachable_code)]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut starting_point = (input.find('S').expect("Should exist"), 0);
    let lines: Vec<(usize, Vec<(usize, &u8)>)> = input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(idx, byte)| {
                    if b'S' == *byte {
                        starting_point.1 = idx
                    };
                    (idx, byte)
                })
                .collect::<Vec<(usize, &u8)>>()
        })
        .enumerate()
        .map(|(idx, k)| {
            if k.iter().any(|(_, k)| k == &&b'S') {
                starting_point.0 = idx
            };
            (idx, k)
        })
        .collect::<Vec<(usize, Vec<(usize, &u8)>)>>();
    let mut queue: VecDeque<(u32, ((&usize, &usize), &&u8))> = VecDeque::new();
    let mut visited_map = vec![vec![false; lines[0].1.len()]; lines.len()];
    queue.push_back((0_u32, ((&starting_point.0, &starting_point.1), &&b'S')));
    loop {
        let point = queue.pop_front().expect("Exists!");
        if let Some(v) = extract_around((*point.1 .0 .0, *point.1 .0 .1), &lines, &mut visited_map)
        {
            v.into_iter().for_each(|neighbour| {
                queue.push_back((point.0 + 1, neighbour));
            })
        }
        if queue.is_empty() {
            return Ok(point.0.to_string());
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!("8", process(input)?);
        Ok(())
    }
}
