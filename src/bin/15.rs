advent_of_code::solution!(15);

use nom::character::complete::char as character;
use nom::{
    bytes::complete::is_not, character::complete::newline, error::Error, multi::separated_list1,
    sequence::terminated, Finish, IResult,
};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
struct Step {
    string: String,
}

fn parse_step(input: &str) -> IResult<&str, Step> {
    // rn=1
    let (i, characters) = is_not(",\n")(input)?;
    Ok((
        i,
        Step {
            string: characters.to_string(),
        },
    ))
}

type Hash = u32;

impl Step {
    fn hash(&self) -> Hash {
        self.string
            .chars()
            .fold(0, |acc, c| ((acc + (c as u32)) * 17) % 256)
    }
}

type Steps = Vec<Step>;

fn parse_steps(input: &str) -> IResult<&str, Steps> {
    // rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
    let (i, steps) = separated_list1(character(','), parse_step)(input)?;
    Ok((i, steps))
}

#[derive(Debug, PartialEq)]
struct Sequence {
    steps: Steps,
}

fn parse_sequence(input: &str) -> IResult<&str, Sequence> {
    // rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7\n
    let (i, steps) = terminated(parse_steps, newline)(input)?;
    Ok((i, Sequence { steps }))
}

impl FromStr for Sequence {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_sequence(s).finish() {
            Ok((_, sequence)) => Ok(sequence),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let sequence = Sequence::from_str(input).ok()?;
    // println!("{:?}\n", sequence);

    let total = sequence.steps.iter().map(|step| step.hash()).sum();

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
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
