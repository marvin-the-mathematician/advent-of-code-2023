advent_of_code::solution!(4);

use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, space0, space1},
    combinator::map_res,
    error::Error,
    multi::separated_list1,
    sequence::separated_pair,
    Finish, IResult,
};
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn score(&self) -> u32 {
        let winning_number_count = self
            .numbers
            .iter()
            .filter(|&number| self.winning_numbers.contains(number))
            .count() as u32;

        let base: u32 = 2;
        match winning_number_count {
            0 => 0,
            count => base.pow(count - 1),
        }
    }
}

impl FromStr for Card {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_id(input: &str) -> IResult<&str, u32> {
            let (i, _) = tag("Card")(input)?;
            let (i, _) = space1(i)?;
            let (i, id) = map_res(digit1, str::parse)(i)?;

            Ok((i, id))
        }

        fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>> {
            let (i, _) = space0(input)?;
            let (i, numbers) = separated_list1(space1, map_res(digit1, str::parse))(i)?;
            let (i, _) = space0(i)?;

            Ok((i, numbers))
        }

        match separated_pair(
            parse_id,
            char(':'),
            separated_pair(parse_numbers, char('|'), parse_numbers),
        )(s)
        .finish()
        {
            Ok((_remaining, (id, (winning_numbers, numbers)))) => Ok(Card {
                id,
                winning_numbers: HashSet::from_iter(winning_numbers),
                numbers,
            }),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let total = input
        .split('\n')
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| Card::from_str(line).unwrap())
        .map(|card| card.score())
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
