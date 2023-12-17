advent_of_code::solution!(17);

use itertools::Itertools;
use ndarray::prelude::*;
use ndarray::Array;
use nom::{
    branch::alt,
    character::complete::{char, newline},
    error::Error,
    multi::{many1, separated_list1},
    Finish, IResult,
};
use std::str::FromStr;

type Loss = u32;

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
struct Block {
    loss: Loss,
}

fn parse_block(input: &str) -> IResult<&str, Block> {
    let (i, c) = alt((
        char('1'),
        char('2'),
        char('3'),
        char('4'),
        char('5'),
        char('6'),
        char('7'),
        char('8'),
        char('9'),
    ))(input)?;
    Ok((
        i,
        Block {
            loss: c.to_digit(10).unwrap(),
        },
    ))
}

type Street = Vec<Block>;

fn parse_street(input: &str) -> IResult<&str, Street> {
    let (i, street) = many1(parse_block)(input)?;
    Ok((i, street))
}

type Streets = Vec<Street>;

fn parse_streets(input: &str) -> IResult<&str, Streets> {
    let (i, streets) = separated_list1(newline, parse_street)(input)?;
    Ok((i, streets))
}

type Blocks = Array<Block, Ix2>;
type Index = [usize; 2];

#[derive(Debug)]
struct City {
    street_count: usize,
    avenue_count: usize,
    blocks: Blocks,
}

fn parse_city(input: &str) -> IResult<&str, City> {
    let (i, streets) = parse_streets(input)?;
    assert!(!streets.is_empty());
    assert!(streets
        .iter()
        .tuple_windows()
        .all(|(street, next_street)| street.len() == next_street.len()));

    let street_count = streets.len();
    let avenue_count = streets.first().map_or(0, |row| row.len());
    let mut data = Vec::new();
    streets
        .iter()
        .for_each(|street| data.extend_from_slice(&street));
    let blocks = Array2::from_shape_vec((street_count, avenue_count), data).unwrap();

    Ok((
        i,
        City {
            street_count,
            avenue_count,
            blocks,
        },
    ))
}

impl FromStr for City {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_city(s).finish() {
            Ok((_, city)) => Ok(city),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

pub fn part_one(input: &str) -> Option<Loss> {
    let city = City::from_str(input).ok()?;
    println!("{:?}", city);

    Some(0)
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
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
