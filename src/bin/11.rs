advent_of_code::solution!(11);

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
use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
enum Pixel {
    #[default]
    Space,
    Galaxy,
}

fn parse_pixel(input: &str) -> IResult<&str, Pixel> {
    let (i, c) = alt((char('.'), char('#')))(input)?;
    let pixel = match c {
        '.' => Pixel::Space,
        '#' => Pixel::Galaxy,
        _ => panic!(),
    };
    Ok((i, pixel))
}

type Row = Vec<Pixel>;

fn parse_row(input: &str) -> IResult<&str, Row> {
    // ...#.....
    let (i, row) = many1(parse_pixel)(input)?;
    Ok((i, row))
}

type Rows = Vec<Row>;

fn parse_rows(input: &str) -> IResult<&str, Rows> {
    // ...#.....#\n#...#.....
    let (i, rows) = separated_list1(newline, parse_row)(input)?;
    Ok((i, rows))
}

type Pixels = Array<Pixel, Ix2>;
type Index = [usize; 2];

fn distance(from_index: &Index, to_index: &Index) -> Distance {
    (0..2)
        .map(|idx| to_index[idx].abs_diff(from_index[idx]))
        .sum()
}

type Indexes = Vec<Index>;
type RowIndex = usize;

fn row_index(index: &Index) -> RowIndex {
    index[0]
}

type RowIndexes = Vec<RowIndex>;
type ColumnIndex = usize;

fn column_index(index: &Index) -> RowIndex {
    index[1]
}

type ColumnIndexes = Vec<ColumnIndex>;
type Distance = usize;

#[derive(Debug)]
struct Image {
    _width: usize,
    _height: usize,
    _pixels: Pixels,
    empty_row_indexes: RowIndexes,
    empty_column_indexes: ColumnIndexes,
    galaxy_indexes: Indexes,
}

fn parse_image(input: &str) -> IResult<&str, Image> {
    // ...#.....#\n#...#.....
    let (i, rows) = parse_rows(input)?;
    assert!(!rows.is_empty());
    assert!(rows.iter().tuple_windows().all(|(a, b)| a.len() == b.len()));

    let width = rows.len();
    let height = rows.first().map_or(0, |row| row.len());
    let mut data = Vec::new();
    rows.iter().for_each(|row| data.extend_from_slice(&row));
    let pixels = Array2::from_shape_vec((width, height), data).unwrap();
    let empty_row_indexes = pixels
        .rows()
        .into_iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|pixel| *pixel == Pixel::Space))
        .map(|(row_index, _)| row_index)
        .collect::<RowIndexes>();

    let empty_column_indexes = pixels
        .columns()
        .into_iter()
        .enumerate()
        .filter(|(_, column)| column.iter().all(|pixel| *pixel == Pixel::Space))
        .map(|(column_index, _)| column_index)
        .collect::<ColumnIndexes>();

    let galaxy_indexes = pixels
        .indexed_iter()
        .filter(|((_, _), &pixel)| pixel == Pixel::Galaxy)
        .map(|((row_index, column_index), _)| [row_index, column_index])
        .collect::<Indexes>();

    Ok((
        i,
        Image {
            _width: width,
            _height: height,
            _pixels: pixels,
            empty_row_indexes,
            empty_column_indexes,
            galaxy_indexes,
        },
    ))
}

impl FromStr for Image {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_image(s).finish() {
            Ok((_, maze)) => Ok(maze),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

impl Image {
    fn row_dilation(&self, from: RowIndex, to: RowIndex) -> Distance {
        match from.cmp(&to) {
            Ordering::Less => self
                .empty_row_indexes
                .iter()
                .filter(|&i| (from..=to).contains(i))
                .count(),
            Ordering::Equal => 0,
            Ordering::Greater => self
                .empty_row_indexes
                .iter()
                .filter(|&i| (to..=from).contains(i))
                .count(),
        }
    }

    fn column_dilation(&self, from: ColumnIndex, to: ColumnIndex) -> Distance {
        match from.cmp(&to) {
            Ordering::Less => self
                .empty_column_indexes
                .iter()
                .filter(|&j| (from..=to).contains(j))
                .count(),
            Ordering::Equal => 0,
            Ordering::Greater => self
                .empty_column_indexes
                .iter()
                .filter(|&j| (to..=from).contains(j))
                .count(),
        }
    }

    fn dilation(&self, from_index: &Index, to_index: &Index) -> Distance {
        self.row_dilation(row_index(&from_index), row_index(&to_index))
            + self.column_dilation(column_index(&from_index), column_index(&to_index))
    }

    fn dilated_distance(&self, from_index: &Index, to_index: &Index) -> Distance {
        distance(from_index, to_index) + self.dilation(from_index, to_index)
    }

    fn mega_dilation(&self, from_index: &Index, to_index: &Index) -> Distance {
        999999 * self.dilation(from_index, to_index)
    }

    fn mega_dilated_distance(&self, from_index: &Index, to_index: &Index) -> Distance {
        distance(from_index, to_index) + self.mega_dilation(from_index, to_index)
    }
}

pub fn part_one(input: &str) -> Option<Distance> {
    let image = Image::from_str(input).ok()?;
    // println!("{:?}\n", image);

    let total = image
        .galaxy_indexes
        .iter()
        .combinations(2)
        .map(|indexes| {
            let from_index = indexes[0];
            let to_index = indexes[1];
            image.dilated_distance(from_index, to_index)
        })
        .sum();

    Some(total)
}

pub fn part_two(input: &str) -> Option<Distance> {
    let image = Image::from_str(input).ok()?;
    // println!("{:?}\n", image);

    let total = image
        .galaxy_indexes
        .iter()
        .combinations(2)
        .map(|indexes| {
            let from_index = indexes[0];
            let to_index = indexes[1];
            image.mega_dilated_distance(from_index, to_index)
        })
        .sum();

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8410));
    }
}
