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
use pathfinding::directed::dijkstra::dijkstra;
use std::iter::once;
use std::str::FromStr;

type Cost = u32;

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
struct Block {
    cost: Cost,
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
            cost: c.to_digit(10).unwrap(),
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

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
enum Heading {
    North,
    South,
    East,
    West,
}

type RunLength = u32;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct State {
    index: Index,
    heading: Heading,
    run_length: RunLength,
}

type MaybeState = Option<State>;
type CostedState = (State, Cost);
type CostedStates = Vec<CostedState>;

#[derive(Debug)]
struct City {
    street_count: usize,
    avenue_count: usize,
    blocks: Blocks,
    start_index: Index,
    goal_index: Index,
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
            start_index: [0, 0],
            goal_index: [street_count - 1, avenue_count - 1],
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

impl City {
    fn block_at(&self, index: Index) -> &Block {
        return &self.blocks[index];
    }

    fn maybe_state_ahead(&self, state: &State) -> MaybeState {
        let [i, j] = state.index;
        // Can continue ahead at run-lengths 1 and 2.
        // Must turn at run-length 3.
        // Otherwise, panic!
        match state.run_length {
            run_length @ 1..=2 => match state.heading {
                Heading::North if i > 0 => Some(State {
                    index: [i - 1, j],
                    heading: Heading::North,
                    run_length: run_length + 1,
                }),
                Heading::South if i < self.street_count - 1 => Some(State {
                    index: [i + 1, j],
                    heading: Heading::South,
                    run_length: run_length + 1,
                }),
                Heading::East if j < self.avenue_count - 1 => Some(State {
                    index: [i, j + 1],
                    heading: Heading::East,
                    run_length: run_length + 1,
                }),
                Heading::West if j > 0 => Some(State {
                    index: [i, j - 1],
                    heading: Heading::West,
                    run_length: run_length + 1,
                }),
                _ => None,
            },
            3 => None,
            _ => panic!(),
        }
    }

    fn maybe_state_on_left(&self, state: &State) -> MaybeState {
        let [i, j] = state.index;
        match state.heading {
            Heading::North if j > 0 => Some(State {
                index: [i, j - 1],
                heading: Heading::West,
                run_length: 1,
            }),
            Heading::South if j < self.avenue_count - 1 => Some(State {
                index: [i, j + 1],
                heading: Heading::East,
                run_length: 1,
            }),
            Heading::East if i > 0 => Some(State {
                index: [i - 1, j],
                heading: Heading::North,
                run_length: 1,
            }),
            Heading::West if i < self.street_count - 1 => Some(State {
                index: [i + 1, j],
                heading: Heading::South,
                run_length: 1,
            }),
            _ => None,
        }
    }

    fn maybe_state_on_right(&self, state: &State) -> MaybeState {
        let [i, j] = state.index;
        match state.heading {
            Heading::North if j < self.avenue_count - 1 => Some(State {
                index: [i, j + 1],
                heading: Heading::East,
                run_length: 1,
            }),
            Heading::South if j > 0 => Some(State {
                index: [i, j - 1],
                heading: Heading::West,
                run_length: 1,
            }),
            Heading::East if i < self.street_count - 1 => Some(State {
                index: [i + 1, j],
                heading: Heading::South,
                run_length: 1,
            }),
            Heading::West if i > 0 => Some(State {
                index: [i - 1, j],
                heading: Heading::North,
                run_length: 1,
            }),
            _ => None,
        }
    }

    fn next_states_with_losses(&self, state: &State) -> CostedStates {
        once(self.maybe_state_ahead(state))
            .chain(once(self.maybe_state_on_left(state)))
            .chain(once(self.maybe_state_on_right(state)))
            .flatten()
            .map(|state| (state, self.block_at(state.index).cost))
            .collect()
    }

    fn did_reach_goal(&self, state: &State) -> bool {
        state.index == self.goal_index
    }
}

pub fn part_one(input: &str) -> Option<Cost> {
    let city = City::from_str(input).ok()?;
    // println!("{:?}", city);

    let start_state_heading_east = State {
        index: city.start_index,
        heading: Heading::East,
        run_length: 1,
    };
    let (_, minimal_loss_heading_east) = dijkstra(
        &start_state_heading_east,
        |state| city.next_states_with_losses(state),
        |state| city.did_reach_goal(state),
    )?;

    let start_state_heading_south = State {
        index: city.start_index,
        heading: Heading::South,
        run_length: 1,
    };
    let (_, minimal_loss_heading_south) = dijkstra(
        &start_state_heading_south,
        |state| city.next_states_with_losses(state),
        |state| city.did_reach_goal(state),
    )?;

    Some(std::cmp::min(
        minimal_loss_heading_east,
        minimal_loss_heading_south,
    ))
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
