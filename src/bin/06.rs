advent_of_code::solution!(6);

use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, space0, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult,
};

type Time = u64;
type Times = Vec<Time>;

fn parse_time(input: &str) -> IResult<&str, Time> {
    let (i, time) = map_res(digit1, str::parse)(input)?;
    Ok((i, time))
}

fn parse_times(input: &str) -> IResult<&str, Times> {
    let (i, times) = preceded(
        tag("Time:"),
        delimited(space0, separated_list1(space1, parse_time), char('\n')),
    )(input)?;
    Ok((i, times))
}

type Distance = u64;
type Distances = Vec<Distance>;

fn parse_distance(input: &str) -> IResult<&str, Distance> {
    let (i, distance) = map_res(digit1, str::parse)(input)?;
    Ok((i, distance))
}

fn parse_distances(input: &str) -> IResult<&str, Distances> {
    let (i, distances) = preceded(
        tag("Distance:"),
        delimited(space0, separated_list1(space1, parse_distance), char('\n')),
    )(input)?;
    Ok((i, distances))
}

fn ways_to_win(time: Time, distance: Distance) -> u64 {
    let discriminant = (time * time) - (4 * distance);
    let radical = (discriminant as f64).sqrt();
    let lower_bound = 0.5 * (time as f64 - radical);
    let lower = if lower_bound < lower_bound.ceil() {
        lower_bound.ceil() as u64
    } else {
        lower_bound.ceil() as u64 + 1
    };
    let upper_bound = 0.5 * (time as f64 + radical);
    let upper = if upper_bound > upper_bound.floor() {
        upper_bound.floor() as u64
    } else {
        upper_bound.floor() as u64 - 1
    };

    upper - lower + 1
}

pub fn part_one(input: &str) -> Option<u64> {
    let (i, times) = parse_times(input).ok()?;
    let (_, distances) = parse_distances(i).ok()?;
    let combinations = times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| ways_to_win(time, distance))
        .product();

    Some(combinations)
}

pub fn part_two(_input: &str) -> Option<u32> {
    // Quicker to do by hand... => 34123437).
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
