advent_of_code::solution!(18);

use hex_color::HexColor;
use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{char, digit1, hex_digit1, newline},
    combinator::{map_res, recognize},
    error::Error,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, terminated},
    Finish, IResult,
};
use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    let (i, c) = alt((char('U'), char('D'), char('L'), char('R')))(input)?;
    let direction = match c {
        'U' => Direction::Up,
        'D' => Direction::Down,
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => panic!(),
    };
    Ok((i, direction))
}

type Increment = i32;

fn parse_increment(input: &str) -> IResult<&str, Increment> {
    let (i, increment) = map_res(digit1, str::parse)(input)?;
    Ok((i, increment))
}

fn parse_hex_color(input: &str) -> IResult<&str, HexColor> {
    let (i, hex_color) = map_res(
        recognize(preceded(char('#'), hex_digit1)),
        HexColor::parse_rgb,
    )(input)?;
    Ok((i, hex_color))
}

type Color = HexColor;

fn parse_color(input: &str) -> IResult<&str, Color> {
    let (i, color) = delimited(char('('), parse_hex_color, char(')'))(input)?;
    Ok((i, color))
}

#[derive(Debug, Eq, PartialEq)]
struct Instruction {
    direction: Direction,
    increment: Increment,
    color: Color,
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (i, (direction, (increment, color))) = separated_pair(
        parse_direction,
        char(' '),
        separated_pair(parse_increment, char(' '), parse_color),
    )(input)?;
    Ok((
        i,
        Instruction {
            direction,
            increment,
            color,
        },
    ))
}

type Instructions = Vec<Instruction>;

fn parse_instructions(input: &str) -> IResult<&str, Instructions> {
    let (i, instructions) = separated_list1(newline, parse_instruction)(input)?;
    Ok((i, instructions))
}

#[derive(Debug)]
struct Plan {
    instructions: Instructions,
}

fn parse_plan(input: &str) -> IResult<&str, Plan> {
    let (i, instructions) = terminated(parse_instructions, newline)(input)?;
    Ok((i, Plan { instructions }))
}

impl FromStr for Plan {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_plan(s).finish() {
            Ok((_, plan)) => Ok(plan),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

type Coordinate = i32;
type Coordinates = Vec<Coordinate>;
type CoordinatesByGroup = Vec<Coordinates>;
type CoordinatesByGroupByGroup = Vec<CoordinatesByGroup>;

#[derive(Copy, Clone, Debug, Hash)]
struct Index {
    x: Coordinate, // Increases rightwards.
    y: Coordinate, // Increase upwards.
}

impl PartialEq for Index {
    fn eq(&self, other: &Self) -> bool {
        self.y == other.y && self.x == other.x
    }
}

impl Eq for Index {}

impl PartialOrd for Index {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match other.y.partial_cmp(&self.y) {
            Some(Ordering::Equal) => self.x.partial_cmp(&other.x),
            result @ _ => result,
        }
    }
}

impl Ord for Index {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.y.cmp(&self.y) {
            Ordering::Equal => self.x.cmp(&other.x),
            result @ _ => result,
        }
    }
}

impl Index {
    fn incremented(&self, increment: &Increment, direction: &Direction) -> Index {
        match direction {
            Direction::Up => Index {
                x: self.x,
                y: self.y + increment,
            },
            Direction::Down => Index {
                x: self.x,
                y: self.y - increment,
            },
            Direction::Left => Index {
                x: self.x - increment,
                y: self.y,
            },
            Direction::Right => Index {
                x: self.x + increment,
                y: self.y,
            },
        }
    }
}

type Indexes = Vec<Index>;

#[derive(Debug)]
struct Lagoon {
    trench_indexes: Indexes,
    xs_by_rank_by_run: CoordinatesByGroupByGroup,
}

type Capacity = usize;

impl Lagoon {
    fn from_plan(plan: &Plan) -> Lagoon {
        let origin = Index { x: 0, y: 0 };
        let trench_indexes = plan
            .instructions
            .iter()
            .scan(origin, |index, instruction| {
                let Instruction {
                    direction,
                    increment,
                    color: _,
                } = instruction;
                let indexes = (1..=*increment)
                    .map(|k| index.incremented(&k, direction))
                    .collect::<Indexes>();
                println!("{:?}", indexes);

                *index = index.incremented(increment, direction);

                Some(indexes)
            })
            .flatten()
            .inspect(|index| println!("{:?}", index))
            .sorted()
            .collect::<Indexes>();

        let xs_by_rank_by_run = trench_indexes
            .iter()
            .group_by(|&index| index.y)
            .into_iter()
            .map(|(_, indexes)| {
                indexes
                    .into_iter()
                    .map(|index| index.x)
                    .enumerate()
                    .group_by(|(i, x)| *x as usize - *i)
                    .into_iter()
                    .map(|(_, group)| group.map(|(_, x)| x).collect())
                    .collect()
            })
            .collect();

        Lagoon {
            trench_indexes,
            xs_by_rank_by_run,
        }
    }

    fn trench_capacity(&self) -> Capacity {
        self.trench_indexes.len()
    }

    fn capacity(&self) -> Capacity {
        self.xs_by_rank_by_run
            .iter()
            .inspect(|xs_for_rank_by_run| println!("{:?}", xs_for_rank_by_run))
            .map(|xs_for_rank_by_run| {
                xs_for_rank_by_run
                    .iter()
                    .tuples()
                    .map(|(xs_for_run, xs_for_next_run)| {
                        (xs_for_next_run.first().unwrap() - xs_for_run.last().unwrap()) as usize - 1
                    })
                    .sum::<Capacity>()
            })
            .sum::<Capacity>()
            + self.trench_capacity()
    }
}

pub fn part_one(input: &str) -> Option<Capacity> {
    let plan = Plan::from_str(input).ok()?;
    println!("{:?}", plan);

    let lagoon = Lagoon::from_plan(&plan);
    println!("{:?}", lagoon);

    let capacity = lagoon.capacity();
    println!("{:?}", capacity);
    println!("{:?}", lagoon.trench_capacity());

    Some(capacity)
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
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_one_trial_one() {
        let result = part_one(&advent_of_code::template::read_file_for_trial(
            "examples", DAY, "1",
        ));
        assert_eq!(result, Some(15));
    }

    #[test]
    fn test_part_one_trial_two() {
        let result = part_one(&advent_of_code::template::read_file_for_trial(
            "examples", DAY, "2",
        ));
        assert_eq!(result, Some(32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
