use std::collections::{HashMap, HashSet};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    multi::{fold_many1, separated_list1},
    IResult, Parser,
};

use crate::custom_error::AocError;

fn parse_components(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, components) = separated_list1(alt((tag(": "), tag(" "))), alpha1).parse(input)?;
    Ok((input, components))
}

fn parse_input(input: &str) -> IResult<&str, HashMap<String, HashSet<String>>> {
    let (input, map) = fold_many1(
        separated_list1(line_ending, parse_components),
        HashMap::new,
        |mut acc, components| {
            for component in components {
                for i in 1..component.len() {
                    acc.entry(component[0].to_owned())
                        .and_modify(|e: &mut HashSet<String>| {
                            e.insert(component[i].to_owned());
                        })
                        .or_insert({
                            let mut k = HashSet::new();
                            k.insert(component[i].to_owned());
                            k
                        });
                    acc.entry(component[i].to_owned())
                        .and_modify(|e: &mut HashSet<String>| {
                            e.insert(component[0].to_owned());
                        })
                        .or_insert({
                            let mut k = HashSet::new();
                            k.insert(component[0].to_owned());
                            k
                        });
                }
            }
            acc
        },
    )
    .parse(input)?;

    Ok((input, map))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, mut graph) = parse_input(input).expect("Should work");
    while graph.len() > 2 {
        let v = graph.keys().next().expect("Exists").clone();
        let (v, mut v_values) = graph.remove_entry(&v).expect("W entry should exist");
        let w = v_values.iter().next().expect("W value should exist").clone();
        let (w, w_values) = graph.remove_entry(&w).expect("V values should exist");
        let new_vw = format!("{}:{}", v, w);
        for set in graph.values_mut() {
            if set.remove(&v) {
                set.insert(new_vw.clone());
            }
            if set.remove(&w) {
                set.insert(new_vw.clone());
            }
        }
        v_values.extend(w_values);
        v_values.remove(&w);
        v_values.remove(&v);
        graph.insert(new_vw, v_values);
    }
    Ok(graph.values().map(|v| v.iter().next().expect("Should exist").split(":").count()).product::<usize>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
        assert_eq!("54", process(input)?);
        Ok(())
    }
}
