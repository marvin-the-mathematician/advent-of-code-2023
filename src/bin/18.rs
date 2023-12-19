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

type Increment = usize;

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

type Coordinate = isize;

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
        let increment_as_coord = *increment as Coordinate;
        match direction {
            Direction::Up => Index {
                x: self.x,
                y: self.y + increment_as_coord,
            },
            Direction::Down => Index {
                x: self.x,
                y: self.y - increment_as_coord,
            },
            Direction::Left => Index {
                x: self.x - increment_as_coord,
                y: self.y,
            },
            Direction::Right => Index {
                x: self.x + increment_as_coord,
                y: self.y,
            },
        }
    }
}

type Capacity = usize;
type Indexes = Vec<Index>;

#[derive(Debug)]
struct Lagoon {
    perimeter: Capacity,
    vertices: Indexes,
}

impl Lagoon {
    fn from_plan(plan: &Plan) -> Lagoon {
        let perimeter = plan
            .instructions
            .iter()
            .map(|instruction| instruction.increment)
            .sum();

        let origin = Index { x: 0, y: 0 };
        let vertices = plan
            .instructions
            .iter()
            .scan(origin, |index, instruction| {
                let Instruction {
                    direction,
                    increment,
                    color: _,
                } = instruction;
                *index = index.incremented(increment, direction);
                Some(*index)
            })
            .collect::<Indexes>();

        Lagoon {
            perimeter,
            vertices,
        }
    }

    fn capacity(&self) -> Capacity {
        // Shoelace formula for area of polygon from vertices...
        // And combine with Pick's Theorem for the number of interior points...
        let twice_area = self
            .vertices
            .iter()
            .cycle()
            .tuple_windows()
            .take(self.vertices.len())
            .map(|(a, b, c)| b.x * (c.y - a.y))
            .sum::<Coordinate>()
            .abs() as Capacity;

        1 + ((self.perimeter + twice_area) / 2)
    }
}

pub fn part_one(input: &str) -> Option<Capacity> {
    let plan = Plan::from_str(input).ok()?;
    // println!("{:?}", plan);

    let lagoon = Lagoon::from_plan(&plan);
    // println!("{:?}", lagoon);

    Some(lagoon.capacity())
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
