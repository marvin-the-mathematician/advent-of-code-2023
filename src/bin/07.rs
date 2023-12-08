advent_of_code::solution!(7);

use counter::Counter;
use nom::{
    character::complete::{anychar, digit1, line_ending, space1},
    combinator::{map_res, verify},
    error::Error,
    multi::{count, separated_list1},
    sequence::separated_pair,
    Finish, IResult,
};
use std::str::FromStr;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Category {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
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

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (i, c) = verify(anychar, |c| c.is_alphanumeric())(input)?;
    let card = match c {
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
    };
    Ok((i, card))
}

type Cards = Vec<Card>;

fn parse_cards(input: &str) -> IResult<&str, Cards> {
    // 32T3K
    let (i, cards) = count(parse_card, 5)(input)?;
    Ok((i, cards))
}

type Bid = u64;

fn parse_bid(input: &str) -> IResult<&str, Bid> {
    // 765
    let (i, bid) = map_res(digit1, str::parse)(input)?;
    Ok((i, bid))
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Hand {
    category: Category,
    cards: Cards,
    bid: Bid,
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
            category,
            cards,
            bid,
        },
    ))
}

type Hands = Vec<Hand>;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
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

impl Game {
    fn ranked_hands(&self) -> Hands {
        let mut hands = self.hands.to_vec();
        hands.sort();
        hands
    }
}

type Winnings = Bid;

pub fn part_one(input: &str) -> Option<Winnings> {
    let game = Game::from_str(input).ok()?;
    // println!("{:?}", game);

    let total_winnings = game
        .ranked_hands()
        .into_iter()
        .enumerate()
        .map(|(idx, hand)| {
            let rank = (idx + 1) as Bid;
            rank * hand.bid
        })
        .sum();
    // println!("{:?}", total_winnings);

    Some(total_winnings)
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
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
