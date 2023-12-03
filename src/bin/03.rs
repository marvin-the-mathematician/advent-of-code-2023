advent_of_code::solution!(3);

use itertools::Itertools;

fn is_symbol(c: char) -> bool {
    !(c.is_digit(10) || c == '.')
}

fn sum_of_part_numbers(line: &str, previous_line: &str, next_line: &str) -> u32 {
    let line_with_padding = ".".chars().chain(line.chars().chain(".".chars()));
    let previous_line_with_padding = ".".chars().chain(previous_line.chars().chain(".".chars()));
    let next_line_with_padding = ".".chars().chain(next_line.chars().chain(".".chars()));
    let flags = line_with_padding
        .tuple_windows()
        .zip(
            previous_line_with_padding
                .tuple_windows()
                .zip(next_line_with_padding.tuple_windows()),
        )
        .map(|((d, e, f), ((a, b, c), (g, h, i)))| {
            e.is_digit(10)
                && (is_symbol(a)
                    || is_symbol(b)
                    || is_symbol(c)
                    || is_symbol(d)
                    || is_symbol(f)
                    || is_symbol(g)
                    || is_symbol(h)
                    || is_symbol(i))
        });

    line.chars()
        .zip(flags)
        .group_by(|(c, _)| c.is_digit(10))
        .into_iter()
        .filter(|(key, _)| *key)
        .map(|(_, group)| group.collect::<Vec<(char, bool)>>())
        .filter(|grouped| grouped.iter().any(|(_, flag)| *flag))
        .map(|grouped| grouped.into_iter().map(|(d, _)| d).collect::<Vec<char>>())
        .map(|digits_for_part_number| {
            let base: u32 = 10;
            digits_for_part_number
                .into_iter()
                .rev()
                .map(|d| d.to_digit(10))
                .enumerate()
                .map(|(exponent, value)| value.unwrap() * base.pow(exponent as u32))
                .sum::<u32>()
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    let total = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .tuple_windows()
        .map(|(previous_line, line, next_line)| sum_of_part_numbers(line, previous_line, next_line))
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
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
