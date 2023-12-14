advent_of_code::solution!(13);

use itertools::Itertools;
use ndarray::prelude::*;
use ndarray::Array;
use nom::multi::many1;
use nom::{
    branch::alt,
    character::complete::{char, newline},
    error::Error,
    multi::{count, separated_list1},
    Finish, IResult,
};
use std::cmp::min;
use std::str::FromStr;

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
enum Feature {
    #[default]
    Ash,
    Rock,
}

fn parse_feature(input: &str) -> IResult<&str, Feature> {
    let (i, c) = alt((char('.'), char('#')))(input)?;
    let element = match c {
        '.' => Feature::Ash,
        '#' => Feature::Rock,
        _ => panic!(),
    };
    Ok((i, element))
}

type Row = Vec<Feature>;

fn parse_row(input: &str) -> IResult<&str, Row> {
    // ...#.....
    let (i, row) = many1(parse_feature)(input)?;
    Ok((i, row))
}

type Rows = Vec<Row>;

fn parse_rows(input: &str) -> IResult<&str, Rows> {
    // ...#.....#\n#...#.....
    let (i, rows) = separated_list1(newline, parse_row)(input)?;
    Ok((i, rows))
}

type Features = Array<Feature, Ix2>;
// type Index = [usize; 2];
// type Indexes = Vec<Index>;
// type Rank = usize;
// type Ranks = Vec<Rank>;
// type File = usize;
// type Files = Vec<File>;

#[derive(Debug)]
struct Pattern {
    row_count: usize,
    column_count: usize,
    features: Features,
}

fn parse_pattern(input: &str) -> IResult<&str, Pattern> {
    // ...#.....#\n#...#.....
    let (i, rows) = parse_rows(input)?;
    assert!(!rows.is_empty());
    assert!(rows.iter().tuple_windows().all(|(a, b)| a.len() == b.len()));

    let row_count = rows.len();
    let column_count = rows.first().map_or(0, |row| row.len());
    let mut data = Vec::new();
    rows.iter().for_each(|row| data.extend_from_slice(&row));
    let features = Array2::from_shape_vec((row_count, column_count), data).unwrap();

    Ok((
        i,
        Pattern {
            row_count,
            column_count,
            features,
        },
    ))
}

type Score = usize;

impl Pattern {
    fn horizontal_line_of_reflection_score(&self) -> Score {
        match (1..self.row_count).find(|i| {
            let h = min(i.abs_diff(0), i.abs_diff(self.row_count));
            let features_above = self.features.slice(s![*i - h..*i;-1, ..]);
            let features_below = self.features.slice(s![*i..*i + h, ..]);
            features_above
                .iter()
                .zip(features_below.iter())
                .all(|(a, b)| a == b)
        }) {
            Some(row_index) => row_index,
            None => 0,
        }
    }

    fn vertical_line_of_reflection_score(&self) -> Score {
        match (1..self.column_count).find(|j| {
            let h = min(j.abs_diff(0), j.abs_diff(self.column_count));
            let features_on_left = self.features.slice(s![.., *j - h..*j;-1]);
            let features_on_right = self.features.slice(s![.., *j..*j + h]);
            features_on_left
                .iter()
                .zip(features_on_right.iter())
                .all(|(a, b)| a == b)
        }) {
            Some(column_index) => column_index,
            None => 0,
        }
    }

    fn score(&self) -> Score {
        let a = self.vertical_line_of_reflection_score();
        let b = self.horizontal_line_of_reflection_score();
        a + (100 * b)
    }
}

type Patterns = Vec<Pattern>;

#[derive(Debug)]
struct Notes {
    patterns: Patterns,
}

fn parse_notes(input: &str) -> IResult<&str, Notes> {
    let (i, patterns) = separated_list1(count(newline, 2), parse_pattern)(input)?;
    Ok((i, Notes { patterns }))
}

impl FromStr for Notes {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_notes(s).finish() {
            Ok((_, notes)) => Ok(notes),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

pub fn part_one(input: &str) -> Option<Score> {
    let notes = Notes::from_str(input).ok()?;
    // println!("{:?}\n", notes);

    let total = notes.patterns.iter().map(|pattern| pattern.score()).sum();

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
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
