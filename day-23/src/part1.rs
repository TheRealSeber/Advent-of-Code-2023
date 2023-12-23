use nom::{
    branch::alt,
    character::complete::{self, line_ending},
    multi::{many1, separated_list1},
    IResult, Parser,
};

use crate::custom_error::AocError;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Entry {
    Path,
    Forest,
    EastSlope,
    SouthSlope,
    WesternSlope,
    NorthSlope,
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Entry>>> {
    let (input, map) = separated_list1(
        line_ending,
        many1(alt((
            complete::char('.').map(|_| Entry::Path),
            complete::char('#').map(|_| Entry::Forest),
            complete::char('^').map(|_| Entry::NorthSlope),
            complete::char('>').map(|_| Entry::EastSlope),
            complete::char('v').map(|_| Entry::SouthSlope),
            complete::char('<').map(|_| Entry::WesternSlope),
        ))),
    )
    .parse(input)?;

    Ok((input, map))
}

fn extract_neighbours((y, x): &(usize, usize), map: &Vec<Vec<Entry>>) -> Vec<(usize, usize)> {
    let mut neighbours: Vec<(usize, usize)> = Vec::new();
    if *y > 0 && map[*y - 1][*x] != Entry::Forest && map[*y - 1][*x] != Entry::SouthSlope {
        neighbours.push((*y - 1, *x));
    }
    if *y + 1 < map.len()
        && map[*y + 1][*x] != Entry::Forest
        && map[*y + 1][*x] != Entry::NorthSlope
    {
        neighbours.push((*y + 1, *x));
    }
    if *x > 1 && map[*y][*x - 1] != Entry::Forest && map[*y][*x - 1] != Entry::EastSlope {
        neighbours.push((*y, *x - 1));
    }
    if *x + 2 < map[0].len()
        && map[*y][*x + 1] != Entry::Forest
        && map[*y][*x + 1] != Entry::WesternSlope
    {
        neighbours.push((*y, *x + 1));
    }
    neighbours
}

fn dfs(
    result: &mut u32,
    mut current_moves: u32,
    mut map: Vec<Vec<Entry>>,
    (mut y, mut x): (usize, usize),
) {
    let mut neighbours = extract_neighbours(&(y, x), &map);
    while neighbours.len() > 0 {
        current_moves += 1;
        map[y][x] = Entry::Forest;
        if neighbours.len() == 1 {
            (y, x) = neighbours.pop().expect("Exists");
            neighbours = extract_neighbours(&(y, x), &map);
        } else {
            for (y_next, x_next) in neighbours {
                dfs(result, current_moves, map.clone(), (y_next, x_next))
            }
            return;
        }
    }
    *result = (*result).max(current_moves);
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, map) = parse_input(input).expect("Should be valid");
    let mut res = 0;
    dfs(&mut res, 0, map, (0, 1));

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
        assert_eq!("94", process(input)?);
        Ok(())
    }
}
