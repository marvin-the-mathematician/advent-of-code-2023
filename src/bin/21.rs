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

type Location = [isize; 2];

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Node {
    location: Location,
    step_count: StepCount,
}

type Nodes = Vec<Node>;

#[derive(Clone, Debug, PartialEq)]
struct Farm {
    row_count: usize,
    column_count: usize,
    plots: Plots,
    start_index: Index,
}

fn parse_farm(input: &str) -> IResult<&str, Farm> {
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
        Farm {
            row_count,
            column_count,
            plots,
            start_index,
        },
    ))
}

impl FromStr for Farm {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_farm(s).finish() {
            Ok((_, farm)) => Ok(farm),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

impl Farm {
    fn plot_at(&self, location: &Location) -> &Plot {
        let [x, y] = *location;
        let i = x.rem_euclid(self.row_count as isize);
        let j = y.rem_euclid(self.column_count as isize);
        let index = [i as usize, j as usize];
        return &self.plots[index];
    }

    fn node_one_step_north(&self, node: &Node) -> Node {
        let [x, y] = node.location;
        Node {
            location: [x - 1, y],
            step_count: node.step_count + 1,
        }
    }

    fn node_one_step_south(&self, node: &Node) -> Node {
        let [x, y] = node.location;
        Node {
            location: [x + 1, y],
            step_count: node.step_count + 1,
        }
    }

    fn node_one_step_east(&self, node: &Node) -> Node {
        let [x, y] = node.location;
        Node {
            location: [x, y + 1],
            step_count: node.step_count + 1,
        }
    }

    fn node_one_step_west(&self, node: &Node) -> Node {
        let [x, y] = node.location;
        Node {
            location: [x, y - 1],
            step_count: node.step_count + 1,
        }
    }

    fn successors(&self, node: &Node) -> Nodes {
        once(self.node_one_step_north(node))
            .chain(once(self.node_one_step_south(node)))
            .chain(once(self.node_one_step_east(node)))
            .chain(once(self.node_one_step_west(node)))
            .filter(|node| match self.plot_at(&node.location) {
                Plot::Garden | Plot::Start => true,
                Plot::Rock => false,
            })
            .collect()
    }

    fn reachable_plot_count(&self, step_count: StepCount) -> PlotCount {
        let [i, j] = self.start_index;
        let start_location = [i as isize, j as isize];
        let count = bfs_reach(
            Node {
                location: start_location,
                step_count: 0,
            },
            |node| self.successors(node),
        )
        .skip_while(|node| node.step_count < step_count)
        .take_while(|node| node.step_count == step_count)
        .count();

        count
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

pub fn part_two(input: &str) -> Option<StepCount> {
    let farm = Farm::from_str(input).ok()?;
    // println!("{:?}\n", farm);
    assert_eq!(farm.column_count, farm.row_count);
    assert_eq!(farm.column_count.rem_euclid(2), 1);

    let [i, j] = farm.start_index;
    assert_eq!(i, (farm.row_count - 1) / 2);
    assert_eq!(j, (farm.column_count - 1) / 2);

    // There must be a algebraic relationship between the number of steps taken and the number plots
    // visited. Obviously not linear. Perhaps quadratic? Try y = (a * s * s) + (b * s) + c. Need
    // three data points to deduce a, b, and c. Try s = x, x + d, and x + 2d, where x is the number
    // of unobstructed steps to reach the edge of starting field and d is the width of each field.

    // The number of steps it would take to reach the edge of the starting field...
    let steps = (farm.column_count - 1) / 2;
    let count = farm.reachable_plot_count(steps);
    // println!("x: {:?} -> u: {:?}", steps, count);

    // The number of steps it would take to reach the edge of the next field...
    let width = farm.column_count;
    let more_steps = steps + width;
    let larger_count = farm.reachable_plot_count(more_steps);
    // println!("x + d: {:?} -> v: {:?}", more_steps, larger_count);

    // The number of steps it would take to reach the edge of the field after that...
    let even_more_steps = steps + (2 * width);
    let even_larger_count = farm.reachable_plot_count(even_more_steps);
    // println!("x + 2d: {:?} -> w: {:?}", even_more_steps, even_larger_count);

    // Build quadratic that interpolates that evenly spaced data...
    let d = width as f64;
    let x = steps as f64;
    let u = count as f64;
    let v = larger_count as f64;
    let w = even_larger_count as f64;
    let a = (w - (2. * v) + u) / (2. * d * d);
    let b = ((v - u) / d) - (((2. * x) + d) * a);
    let c = u - (a * x * x) - (b * x);
    // println!("a: {:?}, b: {:?}, c: {:?}", a, b, c);

    let huge_steps = 26501365;
    // println!("huge_steps: {:?}", huge_steps);

    // Notice that huge_steps is of the form (x + (n * d))!
    assert_eq!((huge_steps - steps).rem_euclid(width), 0);
    let n = ((huge_steps - steps) / width) as f64;
    let s = x + (n * d);
    let huge_count = ((a * s * s) + (b * s) + c).ceil() as usize;
    // println!("x + nd: {:?} -> z: {:?}", huge_steps, huge_count);

    Some(huge_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }
}
