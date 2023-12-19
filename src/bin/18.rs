advent_of_code::solution!(18);

use hex_color::HexColor;
use nom::{
    branch::alt,
    character::complete::{char, digit1, hex_digit1, newline},
    combinator::{map_res, recognize},
    error::Error,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, terminated},
    Finish, IResult,
};
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

type Meters = u32;

fn parse_meters(input: &str) -> IResult<&str, Meters> {
    let (i, meters) = map_res(digit1, str::parse)(input)?;
    Ok((i, meters))
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
    meters: Meters,
    color: Color,
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (i, (direction, (meters, color))) = separated_pair(
        parse_direction,
        char(' '),
        separated_pair(parse_meters, char(' '), parse_color),
    )(input)?;
    Ok((
        i,
        Instruction {
            direction,
            meters,
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

pub fn part_one(input: &str) -> Option<u32> {
    let plan = Plan::from_str(input).ok()?;
    println!("{:?}", plan);

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
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
