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

// use regex::Regex;

pub fn part_two(input: &str) -> Option<u32> {
    let _lines: Vec<&str> = input.split('\n').collect();

    None
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
        assert_eq!(result, None);
    }
}
