use std::collections::VecDeque;

use nom::{
    branch::alt,
    character::complete::{self, line_ending},
    multi::{many1, separated_list1},
    IResult, Parser,
};

use crate::custom_error::AocError;

#[derive(Debug, PartialEq, Eq)]
enum Object {
    Start,
    Plot,
    Rock,
}

struct Entry {
    object: Object,
    visited: bool,
}

impl Entry {
    fn new(object_type: Object) -> Self {
        Self {
            object: object_type,
            visited: false,
        }
    }

    fn possible_neighbours_coordinates(
        (y, x): (usize, usize),
        entries_map: &mut Vec<Vec<Entry>>,
    ) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        if y > 0 && entries_map[y - 1][x].object != Object::Rock && !entries_map[y - 1][x].visited {
            entries_map[y - 1][x].visited = true;
            res.push((y - 1, x));
        }
        if y + 1 < entries_map.len()
            && entries_map[y + 1][x].object != Object::Rock
            && !entries_map[y + 1][x].visited
        {
            entries_map[y + 1][x].visited = true;
            res.push((y + 1, x));
        }
        if x > 0 && entries_map[y][x - 1].object != Object::Rock && !entries_map[y][x - 1].visited {
            entries_map[y][x - 1].visited = true;
            res.push((y, x - 1));
        }
        if x + 1 < entries_map[0].len()
            && entries_map[y][x + 1].object != Object::Rock
            && !entries_map[y][x + 1].visited
        {
            entries_map[y][x + 1].visited = true;
            res.push((y, x + 1));
        }
        res
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Entry>>> {
    let (input, map) = separated_list1(
        line_ending,
        many1(alt((
            complete::char('S').map(|_| Entry::new(Object::Start)),
            complete::char('.').map(|_| Entry::new(Object::Plot)),
            complete::char('#').map(|_| Entry::new(Object::Rock)),
        ))),
    )
    .parse(input)?;

    Ok((input, map))
}

#[tracing::instrument]
pub fn process(input: &str, steps: u32) -> miette::Result<String, AocError> {
    let (_, mut board) = parse_input(input).expect("Should be valid!");
    let start_coordinates = board
        .iter()
        .enumerate()
        .find_map(|(y, some_vec)| {
            some_vec.iter().enumerate().find_map(|(x, object)| {
                if object.object == Object::Start {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .expect("Should exist");
    let mut stack = VecDeque::new();
    stack.push_back((start_coordinates, 0_u32));
    let mut res = 0;
    while let Some(((y, x), curr_steps)) = stack.pop_front() {
        if curr_steps > steps + 1 {
            break;
        }
        if curr_steps % 2 == 0 {
            res += 1;
        }
        for (y_next, x_next) in Entry::possible_neighbours_coordinates((y, x), &mut board) {
            stack.push_back(((y_next, x_next), curr_steps + 1));
        }
    }

    Ok((res - 1).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = ".................................
.....###.#......###.#......###.#.
.###.##..#..###.##..#..###.##..#.
..#.#...#....#.#...#....#.#...#..
....#.#........#.#........#.#....
.##...####..##...####..##...####.
.##..#...#..##..#...#..##..#...#.
.......##.........##.........##..
.##.#.####..##.#.####..##.#.####.
.##..##.##..##..##.##..##..##.##.
.................................
.................................
.....###.#......###.#......###.#.
.###.##..#..###.##..#..###.##..#.
..#.#...#....#.#...#....#.#...#..
....#.#........#.#........#.#....
.##...####..##..S####..##...####.
.##..#...#..##..#...#..##..#...#.
.......##.........##.........##..
.##.#.####..##.#.####..##.#.####.
.##..##.##..##..##.##..##..##.##.
.................................
.................................
.....###.#......###.#......###.#.
.###.##..#..###.##..#..###.##..#.
..#.#...#....#.#...#....#.#...#..
....#.#........#.#........#.#....
.##...####..##...####..##...####.
.##..#...#..##..#...#..##..#...#.
.......##.........##.........##..
.##.#.####..##.#.####..##.#.####.
.##..##.##..##..##.##..##..##.##.
.................................";
        assert_eq!("50", process(input, 10)?);
        Ok(())
    }
}
