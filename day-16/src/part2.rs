use std::collections::VecDeque;

use nom::{
    bytes::complete::is_a, character::complete::line_ending, multi::separated_list1, IResult,
};

use crate::custom_error::AocError;

#[derive(Debug)]
enum MoveDirection {
    SplitVertical,
    SplitHorizontal,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
enum Character {
    VerticalBar,
    HorizontalBar,
    BackSlash,
    ForwardSlash,
    Dot,
}

#[derive(Debug, Clone)]
struct Entry {
    passed_from_down: bool,
    passed_from_up: bool,
    passed_from_left: bool,
    passed_from_right: bool,
    splittable: Option<bool>,
    charachter: Character,
}

impl Entry {
    fn new(ch: char) -> Self {
        match ch {
            '|' => Self {
                passed_from_down: false,
                passed_from_up: false,
                passed_from_left: false,
                passed_from_right: false,
                splittable: Some(false),
                charachter: Character::VerticalBar,
            },
            '-' => Self {
                passed_from_down: false,
                passed_from_up: false,
                passed_from_left: false,
                passed_from_right: false,
                splittable: Some(false),
                charachter: Character::HorizontalBar,
            },
            '\\' => Self {
                passed_from_down: false,
                passed_from_up: false,
                passed_from_left: false,
                passed_from_right: false,
                splittable: None,
                charachter: Character::BackSlash,
            },
            '/' => Self {
                passed_from_down: false,
                passed_from_up: false,
                passed_from_left: false,
                passed_from_right: false,
                splittable: None,
                charachter: Character::ForwardSlash,
            },
            _ => Self {
                passed_from_down: false,
                passed_from_up: false,
                passed_from_left: false,
                passed_from_right: false,
                splittable: None,
                charachter: Character::Dot,
            },
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Entry>>> {
    let (input, lines) = separated_list1(line_ending, is_a(r#"./\|-"#))(input)?;
    let mapped_lines = lines
        .into_iter()
        .map(|k| k.chars().map(|ch| Entry::new(ch)).collect())
        .collect::<Vec<Vec<Entry>>>();
    Ok((input, mapped_lines))
}

fn solve(
    mut mapped_input: &mut Vec<Vec<Entry>>,
    (y, x): (usize, usize),
    start_direction: MoveDirection,
) -> u32 {
    let mut queue: VecDeque<((usize, usize), MoveDirection)> = VecDeque::new();
    match &start_direction {
        MoveDirection::Up => mapped_input[y][x].passed_from_down = true,
        MoveDirection::Down => mapped_input[y][x].passed_from_up = true,
        MoveDirection::Left => mapped_input[y][x].passed_from_right = true,
        MoveDirection::Right => mapped_input[y][x].passed_from_left = true,
        _ => {}
    }
    match mapped_input[y][x].charachter {
        Character::VerticalBar => match start_direction {
            MoveDirection::Up | MoveDirection::Down => queue.push_back(((y, x), start_direction)),
            MoveDirection::Left | MoveDirection::Right => {
                if y > 0 {
                    queue.push_back(((y - 1, x), MoveDirection::Up))
                }
                if y + 1 < mapped_input.len() {
                    queue.push_back(((y + 1, x), MoveDirection::Down))
                }
            }
            _ => {}
        },
        Character::HorizontalBar => match start_direction {
            MoveDirection::Left | MoveDirection::Right => {
                queue.push_back(((y, x), start_direction))
            }
            MoveDirection::Up | MoveDirection::Down => {
                if x > 0 {
                    queue.push_back(((y, x - 1), MoveDirection::Left))
                }
                if x + 1 < mapped_input[0].len() {
                    queue.push_back(((y, x + 1), MoveDirection::Right))
                }
            }
            _ => {}
        },
        Character::BackSlash => match start_direction {
            MoveDirection::Up => {
                if x > 0 {
                    queue.push_back(((y, x - 1), MoveDirection::Left))
                }
            }
            MoveDirection::Down => {
                if x + 1 < mapped_input[0].len() {
                    queue.push_back(((y, x + 1), MoveDirection::Right))
                }
            }
            MoveDirection::Left => {
                if y + 1 < mapped_input.len() {
                    queue.push_back(((y + 1, x), MoveDirection::Down))
                }
            }
            MoveDirection::Right => {
                if y > 0 {
                    queue.push_back(((y - 1, x), MoveDirection::Up))
                }
            }
            _ => {}
        },
        Character::ForwardSlash => match start_direction {
            MoveDirection::Down => {
                if x > 0 {
                    queue.push_back(((y, x - 1), MoveDirection::Left))
                }
            }
            MoveDirection::Up => {
                if x + 1 < mapped_input[0].len() {
                    queue.push_back(((y, x + 1), MoveDirection::Right))
                }
            }
            MoveDirection::Right => {
                if y + 1 < mapped_input.len() {
                    queue.push_back(((y + 1, x), MoveDirection::Down))
                }
            }
            MoveDirection::Left => {
                if y > 0 {
                    queue.push_back(((y - 1, x), MoveDirection::Up))
                }
            }
            _ => {}
        },
        Character::Dot => queue.push_back(((y, x), start_direction)),
    }
    while let Some(((y, x), move_direction)) = queue.pop_front() {
        match move_direction {
            MoveDirection::Up
                if y > 0
                    && (!mapped_input[y - 1][x].passed_from_down
                        || mapped_input[y - 1][x].splittable.is_some_and(|v| !v)) =>
            {
                match mapped_input[y - 1][x].charachter {
                    Character::Dot | Character::VerticalBar => {
                        queue.push_back(((y - 1, x), MoveDirection::Up));
                    }
                    Character::BackSlash => {
                        queue.push_back(((y - 1, x), MoveDirection::Left));
                    }
                    Character::ForwardSlash => {
                        queue.push_back(((y - 1, x), MoveDirection::Right));
                    }
                    Character::HorizontalBar => {
                        queue.push_back(((y - 1, x), MoveDirection::SplitHorizontal));
                        mapped_input[y - 1][x].splittable = Some(true);
                    }
                }
                mapped_input[y - 1][x].passed_from_down = true;
            }
            MoveDirection::Down
                if y + 1 < mapped_input.len()
                    && (!mapped_input[y + 1][x].passed_from_up
                        || mapped_input[y + 1][x].splittable.is_some_and(|v| !v)) =>
            {
                match mapped_input[y + 1][x].charachter {
                    Character::Dot | Character::VerticalBar => {
                        queue.push_back(((y + 1, x), MoveDirection::Down));
                    }
                    Character::BackSlash => {
                        queue.push_back(((y + 1, x), MoveDirection::Right));
                    }
                    Character::ForwardSlash => {
                        queue.push_back(((y + 1, x), MoveDirection::Left));
                    }
                    Character::HorizontalBar => {
                        queue.push_back(((y + 1, x), MoveDirection::SplitHorizontal));
                        mapped_input[y + 1][x].splittable = Some(true);
                    }
                }
                mapped_input[y + 1][x].passed_from_up = true;
            }
            MoveDirection::Left
                if x > 0
                    && (!mapped_input[y][x - 1].passed_from_right
                        || mapped_input[y][x - 1].splittable.is_some_and(|v| !v)) =>
            {
                match mapped_input[y][x - 1].charachter {
                    Character::Dot | Character::HorizontalBar => {
                        queue.push_back(((y, x - 1), MoveDirection::Left));
                    }
                    Character::BackSlash => {
                        queue.push_back(((y, x - 1), MoveDirection::Up));
                    }
                    Character::ForwardSlash => {
                        queue.push_back(((y, x - 1), MoveDirection::Down));
                    }
                    Character::VerticalBar => {
                        queue.push_back(((y, x - 1), MoveDirection::SplitVertical));
                        mapped_input[y][x - 1].splittable = Some(true);
                    }
                }
                mapped_input[y][x - 1].passed_from_right = true;
            }
            MoveDirection::Right
                if x + 1 < mapped_input[0].len()
                    && (!mapped_input[y][x + 1].passed_from_left
                        || mapped_input[y][x + 1].splittable.is_some_and(|v| !v)) =>
            {
                match mapped_input[y][x + 1].charachter {
                    Character::Dot | Character::HorizontalBar => {
                        queue.push_back(((y, x + 1), MoveDirection::Right));
                    }
                    Character::BackSlash => {
                        queue.push_back(((y, x + 1), MoveDirection::Down));
                    }
                    Character::ForwardSlash => {
                        queue.push_back(((y, x + 1), MoveDirection::Up));
                    }
                    Character::VerticalBar => {
                        queue.push_back(((y, x + 1), MoveDirection::SplitVertical));
                        mapped_input[y][x + 1].splittable = Some(true);
                    }
                }
                mapped_input[y][x + 1].passed_from_left = true;
            }
            MoveDirection::SplitVertical => {
                if y > 0
                    && (!mapped_input[y - 1][x].passed_from_down
                        || mapped_input[y - 1][x].splittable.is_some_and(|v| !v))
                {
                    match mapped_input[y - 1][x].charachter {
                        Character::Dot | Character::VerticalBar => {
                            queue.push_back(((y - 1, x), MoveDirection::Up));
                        }
                        Character::BackSlash => {
                            queue.push_back(((y - 1, x), MoveDirection::Left));
                        }
                        Character::ForwardSlash => {
                            queue.push_back(((y - 1, x), MoveDirection::Right));
                        }
                        Character::HorizontalBar => {
                            queue.push_back(((y - 1, x), MoveDirection::SplitHorizontal));
                            mapped_input[y - 1][x].splittable = Some(true);
                        }
                    }
                    mapped_input[y - 1][x].passed_from_down = true;
                }
                if y + 1 < mapped_input.len()
                    && (!mapped_input[y + 1][x].passed_from_up
                        || mapped_input[y + 1][x].splittable.is_some_and(|v| !v))
                {
                    match mapped_input[y + 1][x].charachter {
                        Character::Dot | Character::VerticalBar => {
                            queue.push_back(((y + 1, x), MoveDirection::Down));
                        }
                        Character::BackSlash => {
                            queue.push_back(((y + 1, x), MoveDirection::Right));
                        }
                        Character::ForwardSlash => {
                            queue.push_back(((y + 1, x), MoveDirection::Left));
                        }
                        Character::HorizontalBar => {
                            queue.push_back(((y + 1, x), MoveDirection::SplitHorizontal));
                            mapped_input[y + 1][x].splittable = Some(true);
                        }
                    }
                    mapped_input[y + 1][x].passed_from_up = true;
                }
            }
            MoveDirection::SplitHorizontal => {
                if x > 0
                    && (!mapped_input[y][x - 1].passed_from_right
                        || mapped_input[y][x - 1].splittable.is_some_and(|v| !v))
                {
                    match mapped_input[y][x - 1].charachter {
                        Character::Dot | Character::HorizontalBar => {
                            queue.push_back(((y, x - 1), MoveDirection::Left));
                        }
                        Character::BackSlash => {
                            queue.push_back(((y, x - 1), MoveDirection::Up));
                        }
                        Character::ForwardSlash => {
                            queue.push_back(((y, x - 1), MoveDirection::Down));
                        }
                        Character::VerticalBar => {
                            queue.push_back(((y, x - 1), MoveDirection::SplitVertical));
                            mapped_input[y][x - 1].splittable = Some(true);
                        }
                    }
                    mapped_input[y][x - 1].passed_from_right = true;
                }
                if x + 1 < mapped_input[0].len()
                    && (!mapped_input[y][x + 1].passed_from_left
                        || mapped_input[y][x + 1].splittable.is_some_and(|v| !v))
                {
                    match mapped_input[y][x + 1].charachter {
                        Character::Dot | Character::HorizontalBar => {
                            queue.push_back(((y, x + 1), MoveDirection::Right));
                        }
                        Character::BackSlash => {
                            queue.push_back(((y, x + 1), MoveDirection::Down));
                        }
                        Character::ForwardSlash => {
                            queue.push_back(((y, x + 1), MoveDirection::Up));
                        }
                        Character::VerticalBar => {
                            queue.push_back(((y, x + 1), MoveDirection::SplitVertical));
                            mapped_input[y][x + 1].splittable = Some(true);
                        }
                    }
                    mapped_input[y][x + 1].passed_from_left = true;
                }
            }
            _ => {}
        }
    }
    let res = mapped_input.into_iter().fold(0_u32, |acc, row| {
        acc + row.into_iter().fold(0_u32, |acc_row, k| {
            if k.passed_from_down || k.passed_from_left || k.passed_from_right || k.passed_from_up {
                acc_row + 1
            } else {
                acc_row
            }
        })
    });
    restart_map(&mut mapped_input);
    res
}

fn restart_map(map: &mut Vec<Vec<Entry>>) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            map[y][x].passed_from_down = false;
            map[y][x].passed_from_left = false;
            map[y][x].passed_from_right = false;
            map[y][x].passed_from_up = false;
        }
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, mut mapped_input) = parse_input(input).expect("Shoud be valid");
    let (y_max, x_max) = (mapped_input.len() - 1, mapped_input[0].len() - 1);
    let mut res = 0;
    for y in 0..mapped_input.len() {
        res = res
            .max(solve(&mut mapped_input, (y, 0), MoveDirection::Right))
            .max(solve(&mut mapped_input, (y, x_max), MoveDirection::Left))
    }
    for x in 0..mapped_input[0].len() {
        res = res
            .max(solve(&mut mapped_input, (0, x), MoveDirection::Down))
            .max(solve(&mut mapped_input, (y_max, x), MoveDirection::Up))
    }
    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
        assert_eq!("51", process(input)?);
        Ok(())
    }
}
