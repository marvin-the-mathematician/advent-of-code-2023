advent_of_code::solution!(9);

use itertools::Itertools;
use nom::{
    character::complete::{char, digit1, newline},
    combinator::{map_res, opt, recognize},
    error::Error,
    multi::separated_list1,
    sequence::preceded,
    Finish, IResult,
};
use std::str::FromStr;

type Value = i64;

fn parse_value(input: &str) -> IResult<&str, Value> {
    let (i, value) = map_res(recognize(preceded(opt(char('-')), digit1)), |s| {
        Value::from_str(s)
    })(input)?;

    Ok((i, value))
}

type Values = Vec<Value>;

#[derive(Debug, PartialEq)]
struct History {
    values: Values,
}

fn parse_history(input: &str) -> IResult<&str, History> {
    // 20 27 37 68 149 321...
    let (i, values) = separated_list1(char(' '), parse_value)(input)?;
    Ok((i, History { values }))
}

fn differences(values: &Values) -> Values {
    values
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect::<Values>()
}

impl History {
    fn extrapolated_value(&self) -> Value {
        let length = self.values.len();
        let last = self.values.last().unwrap();
        let first_differences = differences(&self.values);
        let value = last
            + (0..length)
                .scan(first_differences, |values, _| {
                    if values.iter().all(|&value| value == 0) {
                        None
                    } else {
                        let result = values.last().unwrap().clone();
                        *values = differences(values);
                        Some(result)
                    }
                })
                .sum::<Value>();

        value
    }
}

type Histories = Vec<History>;

#[derive(Debug, PartialEq)]
struct Report {
    histories: Histories,
}

fn parse_report(input: &str) -> IResult<&str, Report> {
    // 20 27 37 68 149 321...\n
    // 4 27 79 177 347...
    let (i, histories) = separated_list1(newline, parse_history)(input)?;
    Ok((i, Report { histories }))
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

pub fn part_one(input: &str) -> Option<Value> {
    let report = Report::from_str(input).ok()?;
    // println!("{:?}\n", report);

    let total = report
        .histories
        .into_iter()
        .map(|history| history.extrapolated_value())
        .sum();

    Some(total)
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
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
