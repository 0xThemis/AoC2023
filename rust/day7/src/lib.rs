use std::{cmp::Ordering, collections::HashMap, str::FromStr};

use aoc_traits::AdventOfCodeDay;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Card {
    Empty,
    Joker,
    One,
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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Empty,
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone)]
pub struct Draw {
    hand: Vec<Card>,
    value: Value,
    bid: u64,
}

#[derive(Debug)]
pub struct Game {
    players: Vec<Draw>,
}

impl Draw {
    fn better_than(&self, other: &Draw) -> Ordering {
        if self.value == other.value {
            self.hand
                .iter()
                .zip(other.hand.iter())
                .find_map(|(a, b)| match a.cmp(b) {
                    Ordering::Less => Some(Ordering::Less),
                    Ordering::Greater => Some(Ordering::Greater),
                    Ordering::Equal => None,
                })
                .unwrap()
        } else if self.value > other.value {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}
impl From<&[Card]> for Value {
    fn from(value: &[Card]) -> Self {
        let mut map = HashMap::new();
        value.iter().for_each(|c| {
            let entry = map.entry(c).or_insert(0);
            *entry += 1;
        });
        let mut values = map.values().cloned().collect::<Vec<_>>();
        values.sort();
        match values.as_slice() {
            [5] => Value::FiveOfAKind,
            [1, 4] => Value::FourOfAKind,
            [2, 3] => Value::FullHouse,
            [1, 2, 2] => Value::TwoPair,
            [1, 1, 3] => Value::ThreeOfAKind,
            [1, 1, 1, 2] => Value::OnePair,
            _ => Value::HighCard,
        }
    }
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            '1' => Card::One,
            x => unreachable!("found {x}"),
        }
    }
}

impl Value {
    fn use_joker(self, hand: &[Card]) -> Self {
        let amount_jokers = hand.iter().filter(|card| card == &&Card::Jack).count();
        match self {
            Value::FourOfAKind => match amount_jokers {
                0 => self,
                1 | 4 => Value::FiveOfAKind,
                _ => unreachable!(),
            },
            Value::FullHouse => match amount_jokers {
                0 => self,
                2 | 3 => Value::FiveOfAKind,
                _ => unreachable!(),
            },

            Value::ThreeOfAKind => match amount_jokers {
                0 => self,
                1 | 3 => Value::FourOfAKind,
                _ => unreachable!(),
            },
            Value::TwoPair => match amount_jokers {
                0 => self,
                1 => Value::FullHouse,
                2 => Value::FourOfAKind,
                _ => unreachable!(),
            },
            Value::OnePair => match amount_jokers {
                0 => self,
                1 | 2 => Value::ThreeOfAKind,
                _ => unreachable!(),
            },
            Value::HighCard => match amount_jokers {
                0 => self,
                1 => Value::OnePair,
                _ => unreachable!(),
            },
            _ => Value::FiveOfAKind,
        }
    }
}

impl FromStr for Draw {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_ascii_whitespace();
        let hand = split
            .next()
            .unwrap()
            .chars()
            .map(Card::from)
            .collect::<Vec<_>>();
        Ok(Self {
            value: Value::from(hand.as_slice()),
            hand,
            bid: split.next().unwrap().parse().unwrap(),
        })
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            players: s.trim().lines().map(|s| s.parse().unwrap()).collect(),
        })
    }
}

fn solve_part1(game: &Game) -> u64 {
    let mut players = game.players.to_vec();
    players.sort_by(|a, b| a.better_than(b));
    players
        .iter()
        .enumerate()
        .map(|(idx, player)| player.bid as usize * (idx + 1))
        .sum::<usize>() as u64
}

fn solve_part2(game: &Game) -> u64 {
    let mut players = game.players.to_vec();
    players.iter_mut().for_each(|player| {
        player.value = player.value.clone().use_joker(&player.hand);
        for card in player.hand.iter_mut() {
            if *card == Card::Jack {
                *card = Card::Joker;
            }
        }
    });

    players.sort_by(|a, b| a.better_than(b));
    players
        .iter()
        .enumerate()
        .map(|(idx, player)| player.bid as usize * (idx + 1))
        .sum::<usize>() as u64
}

pub struct Day7Solver;

impl<'a> AdventOfCodeDay<'a> for Day7Solver {
    type ParsedInput = Game;

    type Part1Output = u64;

    type Part2Output = u64;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        solve_part1(input)
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        solve_part2(input)
    }

    fn parse_input(input: &'a str) -> Self::ParsedInput {
        input.parse().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";
        let game = input.parse::<Game>().unwrap();
        assert_eq!(6440, solve_part1(&game));
        assert_eq!(5905, solve_part2(&game));
    }

    #[test]
    fn challenge_1() {
        let input = std::fs::read_to_string("challenge1.txt").unwrap();
        let game = input.parse::<Game>().unwrap();
        assert_eq!(250347426, solve_part1(&game));
        assert_eq!(251224870, solve_part2(&game));
    }
}
