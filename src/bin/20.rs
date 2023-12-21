advent_of_code::solution!(20);

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, newline},
    combinator::opt,
    error::Error,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    Finish, IResult,
};
use std::collections::HashMap;
use std::str::FromStr;

type Count = usize;
type Name = String;

fn parse_name(input: &str) -> IResult<&str, Name> {
    let (i, name) = alpha1(input)?;
    Ok((i, name.to_string()))
}

type Names = Vec<Name>;

fn parse_names(input: &str) -> IResult<&str, Names> {
    let (i, names) = separated_list1(tag(", "), parse_name)(input)?;
    Ok((i, names))
}

#[derive(Copy, Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
enum State {
    #[default]
    Off,
    On,
}

#[derive(Copy, Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
enum Pulse {
    #[default]
    Low,
    High,
}

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
enum Category {
    FlipFlop { state: State },
    Conjunction,
    Broadcast,
}

fn parse_category(input: &str) -> IResult<&str, Category> {
    let (i, maybe_prefix) = opt(alt((char('%'), char('&'))))(input)?;
    let category = match maybe_prefix {
        Some('%') => Category::FlipFlop { state: State::Off },
        Some('&') => Category::Conjunction {},
        None => Category::Broadcast,
        _ => panic!(),
    };
    Ok((i, category))
}

#[derive(Debug)]
struct Module {
    category: Category,
    name: Name,
    destination_module_names: Names,
}

fn parse_module(input: &str) -> IResult<&str, Module> {
    let (i, ((category, name), destination_module_names)) = separated_pair(
        tuple((parse_category, parse_name)),
        tag(" -> "),
        parse_names,
    )(input)?;
    Ok((
        i,
        Module {
            category,
            name,
            destination_module_names,
        },
    ))
}

type Modules = Vec<Module>;

fn parse_modules(input: &str) -> IResult<&str, Modules> {
    let (i, modules) = separated_list1(newline, parse_module)(input)?;
    Ok((i, modules))
}

type ModuleForName = HashMap<Name, Module>;

#[derive(Debug)]
struct Network {
    module_for_name: ModuleForName,
}

fn parse_network(input: &str) -> IResult<&str, Network> {
    let (i, modules) = parse_modules(input)?;
    Ok((
        i,
        Network {
            module_for_name: modules
                .into_iter()
                .map(|module| (module.name.clone(), module))
                .collect(),
        },
    ))
}

impl FromStr for Network {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_network(s).finish() {
            Ok((_, network)) => Ok(network),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

impl Network {
    fn trigger_and_count_pulses(&self) -> (Count, Count) {
        (0, 0)
    }
}

pub fn part_one(input: &str) -> Option<Count> {
    let network = Network::from_str(input).ok()?;
    println!("{:?}", network);

    let (low_pulse_count, high_pulse_count) = network.trigger_and_count_pulses();

    Some(low_pulse_count * high_pulse_count)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(32000000));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
