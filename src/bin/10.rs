advent_of_code::solution!(10);

use itertools::Itertools;
use ndarray::prelude::*;
use ndarray::Array;
use nom::multi::many1;
use nom::{
    branch::alt,
    character::complete::{char, newline},
    error::Error,
    multi::separated_list1,
    Finish, IResult,
};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn reversed(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::East => Direction::West,
        Direction::West => Direction::East,
    }
}

type Directions = Vec<Direction>;

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
enum Tile {
    #[default]
    Ground,
    Link {
        from: Direction,
        to: Direction,
    },
    Start,
}

fn parse_tile(input: &str) -> IResult<&str, Tile> {
    let (i, c) = alt((
        char('.'),
        char('|'),
        char('-'),
        char('J'),
        char('L'),
        char('F'),
        char('7'),
        char('S'),
    ))(input)?;
    let tile = match c {
        '.' => Tile::Ground,
        '|' => Tile::Link {
            from: Direction::North,
            to: Direction::South,
        },
        '-' => Tile::Link {
            from: Direction::East,
            to: Direction::West,
        },
        'J' => Tile::Link {
            from: Direction::North,
            to: Direction::West,
        },
        'L' => Tile::Link {
            from: Direction::North,
            to: Direction::East,
        },
        'F' => Tile::Link {
            from: Direction::South,
            to: Direction::East,
        },
        '7' => Tile::Link {
            from: Direction::South,
            to: Direction::West,
        },
        'S' => Tile::Start,
        _ => panic!(),
    };
    Ok((i, tile))
}

type Row = Vec<Tile>;

fn parse_row(input: &str) -> IResult<&str, Row> {
    // -L|F7...
    let (i, row) = many1(parse_tile)(input)?;
    Ok((i, row))
}

type Rows = Vec<Row>;

fn parse_rows(input: &str) -> IResult<&str, Rows> {
    // -L|F7\n7S-7|\nL|7||...
    let (i, rows) = separated_list1(newline, parse_row)(input)?;
    Ok((i, rows))
}

type Tiles = Array<Tile, Ix2>;
type Index = [usize; 2];

fn adjacent_index_at(direction: Direction, index: &Index) -> Index {
    match direction {
        Direction::North => [index[0] - 1, index[1]],
        Direction::South => [index[0] + 1, index[1]],
        Direction::East => [index[0], index[1] + 1],
        Direction::West => [index[0], index[1] - 1],
    }
}

#[derive(Debug)]
struct Maze {
    width: usize,
    depth: usize,
    tiles: Tiles,
    start_index: Index,
}

fn parse_maze(input: &str) -> IResult<&str, Maze> {
    // -L|F7\n7S-7|\nL|7||...
    let (i, rows) = parse_rows(input)?;
    assert!(!rows.is_empty());
    assert!(rows.iter().tuple_windows().all(|(a, b)| a.len() == b.len()));

    let width = rows.len();
    let depth = rows.first().map_or(0, |row| row.len());
    let mut data = Vec::new();
    rows.iter().for_each(|row| data.extend_from_slice(&row));
    let tiles = Array2::from_shape_vec((width, depth), data).unwrap();
    let start_index = tiles
        .indexed_iter()
        .find(|(_, &ref tile)| *tile == Tile::Start)
        .map(|((i, j), _)| [i, j])
        .unwrap();

    Ok((
        i,
        Maze {
            width,
            depth,
            tiles,
            start_index,
        },
    ))
}

impl FromStr for Maze {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_maze(s).finish() {
            Ok((_, maze)) => Ok(maze),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

impl Maze {
    fn available_directions_at_start(&self) -> Directions {
        let n = self.width - 1;
        let m = self.depth - 1;
        match self.start_index {
            [0, 0] => vec![Direction::South, Direction::East],
            [0, j] if j == m => vec![Direction::South, Direction::West],
            [i, 0] if i == n => vec![Direction::North, Direction::East],
            [i, j] if i == n && j == m => vec![Direction::North, Direction::West],
            [0, j] if (1..m).contains(&j) => {
                vec![Direction::South, Direction::East, Direction::West]
            }
            [i, 0] if (1..n).contains(&i) => {
                vec![Direction::North, Direction::South, Direction::East]
            }
            [i, j] if i == n && (1..m).contains(&j) => {
                vec![Direction::North, Direction::East, Direction::West]
            }
            [i, j] if j == m && (1..n).contains(&i) => {
                vec![Direction::North, Direction::South, Direction::West]
            }
            [i, j] if (1..n).contains(&i) && (1..m).contains(&j) => {
                vec![
                    Direction::North,
                    Direction::South,
                    Direction::East,
                    Direction::West,
                ]
            }
            _ => panic!(),
        }
    }

    fn tile_at(&self, index: Index) -> Tile {
        return self.tiles[index];
    }

    fn tile_at_start(&self) -> Tile {
        let valid_directions_at_start = self
            .available_directions_at_start()
            .into_iter()
            .map(|direction| (direction, adjacent_index_at(direction, &self.start_index)))
            .filter(|(direction, index)| {
                let tile = self.tile_at(*index);
                let from = reversed(*direction);
                match tile {
                    Tile::Link { from: a, to: b } if a == from || b == from => true,
                    _ => false,
                }
            })
            .map(|(direction, _)| direction)
            .sorted()
            .collect::<Directions>();

        match valid_directions_at_start.len() {
            2 => Tile::Link {
                from: valid_directions_at_start[0],
                to: valid_directions_at_start[1],
            },
            _ => panic!(),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let maze = Maze::from_str(input).ok()?;
    // println!("{:?}\n", maze);

    let tile = maze.tile_at_start();
    println!("{:?}\n", tile);

    None
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
