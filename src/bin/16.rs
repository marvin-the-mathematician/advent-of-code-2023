advent_of_code::solution!(16);

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
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
enum Cardinal {
    Vertical,   // As in |.
    Horizontal, // As in -.
}

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
enum Ordinal {
    Diagonal,     // As in \.
    AntiDiagonal, // As in /.
}

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
enum Device {
    Mirror { orientation: Ordinal },
    Splitter { orientation: Cardinal },
}

type Tile = Option<Device>;

fn parse_tile(input: &str) -> IResult<&str, Tile> {
    let (i, c) = alt((char('.'), char('|'), char('-'), char('/'), char('\\')))(input)?;
    let tile = match c {
        '.' => None,
        '|' => Some(Device::Splitter {
            orientation: Cardinal::Vertical,
        }),
        '-' => Some(Device::Splitter {
            orientation: Cardinal::Horizontal,
        }),
        '\\' => Some(Device::Mirror {
            orientation: Ordinal::Diagonal,
        }),
        '/' => Some(Device::Mirror {
            orientation: Ordinal::AntiDiagonal,
        }),
        _ => panic!(),
    };
    Ok((i, tile))
}

type Rank = Vec<Tile>;

fn parse_rank(input: &str) -> IResult<&str, Rank> {
    let (i, rank) = many1(parse_tile)(input)?;
    Ok((i, rank))
}

type Ranks = Vec<Rank>;

fn parse_ranks(input: &str) -> IResult<&str, Ranks> {
    let (i, ranks) = separated_list1(newline, parse_rank)(input)?;
    Ok((i, ranks))
}

type Tiles = Array<Tile, Ix2>;
type Index = [i32; 2];
type IndexSet = HashSet<Index>;

fn _index_in(direction: Direction, index: Index) -> Index {
    let i = &index[0];
    let j = &index[1];
    match direction {
        Direction::North => [*i - 1, *j],
        Direction::South => [*i + 1, *j],
        Direction::East => [*i, *j + 1],
        Direction::West => [*i, *j - 1],
    }
}

#[derive(Debug)]
struct Grid {
    row_count: usize,
    column_count: usize,
    tiles: Tiles,
}

fn parse_grid(input: &str) -> IResult<&str, Grid> {
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
    let tiles = Array2::from_shape_vec((row_count, column_count), data).unwrap();

    Ok((
        i,
        Grid {
            row_count,
            column_count,
            tiles,
        },
    ))
}

impl FromStr for Grid {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_grid(s).finish() {
            Ok((_, grid)) => Ok(grid),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct State {
    index: Index,
    heading: Direction,
}

type States = Vec<State>;

impl State {
    fn propagated_state(&self) -> State {
        let [i, j] = self.index;
        match self.heading {
            Direction::North => State {
                index: [i - 1, j],
                heading: Direction::North,
            },
            Direction::South => State {
                index: [i + 1, j],
                heading: Direction::South,
            },
            Direction::East => State {
                index: [i, j + 1],
                heading: Direction::East,
            },
            Direction::West => State {
                index: [i, j - 1],
                heading: Direction::West,
            },
        }
    }

    fn reflected_state(&self, orientation: &Ordinal) -> State {
        let [i, j] = self.index;
        match orientation {
            Ordinal::Diagonal => match self.heading {
                Direction::North => State {
                    index: [i, j - 1],
                    heading: Direction::West,
                },
                Direction::South => State {
                    index: [i, j + 1],
                    heading: Direction::East,
                },
                Direction::East => State {
                    index: [i + 1, j],
                    heading: Direction::South,
                },
                Direction::West => State {
                    index: [i - 1, j],
                    heading: Direction::North,
                },
            },
            Ordinal::AntiDiagonal => match self.heading {
                Direction::North => State {
                    index: [i, j + 1],
                    heading: Direction::East,
                },
                Direction::South => State {
                    index: [i, j - 1],
                    heading: Direction::West,
                },
                Direction::East => State {
                    index: [i - 1, j],
                    heading: Direction::North,
                },
                Direction::West => State {
                    index: [i + 1, j],
                    heading: Direction::South,
                },
            },
        }
    }

    fn split_states(&self, orientation: &Cardinal) -> States {
        let [i, j] = self.index;
        match orientation {
            Cardinal::Vertical => match self.heading {
                Direction::North | Direction::South => vec![self.propagated_state()],
                Direction::East | Direction::West => vec![
                    State {
                        index: [i - 1, j],
                        heading: Direction::North,
                    },
                    State {
                        index: [i + 1, j],
                        heading: Direction::South,
                    },
                ],
            },
            Cardinal::Horizontal => match self.heading {
                Direction::North | Direction::South => vec![
                    State {
                        index: [i, j - 1],
                        heading: Direction::West,
                    },
                    State {
                        index: [i, j + 1],
                        heading: Direction::East,
                    },
                ],
                Direction::East | Direction::West => vec![self.propagated_state()],
            },
        }
    }
}

#[derive(Debug)]
struct GridIterator<'a> {
    will_visit_states: Vec<State>,
    did_visit_states: HashSet<State>,
    grid: &'a Grid,
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = (Index, &'a Tile);

    fn next(&mut self) -> Option<Self::Item> {
        let mut state = self.will_visit_states.pop()?;
        while state.index[0] < 0
            || state.index[0] >= self.grid.row_count as i32
            || state.index[1] < 0
            || state.index[1] >= self.grid.column_count as i32
            || self.did_visit_states.contains(&state)
        {
            state = self.will_visit_states.pop()?;
        }
        self.did_visit_states.insert(state);

        let item = (state.index, self.grid.tile_at(&state.index));
        match item.1 {
            Some(device) => match device {
                Device::Mirror { orientation } => {
                    let reflected_state = state.reflected_state(orientation);
                    self.will_visit_states.push(reflected_state);
                }
                Device::Splitter { orientation } => {
                    let split_states = state.split_states(orientation);
                    split_states
                        .into_iter()
                        .for_each(|split_state| self.will_visit_states.push(split_state));
                }
            },
            None => self.will_visit_states.push(state.propagated_state()),
        }

        return Some(item);
    }
}

impl Grid {
    fn tile_at(&self, index: &Index) -> &Tile {
        let i = index[0] as usize;
        let j = index[1] as usize;
        return &self.tiles[[i, j]];
    }

    fn indexed_iter_from(&self, state: State) -> GridIterator {
        GridIterator {
            will_visit_states: vec![state],
            did_visit_states: HashSet::new(),
            grid: self,
        }
    }

    fn energised_count_from(&self, state: State) -> usize {
        let mut energised_indexes: IndexSet = IndexSet::new();
        self.indexed_iter_from(state)
            .map(|(index, _)| energised_indexes.insert(index))
            .filter(|is_unique| *is_unique)
            .count()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = Grid::from_str(input).ok()?;
    // println!("{:?}", grid);

    Some(grid.energised_count_from(State {
        index: [0, 0],
        heading: Direction::East,
    }))
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
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
