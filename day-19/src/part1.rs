use std::collections::HashMap;

use crate::custom_error::AocError;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{self, alpha1, anychar, line_ending};
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, preceded, separated_pair};
use nom::{IResult, Parser};

#[derive(PartialEq, Eq, Hash, Debug)]
enum Part {
    Xtremly,
    Musical,
    Aerodynamic,
    Shiny,
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
enum Destination<'a> {
    Workflow(&'a str),
    Accepted,
    Rejected,
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Condition {
    Greater,
    Lower,
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Rule<'a> {
    Test {
        part: Part,
        target: Destination<'a>,
        condition: Condition,
        value: u32,
    },
    Target(Destination<'a>),
}

#[derive(Debug)]
struct Rating<'a> {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
    current_place: Destination<'a>,
}

impl<'a> Rating<'a> {
    fn from_vec(input: Vec<u32>) -> Self {
        Rating {
            x: input[0],
            m: input[1],
            a: input[2],
            s: input[3],
            current_place: Destination::Workflow("in"),
        }
    }

    fn get_part_value(&self, part: &Part) -> &u32 {
        match part {
            Part::Xtremly => &self.x,
            Part::Musical => &self.m,
            Part::Aerodynamic => &self.a,
            Part::Shiny => &self.s,
        }
    }
}

fn parse_rating(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, rating) = delimited(
        complete::char('{'),
        separated_list1(
            complete::char(','),
            preceded(pair(anychar, complete::char('=')), complete::u32),
        ),
        complete::char('}'),
    )
    .parse(input)?;

    Ok((input, rating))
}

fn parse_ratings(input: &str) -> IResult<&str, Vec<Rating>> {
    let (input, ratings) = separated_list1(line_ending, parse_rating)
        .map(|ratings| ratings.into_iter().map(Rating::from_vec).collect())
        .parse(input)?;

    Ok((input, ratings))
}

fn parse_destination(input: &str) -> IResult<&str, Rule> {
    let (input, destination) = alt((
        tag("A").map(|_| Destination::Accepted),
        tag("R").map(|_| Destination::Rejected),
        alpha1.map(Destination::Workflow),
    ))
    .parse(input)?;

    Ok((input, Rule::Target(destination)))
}

fn parse_rule_test(input: &str) -> IResult<&str, Rule> {
    let (input, part) = alt((
        complete::char('a').map(|_| Part::Aerodynamic),
        complete::char('x').map(|_| Part::Xtremly),
        complete::char('s').map(|_| Part::Shiny),
        complete::char('m').map(|_| Part::Musical),
    ))
    .parse(input)?;
    let (input, condition) = alt((
        complete::char('>').map(|_| Condition::Greater),
        complete::char('<').map(|_| Condition::Lower),
    ))
    .parse(input)?;
    let (input, value) = complete::u32(input)?;
    let (input, target) = preceded(
        complete::char(':'),
        alt((
            tag("A").map(|_| Destination::Accepted),
            tag("R").map(|_| Destination::Rejected),
            alpha1.map(Destination::Workflow),
        )),
    )
    .parse(input)?;

    Ok((
        input,
        Rule::Test {
            part,
            condition,
            target,
            value,
        },
    ))
}

fn parse_workflow(input: &str) -> IResult<&str, (&str, Vec<Rule>)> {
    let (input, id) = alpha1(input)?;
    let (input, rules) = delimited(
        complete::char('{'),
        separated_list1(
            complete::char(','),
            alt((parse_rule_test, parse_destination)),
        ),
        complete::char('}'),
    )
    .parse(input)?;
    Ok((input, (id, rules)))
}

fn parse_workflows(input: &str) -> IResult<&str, HashMap<&str, Vec<Rule<'_>>>> {
    let mut workflows_map: HashMap<&str, Vec<Rule<'_>>> = HashMap::new();
    let (input, workflows) = separated_list1(line_ending, parse_workflow).parse(input)?;
    workflows.into_iter().for_each(|(id, workflow)| {
        workflows_map.insert(id, workflow);
    });
    Ok((input, workflows_map))
}

fn parse_input(input: &str) -> IResult<&str, (HashMap<&str, Vec<Rule>>, Vec<Rating>)> {
    let (input, (workflows, ratings)) = separated_pair(
        parse_workflows,
        pair(line_ending, line_ending),
        parse_ratings,
    )
    .parse(input)?;
    Ok((input, (workflows, ratings)))
}

fn check_acceptance<'a>(rating: &'a Rating<'a>, map: &'a HashMap<&str, Vec<Rule<'a>>>) -> bool {
    let mut current_place = rating.current_place;
    while let Destination::Workflow(next_dest) = current_place {
        let rules = map.get(next_dest).expect("Shuld exist");
        for rule in rules {
            match rule {
                Rule::Test {
                    part,
                    target,
                    condition,
                    value,
                } => {
                    let rating_part_value = rating.get_part_value(part);
                    match condition {
                        Condition::Greater => {
                            if rating_part_value > value {
                                current_place = *target;
                                break;
                            }
                        }
                        Condition::Lower => {
                            if rating_part_value < value {
                                current_place = *target;
                                break;
                            }
                        }
                    }
                }
                Rule::Target(destination) => current_place = *destination,
            }
        }
    }
    current_place == Destination::Accepted
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, (workflows, input)) = parse_input(input).expect("Should be valid");
    let res = input.into_iter().fold(0_u32, |acc, rating| {
        if check_acceptance(&rating, &workflows) {
            acc + rating.x + rating.m + rating.a + rating.s
        } else {
            acc
        }
    });
    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        assert_eq!("19114", process(input)?);
        Ok(())
    }
}
