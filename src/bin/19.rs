advent_of_code::solution!(19);

use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{alpha1, char, digit1, newline},
    combinator::{map_res, opt},
    error::Error,
    multi::{count, separated_list1},
    sequence::{delimited, separated_pair, tuple},
    Finish, IResult,
};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
enum Category {
    ExtremelyCool,
    Musical,
    Aerodynamic,
    Shiny,
}

fn parse_category(input: &str) -> IResult<&str, Category> {
    // < or >
    let (i, c) = alt((char('x'), char('m'), char('a'), char('s')))(input)?;
    let category = match c {
        'x' => Category::ExtremelyCool,
        'm' => Category::Musical,
        'a' => Category::Aerodynamic,
        's' => Category::Shiny,
        _ => panic!(),
    };
    Ok((i, category))
}

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
enum Operator {
    LessThan,
    GreaterThan,
}

fn parse_operator(input: &str) -> IResult<&str, Operator> {
    // < or >
    let (i, c) = alt((char('<'), char('>')))(input)?;
    let operator = match c {
        '<' => Operator::LessThan,
        '>' => Operator::GreaterThan,
        _ => panic!(),
    };
    Ok((i, operator))
}

type Rating = usize;

fn parse_rating(input: &str) -> IResult<&str, Rating> {
    let (i, rating) = map_res(digit1, str::parse)(input)?;
    Ok((i, rating))
}

#[derive(Debug)]
struct Predicate {
    category: Category,
    operator: Operator,
    threshold: Rating,
}

fn parse_predicate(input: &str) -> IResult<&str, Predicate> {
    // a<2006
    let (i, (category, operator, threshold)) =
        tuple((parse_category, parse_operator, parse_rating))(input)?;
    Ok((
        i,
        Predicate {
            category,
            operator,
            threshold,
        },
    ))
}

type Name = String;

fn parse_name(input: &str) -> IResult<&str, Name> {
    // px
    let (i, name) = alpha1(input)?;
    Ok((i, name.to_string()))
}

#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
enum Outcome {
    Accept,
    Reject,
    Delegate(Name),
}

fn parse_outcome(input: &str) -> IResult<&str, Outcome> {
    // A or R or rfg
    let (i, maybe) = opt(char('A'))(input)?;
    if maybe.is_some() {
        return Ok((i, Outcome::Accept));
    }
    let (i, maybe) = opt(char('R'))(input)?;
    if maybe.is_some() {
        return Ok((i, Outcome::Reject));
    }
    let (i, name) = parse_name(input)?;
    Ok((i, Outcome::Delegate(name)))
}

#[derive(Debug)]
struct Rule {
    maybe_predicate: Option<Predicate>,
    outcome: Outcome,
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    // a<2006:qkq or rfg
    let (i, maybe) = opt(separated_pair(parse_predicate, char(':'), parse_outcome))(input)?;
    if let Some((predicate, outcome)) = maybe {
        return Ok((
            i,
            Rule {
                maybe_predicate: Some(predicate),
                outcome,
            },
        ));
    }
    let (i, outcome) = parse_outcome(input)?;
    Ok((
        i,
        Rule {
            maybe_predicate: None,
            outcome,
        },
    ))
}

type Rules = Vec<Rule>;

fn parse_rules(input: &str) -> IResult<&str, Rules> {
    // a<2006:qkq,m>2090:A,rfg
    let (i, rules) = separated_list1(char(','), parse_rule)(input)?;
    Ok((i, rules))
}

#[derive(Debug)]
struct Workflow {
    name: Name,
    rules: Rules,
}

fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
    // px{a<2006:qkq,m>2090:A,rfg}
    let (i, (name, rules)) =
        tuple((parse_name, delimited(char('{'), parse_rules, char('}'))))(input)?;
    Ok((i, Workflow { name, rules }))
}

impl FromStr for Workflow {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_workflow(s).finish() {
            Ok((_, workflow)) => Ok(workflow),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

type Workflows = Vec<Workflow>;

fn parse_workflows(input: &str) -> IResult<&str, Workflows> {
    let (i, workflows) = separated_list1(newline, parse_workflow)(input)?;
    Ok((i, workflows))
}

type WorkflowForName = HashMap<Name, Workflow>;

#[derive(Debug, Eq, PartialEq)]
struct Part {
    rating_for_category: HashMap<Category, Rating>,
}

type Entry = (Category, Rating);

fn parse_entry(input: &str) -> IResult<&str, Entry> {
    // x=787
    let (i, entry) = separated_pair(parse_category, char('='), parse_rating)(input)?;
    Ok((i, entry))
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    //{x=787,m=2655,a=1222,s=2876}
    let (i, entries) = delimited(
        char('{'),
        separated_list1(char(','), parse_entry),
        char('}'),
    )(input)?;
    Ok((
        i,
        Part {
            rating_for_category: entries.into_iter().collect(),
        },
    ))
}

type Parts = Vec<Part>;

fn parse_parts(input: &str) -> IResult<&str, Parts> {
    let (i, parts) = separated_list1(newline, parse_part)(input)?;
    Ok((i, parts))
}

#[derive(Debug)]
struct Processor {
    workflows: Workflows,
    parts: Parts,
}

fn parse_processor(input: &str) -> IResult<&str, Processor> {
    let (i, (workflows, parts)) =
        separated_pair(parse_workflows, count(newline, 2), parse_parts)(input)?;
    Ok((i, Processor { workflows, parts }))
}

impl FromStr for Processor {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_processor(s).finish() {
            Ok((_, processor)) => Ok(processor),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let processor = Processor::from_str(input).ok()?;
    println!("{:?}", processor);

    None
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
