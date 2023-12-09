advent_of_code::solution!(8);

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char},
    error::Error,
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair},
    Finish, IResult,
};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    // R|L
    let (i, c) = alt((char('L'), char('R')))(input)?;
    let edge_category = match c {
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => panic!(),
    };
    Ok((i, edge_category))
}

type Directions = Vec<Direction>;

fn parse_directions(input: &str) -> IResult<&str, Directions> {
    // LRRLRRRLLRRR...
    let (i, cards) = many1(parse_direction)(input)?;
    Ok((i, cards))
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Descriptor {
    id: String,
}

fn parse_descriptor(input: &str) -> IResult<&str, Descriptor> {
    let (i, chars) = alpha1(input)?;
    Ok((
        i,
        Descriptor {
            id: chars.to_string(),
        },
    ))
}

#[derive(Debug, Clone, PartialEq)]
struct Neighbours {
    descriptor_on_left: Descriptor,
    descriptor_on_right: Descriptor,
}

type Entry = (Descriptor, Neighbours);

fn parse_entry(input: &str) -> IResult<&str, Entry> {
    // AAA = (BBB, CCC)
    let (i, (descriptor, (descriptor_on_left, descriptor_on_right))) = separated_pair(
        parse_descriptor,
        tag(" = "),
        delimited(
            char('('),
            separated_pair(parse_descriptor, tag(", "), parse_descriptor),
            char(')'),
        ),
    )(input)?;
    Ok((
        i,
        (
            descriptor,
            Neighbours {
                descriptor_on_left,
                descriptor_on_right,
            },
        ),
    ))
}

type Lookup = HashMap<Descriptor, Neighbours>;

fn parse_lookup(input: &str) -> IResult<&str, Lookup> {
    // AAA = (BBB, CCC)\nBBB = (DDD, EEE)...
    let (i, entries) = separated_list1(char('\n'), parse_entry)(input)?;
    Ok((
        i,
        entries
            .into_iter()
            .map(|(descriptor, neighbours)| (descriptor, neighbours))
            .collect(),
    ))
}

#[derive(Debug, PartialEq)]
struct Graph {
    directions: Directions,
    lookup: Lookup,
}

fn parse_graph(input: &str) -> IResult<&str, Graph> {
    // RL\n\nAAA = (BBB, CCC)\nBBB = (DDD, EEE)
    let (i, (directions, lookup)) =
        separated_pair(parse_directions, tag("\n\n"), parse_lookup)(input)?;
    Ok((i, Graph { directions, lookup }))
}

impl FromStr for Graph {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_graph(s).finish() {
            Ok((_, graph)) => Ok(graph),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

impl Graph {
    fn get_neighbouring_descriptor_on_left(&self, descriptor: &Descriptor) -> Descriptor {
        let neighbours = self.lookup.get(&descriptor).unwrap();
        neighbours.descriptor_on_left.clone()
    }

    fn get_neighbouring_descriptor_on_right(&self, descriptor: &Descriptor) -> Descriptor {
        let neighbours = self.lookup.get(&descriptor).unwrap();
        neighbours.descriptor_on_right.clone()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let graph = Graph::from_str(input).ok()?;
    let source = Descriptor {
        id: String::from("AAA"),
    };
    let goal = Descriptor {
        id: String::from("ZZZ"),
    };

    let mut steps = 0;
    let _ = graph
        .directions
        .iter()
        .cycle()
        .fold_while(source, |descriptor, direction| {
            if descriptor == goal {
                Done(descriptor)
            } else {
                steps += 1;
                Continue(match direction {
                    Direction::Left => graph.get_neighbouring_descriptor_on_left(&descriptor),
                    Direction::Right => graph.get_neighbouring_descriptor_on_right(&descriptor),
                })
            }
        })
        .into_inner();

    Some(steps)
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
