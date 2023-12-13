advent_of_code::solution!(12);

use cached::proc_macro::cached;
use nom::{
    branch::alt,
    character::complete::{char, digit1, newline},
    combinator::map_res,
    error::Error,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    Finish, IResult,
};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
enum Status {
    Unknown,
    Damaged,
    Operational,
}

fn parse_status(input: &str) -> IResult<&str, Status> {
    let (i, c) = alt((char('?'), char('#'), char('.')))(input)?;
    let condition = match c {
        '?' => Status::Unknown,
        '#' => Status::Damaged,
        '.' => Status::Operational,
        _ => panic!(),
    };
    Ok((i, condition))
}

type Statuses = Vec<Status>;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Row {
    statuses: Statuses,
}

fn parse_row(input: &str) -> IResult<&str, Row> {
    // ...#.....
    let (i, statuses) = many1(parse_status)(input)?;
    Ok((i, Row { statuses }))
}

type Run = usize;

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

type Arrangements = usize;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Record {
    row: Row,
    runs: Runs,
}

fn parse_record(input: &str) -> IResult<&str, Record> {
    let (i, (row, runs)) = separated_pair(parse_row, char(' '), parse_runs)(input)?;
    Ok((i, Record { row, runs }))
}

#[cached]
fn cached_arrangements(statuses: Statuses, runs: Runs) -> Arrangements {
    fn arrangments_given_possible_match(
        run: &Run,
        statuses: &Statuses,
        runs: &Runs,
    ) -> Arrangements {
        // For a match to exist the next run length statuses must not be Operational and the
        // following status must not be Damaged (so that the run ends).
        // Otherwise, a match is not possible.
        return if statuses[..*run]
            .iter()
            .all(|status| *status != Status::Operational)
            && (statuses.len() == *run || statuses[*run] != Status::Damaged)
        {
            // Consume the match and continue...
            // Be careful not to run off the end since we need to consume the gap if it exists...
            if statuses.len() == *run {
                cached_arrangements(statuses[(*run)..].to_vec(), runs[1..].to_vec())
            } else {
                cached_arrangements(statuses[(*run + 1)..].to_vec(), runs[1..].to_vec())
            }
        } else {
            0
        };
    }

    // If there are no unprocessed statuses...
    if statuses.is_empty() {
        // If there are no unmatched runs then we are done. Otherwise, a match is not possible.
        return if runs.is_empty() { 1 } else { 0 };
    }
    assert!(!statuses.is_empty());

    // If there are unprocessed statuses...
    if runs.is_empty() {
        // If there are no unmatched runs then we are done as long as all remaining statuses are not
        // Damaged; since all Unknown statuses could be chosen as Operational.
        // Otherwise, a match is not possible.
        return if !statuses.contains(&Status::Damaged) {
            1
        } else {
            0
        };
    }
    assert!(!runs.is_empty());

    // If there is an unprocessed status and an unprocessed run...
    let status = statuses.first().unwrap();
    let run = runs.first().unwrap();

    // If the number of unprocessed statuses is too small to accommodate the unprocessed runs...
    let min_match_len = runs.iter().sum::<Run>() + runs.len() - 1;
    if statuses.len() < min_match_len {
        return 0;
    }

    // If the next status is Operational then we can skip it and continue as if it wasn't there...
    if *status == Status::Operational {
        return cached_arrangements(statuses[1..].to_vec(), runs.clone());
    }
    assert_ne!(*status, Status::Operational);

    // If the next status is Damaged then we should check for a match with the next run...
    if *status == Status::Damaged {
        return arrangments_given_possible_match(run, &statuses, &runs);
    }
    assert_ne!(*status, Status::Damaged);
    assert_eq!(*status, Status::Unknown);

    // We have to branch here since the next status is unknown...
    // Return the sum of the arrangements both assuming the next status is Operational or
    // assuming the next status is Damaged...
    cached_arrangements(statuses[1..].to_vec(), runs.clone())
        + arrangments_given_possible_match(run, &statuses, &runs)
}

impl Record {
    fn unfolded(record: &Record) -> Record {
        let Record {
            row: Row { statuses },
            runs,
        } = record;

        let unfolded_statuses = (0..4)
            .flat_map(|_| {
                statuses
                    .iter()
                    .map(|status| *status)
                    .chain((0..1).map(|_| Status::Unknown))
            })
            .chain(statuses.iter().map(|status| *status))
            .collect::<Statuses>();

        let unfolded_runs = (0..5)
            .flat_map(|_| runs.iter().map(|run| *run))
            .collect::<Runs>();

        Record {
            row: Row {
                statuses: unfolded_statuses,
            },
            runs: unfolded_runs,
        }
    }

    fn arrangements(&self) -> Arrangements {
        cached_arrangements(self.row.statuses.clone(), self.runs.clone())
    }
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

pub fn part_one(input: &str) -> Option<Arrangements> {
    let report = Report::from_str(input).ok()?;
    // println!("{:?}\n", report);

    let total = report
        .records
        .iter()
        .map(|record| record.arrangements())
        .sum();

    Some(total)
}

pub fn part_two(input: &str) -> Option<Arrangements> {
    let report = Report::from_str(input).ok()?;
    // println!("{:?}\n", report);

    let total = report
        .records
        .iter()
        .map(|record| Record::unfolded(record))
        .map(|unfolded_record| unfolded_record.arrangements())
        .sum();

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
