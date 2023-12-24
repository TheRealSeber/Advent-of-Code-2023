use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult, Parser,
};

use crate::custom_error::AocError;

#[derive(Debug)]
#[warn(dead_code)]
struct Hailstone {
    px: i64,
    py: i64,
    pz: i64,
    vx: i64,
    vy: i64,
    vz: i64,
}

impl Hailstone {
    fn new(((px, py, pz), (vx, vy, vz)): ((i64, i64, i64), (i64, i64, i64))) -> Self {
        Self {
            px,
            py,
            pz,
            vx,
            vy,
            vz,
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Hailstone>> {
    let (input, hailstones) = separated_list1(
        line_ending,
        separated_pair(
            tuple((
                complete::i64,
                preceded(tag(", "), complete::i64),
                preceded(tag(", "), complete::i64),
            )),
            tag(" @ "),
            tuple((
                complete::i64,
                preceded(tag(", "), complete::i64),
                preceded(tag(", "), complete::i64),
            )),
        )
        .map(|(position, velocity)| Hailstone::new((position, velocity))),
    )
    .parse(input)?;

    Ok((input, hailstones))
}

// https://paulbourke.net/geometry/pointlineplane/
fn calculate_intersection(
    (p1, p2, p3, p4): ((i64, i64), (i64, i64), (i64, i64), (i64, i64)),
) -> Option<(f64, f64)> {
    if (p1.0 == p2.0 && p1.1 == p2.1) || (p3.0 == p4.0 && p3.1 == p4.1) {
        return None;
    }

    let denominator = ((p4.1 - p3.1) * (p2.0 - p1.0) - (p4.0 - p3.0) * (p2.1 - p1.1)) as f64;

    if denominator == 0.0 {
        return None;
    }

    let ua = ((p4.0 - p3.0) * (p1.1 - p3.1) - (p4.1 - p3.1) * (p1.0 - p3.0)) as f64 / denominator;

    Some((
        p1.0 as f64 + ua * (p2.0 - p1.0) as f64,
        p1.1 as f64 + ua * (p2.1 - p1.1) as f64,
    ))
}

#[tracing::instrument]
pub fn process(input: &str, (left, right): (f64, f64)) -> miette::Result<String, AocError> {
    let (_, hailstones) = parse_input(input).expect("Should be valid");
    let mut res = 0;
    for i in 0..hailstones.len() {
        for j in i..hailstones.len() {
            let (hailstone_a, hailstone_b) = (&hailstones[i], &hailstones[j]);
            let point_1 = (hailstone_a.px, hailstone_a.py);
            let point_2 = (
                hailstone_a.px + hailstone_a.vx,
                hailstone_a.py + hailstone_a.vy,
            );
            let point_3 = (hailstone_b.px, hailstone_b.py);
            let point_4 = (
                hailstone_b.px + hailstone_b.vx,
                hailstone_b.py + hailstone_b.vy,
            );
            if let Some((x_inct, y_inct)) =
                calculate_intersection((point_1, point_2, point_3, point_4))
            {
                if !(hailstone_a.vx.is_negative() && x_inct > hailstone_a.px as f64)
                    && !(hailstone_a.vx.is_positive() && x_inct < hailstone_a.px as f64)
                    && !(hailstone_b.vx.is_negative() && x_inct > hailstone_b.px as f64)
                    && !(hailstone_b.vx.is_positive() && x_inct < hailstone_b.px as f64)
                    && x_inct >= left
                    && x_inct <= right
                    && y_inct >= left
                    && y_inct <= right
                {
                    res += 1;
                }
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
        let input = "19, 13, 30 @ -2, 1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @ 1, -5, -3";
        assert_eq!("2", process(input, (7.0, 27.0))?);
        Ok(())
    }
}
