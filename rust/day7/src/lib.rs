use std::{cmp::Ordering, str::FromStr};

use aoc_traits::AdventOfCodeDay;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Card {
    Empty,
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

#[derive(Debug, Clone, Copy)]
pub struct UnorderedCard {
    card: Card,
}

impl PartialEq for UnorderedCard {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl PartialOrd for UnorderedCard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        None
    }
}

impl From<Card> for UnorderedCard {
    fn from(card: Card) -> Self {
        Self { card }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Value {
    Empty,
    HighCard(UnorderedCard),
    OnePair(UnorderedCard),
    TwoPair(UnorderedCard, UnorderedCard),
    ThreeOfAKind(UnorderedCard),
    FullHouse(UnorderedCard, UnorderedCard),
    FourOfAKind(UnorderedCard),
    FiveOfAKind(UnorderedCard),
}

#[derive(Debug)]
pub struct Draw {
    hand: [Card; 5],
    value: Value,
    bid: u64,
}

#[derive(Debug)]
pub struct Game {
    players: Vec<Draw>,
}

impl Draw {
    fn better_than(&self, other: &Draw) -> Ordering {
        println!();
        println!();
        println!("=================");
        println!("comparing {self:?}");
        println!("with      {other:?}");
        if self.value == other.value {
            //check which has higher first card
            self.hand
                .iter()
                .zip(other.hand.iter())
                .find_map(|(a, b)| {
                    if a == b {
                        None
                    } else if a < b {
                        Some(Ordering::Less)
                    } else {
                        Some(Ordering::Greater)
                    }
                })
                .unwrap()
        } else if self.value > other.value {
            println!("is greater!");
            Ordering::Greater
        } else {
            println!("is less!");
            Ordering::Less
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
    fn add_card(self, card: Card, hand: &[Card]) -> Self {
        match self {
            Value::FourOfAKind(hold_card) => {
                if hold_card.card == card {
                    Value::FiveOfAKind(hold_card)
                } else {
                    self
                }
            }
            Value::ThreeOfAKind(hold_card) => {
                if hold_card.card == card {
                    Value::FourOfAKind(hold_card)
                } else if hand.contains(&card) {
                    if card > hold_card.card {
                        Value::FullHouse(UnorderedCard::from(card), hold_card)
                    } else {
                        Value::FullHouse(hold_card, UnorderedCard::from(card))
                    }
                } else {
                    self
                }
            }
            Value::TwoPair(pair1, pair2) => {
                if card == pair1.card {
                    Value::FullHouse(pair1, pair2)
                } else if card == pair2.card {
                    Value::FullHouse(pair2, pair1)
                } else {
                    self
                }
            }
            Value::OnePair(hold_card) => {
                if card == hold_card.card {
                    Value::ThreeOfAKind(UnorderedCard::from(card))
                } else if hand.contains(&card) {
                    if card > hold_card.card {
                        Value::TwoPair(UnorderedCard::from(card), hold_card)
                    } else {
                        Value::TwoPair(hold_card, UnorderedCard::from(card))
                    }
                } else {
                    self
                }
            }
            Value::HighCard(hold_card) => {
                if hand.contains(&card) {
                    Value::OnePair(UnorderedCard::from(card))
                } else if card > hold_card.card {
                    Value::HighCard(UnorderedCard::from(card))
                } else {
                    self
                }
            }
            Value::Empty => Value::HighCard(UnorderedCard::from(card)),
            _ => unreachable!(),
        }
    }
}

impl FromStr for Draw {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_ascii_whitespace();
        let cards = split.next().unwrap();
        let mut value = Value::Empty;
        let mut draw = [Card::Empty; 5];
        for (idx, char) in cards.chars().enumerate() {
            let card = Card::from(char);
            println!("===============");
            println!("adding {card:?}");
            println!("transitioning from {value:?}");
            value = value.add_card(card, &draw);
            println!("transitioning to {value:?}");
            draw[idx] = card;
        }
        println!();
        println!();
        Ok(Self {
            hand: draw,
            value,
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

fn solve_part1(draw: &mut Game) -> u64 {
    draw.players.sort_by(|a, b| a.better_than(b));
    for (idx, test) in draw.players.iter().enumerate() {
        println!(
            "draw: {:?} {:?} has rank {} {}",
            test.value,
            test.hand,
            idx + 1,
            test.bid
        );
    }
    draw.players
        .iter()
        .enumerate()
        .map(|(idx, player)| {
            let won = player.bid as usize * (idx + 1);
            //println!("player {idx} won {won}");
            won
        })
        .sum::<usize>() as u64
}

pub struct Day7Solver;

impl<'a> AdventOfCodeDay<'a> for Day7Solver {
    type ParsedInput = Game;

    type Part1Output = u64;

    type Part2Output = u64;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        todo!()
        //solve_part1(input)
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        todo!()
    }

    fn parse_input(input: &'a str) -> Self::ParsedInput {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bug() {
        let input = "TATAT 765
        QQQJJ 483
        QQJJJ 278
        QQ7JJ 21
        JJ7QQ 21";
        let mut game = input.parse::<Game>().unwrap();
        solve_part1(&mut game);
    }

    #[test]
    fn example_1() {
        let input = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";
        let mut game = input.parse::<Game>().unwrap();
        assert_eq!(6440, solve_part1(&mut game));
    }

    #[test]
    fn challenge_1() {
        let input = std::fs::read_to_string("challenge1.txt").unwrap();
        let mut game = input.parse::<Game>().unwrap();
        assert_eq!(250347426, solve_part1(&mut game));
    }
}
