advent_of_code::solution!(2);

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

fn maybe_id(header: &str) -> Option<u32> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^Game (?P<id>[0-9]+)$").unwrap();
    }
    let maybe_id_as_string: Option<&str> = RE
        .captures(header)
        .and_then(|capture| capture.name("id").map(|id| id.as_str()));

    Some(maybe_id_as_string?.parse::<u32>().unwrap())
}

fn maybe_key_value(subset: &str) -> Option<(&str, u32)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^ (?P<value>[0-9]+) (?P<key>red|blue|green)$").unwrap();
    }
    let maybe_key_string: Option<&str> = RE
        .captures(subset)
        .and_then(|capture| capture.name("key").map(|key| key.as_str()));
    let maybe_value_string: Option<&str> = RE
        .captures(subset)
        .and_then(|capture| capture.name("value").map(|value| value.as_str()));

    Some((
        maybe_key_string?,
        maybe_value_string?.parse::<u32>().unwrap(),
    ))
}

struct Reveal {
    red: u32,
    blue: u32,
    green: u32,
}

impl Reveal {
    fn from_subset(subset: &str) -> Reveal {
        // println!("{subset}");
        let value_for_key: HashMap<&str, u32> = subset
            .split(',')
            .into_iter()
            .map(|subset| maybe_key_value(subset))
            .filter(|maybe| maybe.is_some())
            .map(|maybe| maybe.unwrap())
            .collect::<HashMap<&str, u32>>();
        //println!("{:?}", value_for_key);

        let maybe_red_value = value_for_key.get("red");
        let red_value = match maybe_red_value {
            Some(x) => x.clone(),
            None => 0,
        };

        let maybe_blue_value = value_for_key.get("blue");
        let blue_value = match maybe_blue_value {
            Some(x) => x.clone(),
            None => 0,
        };

        let maybe_green_value = value_for_key.get("green");
        let green_value = match maybe_green_value {
            Some(x) => x.clone(),
            None => 0,
        };

        Reveal {
            red: red_value,
            blue: blue_value,
            green: green_value,
        }
    }

    fn is_possible(&self) -> bool {
        self.red <= 12 && self.blue <= 14 && self.green <= 13
    }
}

struct Game {
    id: u32,
    reveals: Vec<Reveal>,
}

impl Game {
    fn from_line(line: &str) -> Game {
        //println!("{line}");
        let maybe_header = line.split(':').into_iter().next();
        let header = match maybe_header {
            Some(x) => x,
            None => panic!(),
        };
        //println!("{header}");

        let maybe_id = maybe_id(header);
        let id = match maybe_id {
            Some(x) => x,
            None => panic!(),
        };
        //println!("{id}");

        let maybe_body = line.split(':').into_iter().skip(1).next();
        let body = match maybe_body {
            Some(x) => x,
            None => panic!(),
        };
        //println!("{body}");

        Game {
            id,
            reveals: body
                .split(';')
                .into_iter()
                .map(|subset| Reveal::from_subset(subset))
                .collect::<Vec<Reveal>>(),
        }
    }

    fn minimum_power(&self) -> u32 {
        let red_max = self.reveals.iter().map(|reveal| reveal.red).max();
        let blue_max = self.reveals.iter().map(|reveal| reveal.blue).max();
        let green_max = self.reveals.iter().map(|reveal| reveal.green).max();

        red_max.unwrap_or(0) * blue_max.unwrap_or(0) * green_max.unwrap_or(0)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let total = input
        .split('\n')
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| Game::from_line(line))
        .filter(|game| game.reveals.iter().all(|reveal| reveal.is_possible()))
        .map(|game| game.id)
        .sum();

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let total = input
        .split('\n')
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| Game::from_line(line))
        .map(|game| game.minimum_power())
        .sum();

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
