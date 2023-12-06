advent_of_code::solution!(5);

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, space1},
    combinator::map_res,
    error::Error,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    Finish, IResult,
};
use std::str::FromStr;

type Distance = u64;
type Descriptor = u64;
type Descriptors = Vec<Descriptor>;

fn parse_distance(input: &str) -> IResult<&str, Descriptor> {
    let (i, distance) = map_res(digit1, str::parse)(input)?;
    Ok((i, distance))
}

fn parse_descriptor(input: &str) -> IResult<&str, Descriptor> {
    let (i, descriptor) = map_res(digit1, str::parse)(input)?;
    Ok((i, descriptor))
}

#[derive(Debug, PartialEq)]
struct Range {
    source_start: Descriptor,
    destination_start: Descriptor,
    length: Distance,
}

fn parse_range(input: &str) -> IResult<&str, Range> {
    // 50 98 2
    let (i, (destination_start, (source_start, length))) = separated_pair(
        parse_descriptor,
        char(' '),
        separated_pair(parse_descriptor, char(' '), parse_distance),
    )(input)?;
    Ok((
        i,
        Range {
            source_start,
            destination_start,
            length,
        },
    ))
}

impl Range {
    fn contains(&self, source: Descriptor) -> bool {
        source >= self.source_start && source < self.source_start + self.length
    }

    fn get_destination(&self, source: Descriptor) -> Descriptor {
        assert!(self.contains(source));
        self.destination_start + source - self.source_start
    }
}

type Ranges = Vec<Range>;

fn parse_ranges(input: &str) -> IResult<&str, Ranges> {
    // 50 98 2\n52 50 48
    let (i, ranges) = separated_list1(char('\n'), parse_range)(input)?;
    Ok((i, ranges))
}

#[derive(Debug, PartialEq)]
struct Map {
    source_category: String,
    destination_category: String,
    ranges: Ranges,
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    // seed-to-soil map:\n50 98 2\n52 50 48
    let (i, (source_category, destination_category)) =
        separated_pair(alpha1, tag("-to-"), alpha1)(input)?;
    let (i, _) = tag(" map:\n")(i)?;
    let (i, ranges) = parse_ranges(i)?;
    Ok((
        i,
        Map {
            source_category: source_category.to_string(),
            destination_category: destination_category.to_string(),
            ranges,
        },
    ))
}

impl Map {
    fn get_destination(&self, source: Descriptor) -> Descriptor {
        let maybe_range = self.ranges.iter().find(|&range| range.contains(source));
        match maybe_range {
            Some(range) => range.get_destination(source),
            None => source,
        }
    }
}

type Maps = Vec<Map>;

fn parse_maps(input: &str) -> IResult<&str, Maps> {
    // seed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2
    let (i, maps) = separated_list1(tag("\n\n"), parse_map)(input)?;
    Ok((i, maps))
}

#[derive(Debug, PartialEq)]
struct Almanac {
    seeds: Descriptors,
    maps: Maps,
}

fn parse_seeds(input: &str) -> IResult<&str, Descriptors> {
    // seeds: 3139431799 50198205 3647185634\n\n
    let (i, descriptors) = delimited(
        tag("seeds: "),
        separated_list1(space1, parse_descriptor),
        tag("\n\n"),
    )(input)?;
    Ok((i, descriptors))
}

fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    // seeds: 3139431799 50198205 3647185634\n\nseed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2
    let (i, seeds) = parse_seeds(input)?;
    let (i, maps) = parse_maps(i)?;
    Ok((i, Almanac { seeds, maps }))
}

impl FromStr for Almanac {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_almanac(s).finish() {
            Ok((_, almanac)) => Ok(almanac),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

impl Almanac {
    fn get_location_for_seed(&self, seed: Descriptor) -> Descriptor {
        self.maps
            .iter()
            .fold(seed, |source, map| map.get_destination(source))
    }
}

pub fn part_one(input: &str) -> Option<Descriptor> {
    let almanac = Almanac::from_str(input).ok()?;
    // println!("{:?}", almanac);

    let nearest_location = almanac
        .seeds
        .iter()
        .map(|&seed| almanac.get_location_for_seed(seed))
        .min()?;

    Some(nearest_location)
}

pub fn part_two(_input: &str) -> Option<Descriptor> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
