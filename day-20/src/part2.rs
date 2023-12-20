use std::collections::{HashMap, VecDeque};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    multi::separated_list1,
    sequence::{pair, separated_pair},
    IResult, Parser,
};

use crate::custom_error::AocError;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Signal {
    Low,
    High,
}

#[derive(Debug, Clone)]
enum Module<'a> {
    FlipFlop { activated: bool },
    Conjunction { inputs: HashMap<&'a str, Signal> },
    Broadcaster,
}

#[derive(Debug)]
struct SignalCounter {
    low: u64,
    high: u64,
}

impl SignalCounter {
    fn add(&mut self, signal: &Signal) {
        match signal {
            Signal::Low => self.low += 1,
            Signal::High => self.high += 1,
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, HashMap<&str, (Module, Vec<&str>)>> {
    let mut hashed_input: HashMap<&str, (Module, Vec<&str>)> = HashMap::new();
    let (input, modules) = separated_list1(
        line_ending,
        separated_pair(
            alt((
                alpha1.map(|name| (name, Module::Broadcaster)),
                pair(complete::char('%'), alpha1)
                    .map(|(_, name)| (name, Module::FlipFlop { activated: false })),
                pair(complete::char('&'), alpha1).map(|(_, name)| {
                    (
                        name,
                        Module::Conjunction {
                            inputs: HashMap::new(),
                        },
                    )
                }),
            )),
            tag(" -> "),
            separated_list1(tag(", "), alpha1),
        ),
    )
    .parse(input)?;
    modules
        .into_iter()
        .for_each(|((name, module), destinations)| {
            hashed_input.insert(name, (module, destinations));
        });
    hashed_input
        .clone()
        .into_iter()
        .for_each(|(module_name, (_, destinations))| {
            destinations.iter().for_each(|dest| {
                if let Some((Module::Conjunction { inputs }, _)) = hashed_input.get_mut(dest) {
                    inputs.insert(module_name, Signal::Low);
                }
            })
        });
    Ok((input, hashed_input))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, mut modules_map) = parse_input(input).expect("Should be valid");
    let mut signal_counter = SignalCounter { low: 0, high: 0 };
    for _ in 0..1000 {
        let mut queue = VecDeque::new();
        queue.push_back(("start", Signal::Low, "broadcaster"));
        while let Some((sender, signal, module_name)) = queue.pop_front() {
            signal_counter.add(&signal);
            if let Some((module_type, destinations)) = modules_map.get_mut(module_name) {
                let mut signal_to_send = Signal::Low;
                match module_type {
                    Module::FlipFlop { activated } => {
                        if signal == Signal::Low {
                            *activated = !*activated;
                            if *activated {
                                signal_to_send = Signal::High;
                            };
                            destinations.iter().for_each(|dest| {
                                queue.push_back((module_name, signal_to_send, dest));
                            })
                        }
                    }
                    Module::Conjunction { inputs } => {
                        inputs.entry(sender).and_modify(|sig| *sig = signal);
                        if inputs.iter().any(|(_, &signaled)| signaled == Signal::Low) {
                            signal_to_send = Signal::High;
                        };
                        destinations.iter().for_each(|dest| {
                            queue.push_back((module_name, signal_to_send, dest));
                        })
                    }
                    Module::Broadcaster => destinations.iter().for_each(|dest| {
                        queue.push_back((module_name, signal_to_send, dest));
                    }),
                }
            }
        }
    }
    Ok((signal_counter.low * signal_counter.high).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() -> miette::Result<()> {
        let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        assert_eq!("32000000", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_2() -> miette::Result<()> {
        let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
        assert_eq!("11687500", process(input)?);
        Ok(())
    }
}
