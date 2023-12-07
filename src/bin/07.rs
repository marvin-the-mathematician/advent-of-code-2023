advent_of_code::solution!(7);

use counter::Counter;
use nom::{
    character::complete::{alphanumeric1, digit1, line_ending, space1},
    combinator::map_res,
    error::Error,
    multi::separated_list1,
    sequence::separated_pair,
    Finish, IResult,
};
use std::str::FromStr;

#[derive(Debug, Hash, Eq, PartialEq)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

type Cards = Vec<Card>;

fn parse_cards(input: &str) -> IResult<&str, Cards> {
    // 32T3K
    let (i, parsed) = alphanumeric1(input)?;
    Ok((
        i,
        parsed
            .chars()
            .into_iter()
            .map(|c| match c {
                '2' => Card::Two,
                '3' => Card::Three,
                '4' => Card::Four,
                '5' => Card::Five,
                '6' => Card::Six,
                '7' => Card::Seven,
                '8' => Card::Eight,
                '9' => Card::Nine,
                'T' => Card::Ten,
                'J' => Card::Jack,
                'Q' => Card::Queen,
                'K' => Card::King,
                'A' => Card::Ace,
                _ => panic!(),
            })
            .collect::<Vec<Card>>(),
    ))
}

type Bid = u32;

fn parse_bid(input: &str) -> IResult<&str, Bid> {
    // 765
    let (i, bid) = map_res(digit1, str::parse)(input)?;
    Ok((i, bid))
}

#[derive(Debug, PartialEq)]
struct Hand {
    cards: Cards,
    bid: Bid,
    category: Category,
}

fn get_category(cards: &Cards) -> Category {
    let counts = cards
        .iter()
        .collect::<Counter<_>>()
        .most_common()
        .iter()
        .map(|(_, count)| *count)
        .collect::<Vec<usize>>();

    match counts[..] {
        [5] => Category::FiveOfAKind,
        [4, 1] => Category::FourOfAKind,
        [3, 2] => Category::FullHouse,
        [3, 1, 1] => Category::ThreeOfAKind,
        [2, 2, 1] => Category::TwoPair,
        [2, 1, 1, 1] => Category::OnePair,
        [1, 1, 1, 1, 1] => Category::HighCard,
        _ => panic!(),
    }
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    // 32T3K 765
    let (i, (cards, bid)) = separated_pair(parse_cards, space1, parse_bid)(input)?;
    let category = get_category(&cards);
    Ok((
        i,
        Hand {
            cards,
            bid,
            category,
        },
    ))
}

#[derive(Debug, PartialEq)]
enum Category {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

//type Categories = Vec<Category>;

impl Hand {}

type Hands = Vec<Hand>;

#[derive(Debug, PartialEq)]
struct Game {
    hands: Hands,
}

fn parse_hands(input: &str) -> IResult<&str, Hands> {
    // 32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483
    let (i, hands) = separated_list1(line_ending, parse_hand)(input)?;
    Ok((i, hands))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    // 32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483\n
    let (i, hands) = parse_hands(input)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Game { hands }))
}

impl FromStr for Game {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_game(s).finish() {
            Ok((_, game)) => Ok(game),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let game = Game::from_str(input).ok()?;
    println!("{:?}", game);

    Some(0)
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
