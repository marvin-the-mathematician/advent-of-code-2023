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
use pathfinding::directed::cycle_detection::brent;
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

#[derive(Clone, Debug, PartialEq)]
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

impl State {
    fn tilted_in(&self, direction: Direction) -> Self {
        let data = match direction {
            Direction::North => self
                .maybe_rocks
                .columns()
                .into_iter()
                .map(|column| {
                    column
                        .into_iter()
                        .group_by(|&maybe| maybe.map_or(false, |rock| rock == Rock::Cubic))
                        .into_iter()
                        .map(|(_, group)| group.sorted().rev().cloned())
                        .flatten()
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect::<Vec<MaybeRock>>(),
            Direction::South => self
                .maybe_rocks
                .columns()
                .into_iter()
                .map(|column| {
                    column
                        .into_iter()
                        .group_by(|&maybe| maybe.map_or(false, |rock| rock == Rock::Cubic))
                        .into_iter()
                        .map(|(_, group)| group.sorted().cloned())
                        .flatten()
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect::<Vec<MaybeRock>>(),
            Direction::East => self
                .maybe_rocks
                .rows()
                .into_iter()
                .map(|row| {
                    row.into_iter()
                        .group_by(|&maybe| maybe.map_or(false, |rock| rock == Rock::Cubic))
                        .into_iter()
                        .map(|(_, group)| group.sorted().cloned())
                        .flatten()
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect::<Vec<MaybeRock>>(),
            Direction::West => self
                .maybe_rocks
                .rows()
                .into_iter()
                .map(|row| {
                    row.into_iter()
                        .group_by(|&maybe| maybe.map_or(false, |rock| rock == Rock::Cubic))
                        .into_iter()
                        .map(|(_, group)| group.sorted().rev().cloned())
                        .flatten()
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect::<Vec<MaybeRock>>(),
        };

        State {
            row_count: self.row_count,
            column_count: self.column_count,
            maybe_rocks: match direction {
                Direction::North | Direction::South => {
                    Array2::from_shape_vec((self.row_count, self.column_count).f(), data).unwrap()
                }
                Direction::East | Direction::West => {
                    Array2::from_shape_vec((self.row_count, self.column_count), data).unwrap()
                }
            },
        }
    }

    fn load(&self) -> Load {
        self.maybe_rocks
            .rows()
            .into_iter()
            .enumerate()
            .map(|(row_index, row)| {
                self.row_count.abs_diff(row_index)
                    * row
                        .iter()
                        .flatten()
                        .filter(|&rock| *rock == Rock::Round)
                        .count()
            })
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<Load> {
    let mut state = State::from_str(input).ok()?;
    // println!("{:?}\n", state);

    state = state.tilted_in(Direction::North);
    // println!("{:?}\n", state);

    Some(state.load())
}

pub fn part_two(input: &str) -> Option<Load> {
    let state = State::from_str(input).ok()?;
    // println!("{:?}", state.load());

    fn successor(state: State) -> State {
        let mut result = state.clone();
        result = result.tilted_in(Direction::North);
        result = result.tilted_in(Direction::West);
        result = result.tilted_in(Direction::South);
        result = result.tilted_in(Direction::East);
        // println!("{:?}", result.load());

        result
    }

    let (period, init_state, init_idx) = brent(state, successor);
    // println!("{:?}", (period, init_idx));

    let count = (1000000000 - init_idx) % period;
    let final_state = (0..count).fold(init_state, |state, _| successor(state));

    Some(final_state.load())
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
        assert_eq!(result, Some(64));
    }
}
