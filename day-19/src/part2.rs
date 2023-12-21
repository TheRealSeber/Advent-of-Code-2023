use std::collections::HashMap;

use crate::custom_error::AocError;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{self, alpha1, line_ending};
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded};
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
#[derive(Copy, Clone, Debug)]
struct RatingRanges {
    x: (u64, u64),
    m: (u64, u64),
    a: (u64, u64),
    s: (u64, u64),
}

impl RatingRanges {
    fn sum(&self) -> u64 {
        (self.x.1 - self.x.0 + 1)
            * (self.m.1 - self.m.0 + 1)
            * (self.a.1 - self.a.0 + 1)
            * (self.s.1 - self.s.0 + 1)
    }

    fn adjust_high_range(&mut self, part: &Part, value: u64) {
        match part {
            Part::Xtremly => self.x.1 = value,
            Part::Musical => self.m.1 = value,
            Part::Aerodynamic => self.a.1 = value,
            Part::Shiny => self.s.1 = value,
        }
    }

    fn adjust_low_range(&mut self, part: &Part, value: u64) {
        match part {
            Part::Xtremly => self.x.0 = value,
            Part::Musical => self.m.0 = value,
            Part::Aerodynamic => self.a.0 = value,
            Part::Shiny => self.s.0 = value,
        }
    }
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

fn parse_input(input: &str) -> IResult<&str, HashMap<&str, Vec<Rule>>> {
    let (_, workflows_input) = alt((take_until("\r\n\r\n"), take_until("\n\n"))).parse(input)?;

    let (input, workflows) = parse_workflows(workflows_input)?;
    Ok((input, workflows))
}

fn get_ranges<'a>(
    res: &mut u64,
    mut range: RatingRanges,
    map: &'a HashMap<&str, Vec<Rule<'a>>>,
    current_place: Destination,
) {
    if let Destination::Workflow(next_destination) = current_place {
        let rules = map.get(next_destination).expect("Shuld exist");
        rules.iter().for_each(|rule| match rule {
            Rule::Test {
                part,
                target,
                condition,
                value,
            } => {
                let mut new_rating_range = range;
                match condition {
                    Condition::Greater => {
                        new_rating_range.adjust_low_range(part, *value as u64 + 1);
                        range.adjust_high_range(part, *value as u64);
                        get_ranges(res, new_rating_range, map, *target);
                    }
                    Condition::Lower => {
                        new_rating_range.adjust_high_range(part, *value as u64 - 1);
                        range.adjust_low_range(part, *value as u64);
                        get_ranges(res, new_rating_range, map, *target);
                    }
                }
            }
            Rule::Target(destination) => get_ranges(res, range, map, *destination),
        });
    } else if current_place == Destination::Accepted {
        *res += range.sum();
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, workflows) = parse_input(input).expect("Should be valid");
    let ranges = RatingRanges {
        x: (1, 4000),
        m: (1, 4000),
        a: (1, 4000),
        s: (1, 4000),
    };
    let mut res = 0_u64;
    get_ranges(&mut res, ranges, &workflows, Destination::Workflow("in"));
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
        assert_eq!("167409079868000", process(input)?);
        Ok(())
    }
}
