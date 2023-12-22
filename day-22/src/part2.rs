use std::collections::{HashMap, HashSet};

use nom::{
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult, Parser,
};

use crate::custom_error::AocError;

#[derive(Debug)]
struct Brick {
    x_range: (u32, u32),
    y_range: (u32, u32),
    z_range: (u32, u32),
}

fn parse_input(input: &str) -> IResult<&str, Vec<Brick>> {
    let (input, bricks) = separated_list1(
        line_ending,
        separated_pair(
            tuple((
                complete::u32,
                preceded(complete::char(','), complete::u32),
                preceded(complete::char(','), complete::u32),
            )),
            complete::char('~'),
            tuple((
                complete::u32,
                preceded(complete::char(','), complete::u32),
                preceded(complete::char(','), complete::u32),
            )),
        )
        .map(|((x_min, y_min, z_min), (x_max, y_max, z_max))| Brick {
            x_range: (x_min, x_max),
            y_range: (y_min, y_max),
            z_range: (z_min - 1, z_max - 1),
        }),
    )
    .parse(input)?;

    Ok((input, bricks))
}

fn get_supporting_bricks_and_fallen_ammount_of_z(
    brick: &Brick,
    space: &Vec<Vec<Vec<u32>>>,
    fallen_by: &mut u32,
) -> HashSet<u32> {
    let mut supported_ids = HashSet::new();
    if brick.z_range.0 <= *fallen_by {
        return supported_ids;
    }
    for y in brick.y_range.0..brick.y_range.1 + 1 {
        for x in brick.x_range.0..brick.x_range.1 + 1 {
            if space[(brick.z_range.0 as usize - *fallen_by as usize) - 1][y as usize][x as usize]
                != 0
            {
                supported_ids.insert(
                    space[(brick.z_range.0 as usize - *fallen_by as usize) - 1][y as usize]
                        [x as usize],
                );
            }
        }
    }
    if !supported_ids.is_empty() {
        return supported_ids;
    }
    *fallen_by += 1;
    get_supporting_bricks_and_fallen_ammount_of_z(brick, space, fallen_by)
}

fn how_much_fails(
    mut fallen_brick_ids: Vec<u32>,
    supporting_block: &HashMap<u32, Vec<u32>>,
) -> u32 {
    let mut supporting_block = supporting_block.clone();
    let mut res = 0;
    loop {
        for prev_brick_id in fallen_brick_ids {
            supporting_block
                .values_mut()
                .for_each(|v| v.retain(|k| *k != prev_brick_id))
        }
        fallen_brick_ids = Vec::new();
        supporting_block.retain(|k, v| {
            if v.is_empty() {
                fallen_brick_ids.push(*k);
                false
            } else {
                true
            }
        });
        res += fallen_brick_ids.len() as u32;
        if fallen_brick_ids.is_empty() {
            break;
        }
    }
    res
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, mut bricks) = parse_input(input).expect("Should be valid");
    let bricks_ammount = bricks.len();
    bricks.sort_unstable_by(|a, b| a.z_range.cmp(&b.z_range));
    let mut supporting: HashMap<u32, HashSet<u32>> = HashMap::new();
    let (x_max, y_max, z_max) = bricks.iter().fold((0, 0, 0), |(x, y, z), k| {
        (x.max(k.x_range.1), y.max(k.y_range.1), z.max(k.z_range.1))
    });
    let mut space = vec![vec![vec![0; x_max as usize + 1]; y_max as usize + 1]; z_max as usize + 1];
    for (id, brick) in bricks.into_iter().enumerate() {
        let mut fallen_by = 0;
        let supported_by =
            get_supporting_bricks_and_fallen_ammount_of_z(&brick, &space, &mut fallen_by);
        for brick in supported_by {
            supporting
                .entry(id as u32 + 1)
                .and_modify(|k| {
                    k.insert(brick);
                })
                .or_insert({
                    let mut new_hash = HashSet::new();
                    new_hash.insert(brick);
                    new_hash
                });
        }
        for z in brick.z_range.0 - fallen_by..brick.z_range.1 - fallen_by + 1 {
            for y in brick.y_range.0..brick.y_range.1 + 1 {
                for x in brick.x_range.0..brick.x_range.1 + 1 {
                    space[z as usize][y as usize][x as usize] = id as u32 + 1;
                }
            }
        }
    }
    let supporting = supporting
        .into_iter()
        .map(|(id, value)| (id, value.into_iter().collect::<Vec<u32>>()))
        .collect::<HashMap<u32, Vec<u32>>>();
    let mut res = 0;
    for i in 1..bricks_ammount + 1 {
        res += how_much_fails(vec![i as u32], &supporting);
    }
    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        assert_eq!("7", process(input)?);
        Ok(())
    }
}
