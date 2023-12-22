advent_of_code::solution!(21);

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
use pathfinding::directed::bfs::bfs_reach;
use std::iter::once;
use std::str::FromStr;

type StepCount = usize;
type PlotCount = usize;

/*#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq)]
enum Direction {
    #[default]
    North,
    South,
    East,
    West,
}*/

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq)]
enum Plot {
    #[default]
    Garden,
    Rock,
    Start,
}

fn parse_plot(input: &str) -> IResult<&str, Plot> {
    let (i, c) = alt((char('.'), char('#'), char('S')))(input)?;
    let plot = match c {
        '.' => Plot::Garden,
        '#' => Plot::Rock,
        'S' => Plot::Start,
        _ => panic!(),
    };
    Ok((i, plot))
}

type Rank = Vec<Plot>;

fn parse_rank(input: &str) -> IResult<&str, Rank> {
    let (i, rank) = many1(parse_plot)(input)?;
    Ok((i, rank))
}

type Ranks = Vec<Rank>;

fn parse_ranks(input: &str) -> IResult<&str, Ranks> {
    let (i, ranks) = separated_list1(newline, parse_rank)(input)?;
    Ok((i, ranks))
}

type Plots = Array<Plot, Ix2>;
type Index = [usize; 2];

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct State {
    index: Index,
    step_count: StepCount,
}

type MaybeState = Option<State>;
type States = Vec<State>;

#[derive(Clone, Debug, PartialEq)]
struct Map {
    row_count: usize,
    column_count: usize,
    plots: Plots,
    start_index: Index,
}

fn parse_map(input: &str) -> IResult<&str, Map> {
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
    let plots = Array2::from_shape_vec((row_count, column_count), data).unwrap();
    let start_index = plots
        .indexed_iter()
        .find(|(_, plot)| **plot == Plot::Start)
        .map(|((i, j), _)| [i, j])
        .unwrap();

    Ok((
        i,
        Map {
            row_count,
            column_count,
            plots,
            start_index,
        },
    ))
}

impl FromStr for Map {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_map(s).finish() {
            Ok((_, map)) => Ok(map),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

impl Map {
    fn plot_at(&self, index: &Index) -> &Plot {
        return &self.plots[*index];
    }

    fn maybe_state_one_step_north(&self, state: &State) -> MaybeState {
        match state.index {
            [i, j] if i > 0 => Some(State {
                index: [i - 1, j],
                step_count: state.step_count + 1,
            }),
            _ => None,
        }
    }

    fn maybe_state_one_step_south(&self, state: &State) -> MaybeState {
        match state.index {
            [i, j] if i < self.row_count - 1 => Some(State {
                index: [i + 1, j],
                step_count: state.step_count + 1,
            }),
            _ => None,
        }
    }

    fn maybe_state_one_step_east(&self, state: &State) -> MaybeState {
        match state.index {
            [i, j] if j < self.column_count - 1 => Some(State {
                index: [i, j + 1],
                step_count: state.step_count + 1,
            }),
            _ => None,
        }
    }

    fn maybe_state_one_step_west(&self, state: &State) -> MaybeState {
        match state.index {
            [i, j] if j > 0 => Some(State {
                index: [i, j - 1],
                step_count: state.step_count + 1,
            }),
            _ => None,
        }
    }

    fn next_states(&self, state: &State) -> States {
        once(self.maybe_state_one_step_north(state))
            .chain(once(self.maybe_state_one_step_south(state)))
            .chain(once(self.maybe_state_one_step_east(state)))
            .chain(once(self.maybe_state_one_step_west(state)))
            .flatten()
            .filter(|state| match self.plot_at(&state.index) {
                Plot::Garden | Plot::Start => true,
                Plot::Rock => false,
            })
            .collect()
    }
}

pub fn part_one(input: &str) -> Option<PlotCount> {
    let map = Map::from_str(input).ok()?;
    // println!("{:?}\n", map);

    let target_step_count = 64;
    let reachable_plot_count = bfs_reach(
        State {
            index: map.start_index,
            step_count: 0,
        },
        |state| map.next_states(state),
    )
    .skip_while(|state| state.step_count < target_step_count)
    .take_while(|state| state.step_count == target_step_count)
    .count();
    // println!("{:?}", reachable_plot_count);

    Some(reachable_plot_count)
}

pub fn part_two(_input: &str) -> Option<StepCount> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
