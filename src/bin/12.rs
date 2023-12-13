advent_of_code::solution!(12);

use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::{
    branch::alt,
    character::complete::{char, newline},
    error::Error,
    multi::separated_list1,
    sequence::separated_pair,
    Finish, IResult,
};
use std::str::FromStr;

#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
enum Condition {
    Unknown,
    Damaged,
    Operational,
}

fn parse_condition(input: &str) -> IResult<&str, Condition> {
    let (i, c) = alt((char('?'), char('#'), char('.')))(input)?;
    let condition = match c {
        '?' => Condition::Unknown,
        '#' => Condition::Damaged,
        '.' => Condition::Operational,
        _ => panic!(),
    };
    Ok((i, condition))
}

type Row = Vec<Condition>;

fn parse_row(input: &str) -> IResult<&str, Row> {
    // ...#.....
    let (i, row) = many1(parse_condition)(input)?;
    Ok((i, row))
}

type Run = u32;

fn parse_run(input: &str) -> IResult<&str, Run> {
    let (i, run) = map_res(digit1, str::parse)(input)?;
    Ok((i, run))
}

type Runs = Vec<Run>;

fn parse_runs(input: &str) -> IResult<&str, Runs> {
    // 20 27 37 68 149 321...
    let (i, runs) = separated_list1(char(','), parse_run)(input)?;
    Ok((i, runs))
}

#[derive(Debug, PartialEq)]
struct Record {
    row: Row,
    runs: Runs,
}

fn parse_record(input: &str) -> IResult<&str, Record> {
    let (i, (row, runs)) = separated_pair(parse_row, char(' '), parse_runs)(input)?;
    Ok((i, Record { row, runs }))
}

type Records = Vec<Record>;

#[derive(Debug, PartialEq)]
struct Report {
    records: Records,
}

fn parse_report(input: &str) -> IResult<&str, Report> {
    let (i, records) = separated_list1(newline, parse_record)(input)?;
    Ok((i, Report { records }))
}

impl FromStr for Report {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_report(s).finish() {
            Ok((_, report)) => Ok(report),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let report = Report::from_str(input).ok()?;
    println!("{:?}\n", report);

    None
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
