advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.split('\n').collect();

    let mut sum = 0;
    for line in lines {
        // println!("{}", line);
        if line.is_empty() {
            continue;
        }

        let maybe_first_digit_as_char = line.chars().find(|&c| c.is_digit(10));
        let maybe_last_digit_as_char = line.chars().rev().find(|&c| c.is_digit(10));
        if maybe_first_digit_as_char.is_none() || maybe_last_digit_as_char.is_none() {
            return None;
        }

        let maybe_first_digit = maybe_first_digit_as_char.unwrap().to_digit(10);
        let maybe_last_digit = maybe_last_digit_as_char.unwrap().to_digit(10);
        if maybe_first_digit.is_none() || maybe_last_digit.is_none() {
            return None;
        }

        let code = (10 * maybe_first_digit.unwrap()) + maybe_last_digit.unwrap();
        // println!("{}", code);

        sum = sum + code;
    }

    // println!("{}", sum);
    Some(sum)
}

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
