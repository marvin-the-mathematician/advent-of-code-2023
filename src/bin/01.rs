advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    fn first_digit(line: &str) -> Option<u32> {
        line.chars().find(|&c| c.is_digit(10))?.to_digit(10)
    }

    fn last_digit(line: &str) -> Option<u32> {
        line.chars().rev().find(|&c| c.is_digit(10))?.to_digit(10)
    }

    fn calibration_value(line: &str) -> Option<u32> {
        Some((10 * first_digit(line)?) + last_digit(line)?)
    }

    let total = input
        .split('\n')
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| calibration_value(line).unwrap())
        .sum();

    Some(total)
}

use lazy_static::lazy_static;
use regex::Regex;

pub fn as_digit(digit_as_string: &str) -> Option<u32> {
    match digit_as_string {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        "1" => Some(1),
        "2" => Some(2),
        "3" => Some(3),
        "4" => Some(4),
        "5" => Some(5),
        "6" => Some(6),
        "7" => Some(7),
        "8" => Some(8),
        "9" => Some(9),
        _ => None,
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    pub fn first_digit(line: &str) -> Option<u32> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^.*?(?P<digit>one|two|three|four|five|six|seven|eight|nine|[0-9])")
                    .unwrap();
        }
        let first_digit_as_string: Option<&str> = RE
            .captures(line)
            .and_then(|capture| capture.name("digit").map(|digit| digit.as_str()));

        as_digit(first_digit_as_string?)
    }

    pub fn last_digit(line: &str) -> Option<u32> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^.*(?P<digit>one|two|three|four|five|six|seven|eight|nine|[0-9])")
                    .unwrap();
        }
        let last_digit_as_string: Option<&str> = RE
            .captures(line)
            .and_then(|capture| capture.name("digit").map(|digit| digit.as_str()));

        as_digit(last_digit_as_string?)
    }

    fn calibration_value(line: &str) -> Option<u32> {
        Some((10 * first_digit(line)?) + last_digit(line)?)
    }

    let total = input
        .split('\n')
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| calibration_value(line).unwrap())
        .sum();

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
