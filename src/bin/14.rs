advent_of_code::solution!(14);

use itertools::Itertools;
use ndarray::prelude::*;
use ndarray::Array;
use nom::multi::many1;
use nom::{
    branch::alt,
    character::complete::{char, newline},
    error::Error,
    multi::separated_list1,
    Finish, IResult,
};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq)]
enum Direction {
    #[default]
    North,
    South,
    East,
    West,
}

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq)]
enum Rock {
    #[default]
    Round,
    Cubic,
}

type MaybeRock = Option<Rock>;

fn parse_maybe_rock(input: &str) -> IResult<&str, MaybeRock> {
    let (i, c) = alt((char('.'), char('O'), char('#')))(input)?;
    let maybe_rock = match c {
        '.' => None,
        'O' => Some(Rock::Round),
        '#' => Some(Rock::Cubic),
        _ => panic!(),
    };
    Ok((i, maybe_rock))
}

type Rank = Vec<MaybeRock>;

fn parse_rank(input: &str) -> IResult<&str, Rank> {
    // .O.#.....
    let (i, rank) = many1(parse_maybe_rock)(input)?;
    Ok((i, rank))
}

type Ranks = Vec<Rank>;

fn parse_ranks(input: &str) -> IResult<&str, Ranks> {
    // .O.#.....#\n#...#.....
    let (i, ranks) = separated_list1(newline, parse_rank)(input)?;
    Ok((i, ranks))
}

type MaybeRocks = Array<MaybeRock, Ix2>;
// type Index = [usize; 2];
// type Indexes = Vec<Index>;
// type Rank = usize;
// type Ranks = Vec<Rank>;
// type File = usize;
// type Files = Vec<File>;

#[derive(Debug)]
struct State {
    row_count: usize,
    column_count: usize,
    maybe_rocks: MaybeRocks,
}

fn parse_state(input: &str) -> IResult<&str, State> {
    // ...#.....#\n#...#.....
    let (i, ranks) = parse_ranks(input)?;
    assert!(!ranks.is_empty());
    assert!(ranks
        .iter()
        .tuple_windows()
        .all(|(a, b)| a.len() == b.len()));

    let row_count = ranks.len();
    let column_count = ranks.first().map_or(0, |row| row.len());
    let mut data = Vec::new();
    ranks.iter().for_each(|rank| data.extend_from_slice(&rank));
    let maybe_rocks = Array2::from_shape_vec((row_count, column_count), data).unwrap();

    Ok((
        i,
        State {
            row_count,
            column_count,
            maybe_rocks,
        },
    ))
}

impl FromStr for State {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_state(s).finish() {
            Ok((_, state)) => Ok(state),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

type Load = usize;

pub fn part_one(input: &str) -> Option<Load> {
    let state = State::from_str(input).ok()?;
    println!("{:?}\n", state);

    /*let total = state
    .rocks
    .rows()
    .map(|row| row.iter().filter(|rock| *rock == Rock::Round).count)
    .sum();*/

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
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
