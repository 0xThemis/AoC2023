use std::{collections::HashMap, str::Chars};

use aoc_traits::AdventOfCodeDay;

#[derive(Debug)]
struct Config(HashMap<Colors, u32>);
#[derive(PartialEq, Eq, Hash, Debug)]
enum Colors {
    Red,
    Blue,
    Green,
}

impl Default for Config {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert(Colors::Blue, 0);
        map.insert(Colors::Red, 0);
        map.insert(Colors::Green, 0);
        Self(map)
    }
}
impl Config {
    fn new(red: u32, blue: u32, green: u32) -> Self {
        let mut map = HashMap::new();
        map.insert(Colors::Blue, blue);
        map.insert(Colors::Red, red);
        map.insert(Colors::Green, green);
        Self(map)
    }

    fn set_lowest_config(&mut self, draw: Self) {
        if let Some(x) = self.0.get_mut(&Colors::Blue) {
            *x = std::cmp::max(*x, *draw.0.get(&Colors::Blue).unwrap());
        }
        if let Some(x) = self.0.get_mut(&Colors::Red) {
            *x = std::cmp::max(*x, *draw.0.get(&Colors::Red).unwrap());
        }
        if let Some(x) = self.0.get_mut(&Colors::Green) {
            *x = std::cmp::max(*x, *draw.0.get(&Colors::Green).unwrap());
        }
    }

    fn power_set(self) -> u32 {
        self.0.values().cloned().reduce(|a, b| a * b).unwrap()
    }

    fn set_color(&mut self, color: Colors, amount: u32) {
        self.0.insert(color, amount);
    }

    fn check(&self, other: Self) -> bool {
        let mut result = self.0.get(&Colors::Blue).unwrap() >= other.0.get(&Colors::Blue).unwrap();
        result &= self.0.get(&Colors::Red).unwrap() >= other.0.get(&Colors::Red).unwrap();
        result &= self.0.get(&Colors::Green).unwrap() >= other.0.get(&Colors::Green).unwrap();
        result
    }
}

fn parse_game_id(char_iter: &mut Chars) -> u32 {
    let mut game_id = String::new();
    for x in char_iter.by_ref() {
        if x.is_ascii_digit() {
            game_id.push(x);
        } else if x != ':' {
            panic!()
        } else {
            break;
        }
    }
    char_iter.next(); //remove ' '
    game_id.parse().unwrap()
}

fn parse_color(char_iter: &mut Chars) -> Colors {
    match char_iter.next() {
        Some('r') => {
            char_iter.next(); //'e';
            char_iter.next(); //'d';
            Colors::Red
        }
        Some('g') => {
            char_iter.next(); //'r';
            char_iter.next(); //'e';
            char_iter.next(); //'e';
            char_iter.next(); //'n';
            Colors::Green
        }
        Some('b') => {
            char_iter.next(); //'l';
            char_iter.next(); //'u';
            char_iter.next(); //'e';
            Colors::Blue
        }
        x => unreachable!("found {x:?}"),
    }
}

fn parse_next_set(char_iter: &mut Chars) -> (bool, Config) {
    let mut config = Config::default();
    let mut current_draw = String::new();
    loop {
        match char_iter.next() {
            Some(';') => {
                char_iter.next(); // ' '
                break (true, config);
            }
            Some(',') => {
                //next draw remove ' ',
                char_iter.next().unwrap();
            }
            Some(' ') => {
                config.set_color(parse_color(char_iter), current_draw.parse().unwrap());
                current_draw.clear();
            }
            Some(x) => current_draw.push(x),

            None => break (false, config),
        }
    }
}

fn example_2(s: &str) -> u32 {
    s.lines()
        .map(|s| &s.trim()["Game ".len()..])
        .map(|s| {
            let mut chars = s.chars();
            let game_id = parse_game_id(&mut chars);
            let mut current_lowest_config = Config::default();
            loop {
                let (cont, next_set) = parse_next_set(&mut chars);
                current_lowest_config.set_lowest_config(next_set);
                if !cont {
                    break;
                }
            }
            current_lowest_config.power_set()
        })
        .sum()
}

fn example_1(s: &str, config: Config) -> u32 {
    s.lines()
        .map(|s| &s.trim()["Game ".len()..])
        .map(|s| {
            let mut chars = s.chars();
            let game_id = parse_game_id(&mut chars);
            loop {
                let (cont, next_set) = parse_next_set(&mut chars);
                if !config.check(next_set) {
                    return 0;
                }
                if !cont {
                    break;
                }
            }
            game_id
        })
        .sum()
}

pub struct Day2Solver;

impl<'a> AdventOfCodeDay<'a> for Day2Solver {
    type ParsedInput = &'a str;

    type Part1Output = u32;

    type Part2Output = u32;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        example_1(input, Config::new(12, 13, 14))
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        example_2(input)
    }

    fn parse_input(input: &'a str) -> Self::ParsedInput {
        input
    }
}

#[test]
fn test_example_1() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    //example_1(input, Config::new(12, 12, 12));
    assert_eq!(example_1(input, Config::new(12, 12, 12)), 8);
}

#[test]
fn test_example_2() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    //example_1(input, Config::new(12, 12, 12));
    assert_eq!(example_2(input), 2286);
}

#[test]
fn my_example_1() {
    let input = "Game 1: 1 red, 5 blue, 1 green; 16 blue, 3 red; 6 blue, 5 red; 4 red, 7 blue, 1 green
    Game 2: 4 blue; 4 red, 3 blue, 1 green; 4 red, 9 blue, 2 green; 5 blue, 7 green, 4 red
    Game 3: 10 blue; 7 blue, 1 green; 19 blue, 1 green, 9 red
    Game 4: 2 green; 14 blue, 14 red, 4 green; 12 red, 11 green, 13 blue; 5 green, 9 red, 4 blue; 9 red, 7 green, 12 blue; 2 green, 3 blue, 8 red
    Game 5: 3 blue, 4 red; 12 red, 2 green, 15 blue; 1 red, 10 blue, 1 green
    Game 6: 1 blue, 7 red; 3 green, 5 red, 1 blue; 1 green, 7 red; 6 red, 1 blue, 4 green; 1 green, 8 red, 1 blue; 2 green, 4 red, 1 blue
    Game 7: 11 green, 10 blue, 2 red; 1 green, 12 blue, 2 red; 9 green, 14 blue; 1 red, 19 blue, 15 green
    Game 8: 4 green, 2 red, 14 blue; 9 green, 1 red, 15 blue; 2 green, 9 red, 8 blue; 11 green, 7 red, 8 blue; 9 red, 7 green, 6 blue
    Game 9: 4 blue, 1 green, 2 red; 1 blue, 3 red; 1 red, 3 blue, 3 green
    Game 10: 4 red, 3 green, 6 blue; 2 green, 15 blue, 6 red; 3 green, 2 blue; 2 red, 1 green; 11 blue, 7 red, 4 green; 2 blue, 2 red, 4 green
    Game 11: 10 red, 1 green, 7 blue; 5 red, 2 green, 7 blue; 2 green, 4 blue; 1 green, 10 red, 10 blue; 8 blue, 4 red
    Game 12: 6 green, 2 blue, 7 red; 3 green, 2 blue, 3 red; 6 red, 1 blue, 9 green; 9 green, 13 red, 5 blue; 6 green, 4 blue, 8 red
    Game 13: 10 green, 4 red, 6 blue; 19 red, 6 green, 7 blue; 6 blue, 5 red, 8 green
    Game 14: 4 blue, 2 green; 19 blue; 6 red, 17 blue; 10 blue, 7 red; 1 green, 2 blue, 7 red
    Game 15: 4 green, 12 blue, 15 red; 10 blue, 18 green, 13 red; 20 blue, 6 green, 10 red; 20 red, 12 blue, 13 green; 12 blue, 17 green, 10 red; 1 red, 3 blue, 7 green
    Game 16: 1 blue, 6 red, 5 green; 3 red, 3 green; 5 green, 1 red; 2 red, 1 blue, 6 green; 1 blue, 1 red, 6 green; 1 blue, 6 green
    Game 17: 4 red, 3 blue, 3 green; 8 blue, 8 green; 5 red, 3 green, 9 blue; 9 green, 12 blue, 13 red; 1 green, 1 blue, 5 red; 7 green, 6 red
    Game 18: 2 green, 11 blue, 6 red; 2 green, 11 red, 2 blue; 7 red, 4 blue, 9 green; 18 blue, 6 red, 1 green
    Game 19: 4 red, 7 green, 17 blue; 5 green, 6 red, 4 blue; 4 blue, 4 red, 1 green
    Game 20: 2 blue, 5 green, 9 red; 4 green, 8 red, 10 blue; 7 blue, 9 red, 1 green; 1 green, 10 blue, 9 red; 1 green, 8 red; 8 blue, 8 red, 1 green
    Game 21: 1 blue, 14 red; 1 green, 2 red; 9 red, 1 blue, 1 green
    Game 22: 7 green, 9 red, 4 blue; 9 red, 7 green, 9 blue; 8 green, 14 red; 5 blue; 10 red, 1 blue, 1 green; 8 green
    Game 23: 2 red, 12 green, 5 blue; 3 red, 5 blue, 3 green; 1 red, 9 green, 1 blue; 8 green, 6 blue; 13 green
    Game 24: 8 red, 7 green, 1 blue; 1 red, 6 green, 7 blue; 1 green, 3 red
    Game 25: 4 green, 2 red; 1 red, 2 green, 8 blue; 1 green; 8 blue
    Game 26: 1 green, 4 blue, 17 red; 15 red, 3 green, 3 blue; 2 blue, 2 red; 18 red, 2 green, 11 blue; 6 red, 7 blue; 10 blue, 1 green, 4 red
    Game 27: 2 red, 5 blue, 1 green; 14 green, 2 red, 6 blue; 1 red, 4 blue, 14 green
    Game 28: 3 red, 5 green, 2 blue; 2 red, 3 green, 4 blue; 1 red, 9 green, 3 blue; 13 green, 4 red, 4 blue
    Game 29: 18 red, 11 green; 4 blue, 18 red, 9 green; 16 red, 2 green, 4 blue; 2 red, 3 blue, 12 green; 1 green, 18 red; 2 blue, 15 green, 1 red
    Game 30: 10 red, 3 blue, 1 green; 6 red, 1 blue, 3 green; 2 green, 2 blue, 10 red; 6 green; 3 blue, 15 red
    Game 31: 1 blue, 7 green, 2 red; 12 red, 8 green, 4 blue; 2 green, 2 blue, 5 red; 2 blue, 3 green, 12 red; 7 red, 5 green, 4 blue; 7 red, 1 blue
    Game 32: 4 blue, 5 red, 11 green; 20 red, 8 green, 1 blue; 10 red, 7 green, 1 blue; 1 blue, 7 red, 2 green; 1 red, 19 green, 3 blue
    Game 33: 9 red; 9 red, 6 green, 7 blue; 5 red, 7 blue, 2 green
    Game 34: 5 green, 5 red, 3 blue; 8 green, 6 blue, 16 red; 12 blue, 8 red, 8 green; 1 blue, 10 red, 3 green; 1 green, 13 blue, 18 red; 4 blue, 5 green, 8 red
    Game 35: 15 green, 4 red, 8 blue; 7 red, 1 green, 14 blue; 12 green, 16 blue, 2 red
    Game 36: 3 blue, 3 green, 2 red; 7 red, 8 blue; 11 blue, 9 red; 4 red, 13 blue, 1 green
    Game 37: 4 red, 11 blue, 8 green; 6 green, 4 blue, 14 red; 5 blue, 7 green, 13 red; 6 red, 2 green, 5 blue; 4 red, 3 blue, 1 green; 6 red, 4 green, 6 blue
    Game 38: 10 green, 5 blue, 1 red; 3 red, 6 blue, 3 green; 9 green, 9 blue, 3 red; 9 blue, 1 red, 6 green
    Game 39: 3 blue, 16 red; 10 red, 4 green, 2 blue; 2 blue, 13 red, 1 green; 2 blue, 11 red, 2 green; 3 green, 13 red
    Game 40: 2 blue, 3 red, 2 green; 2 green, 2 blue, 6 red; 1 green, 9 red
    Game 41: 1 blue, 12 red; 8 blue, 1 red, 5 green; 1 green, 7 blue, 13 red; 8 red, 7 blue, 7 green; 4 green, 17 red, 9 blue; 2 green, 8 blue
    Game 42: 2 green, 6 red, 1 blue; 3 red, 2 green; 9 red, 1 green; 2 red, 2 green
    Game 43: 10 blue, 9 red; 14 blue, 4 green; 5 red, 3 green, 9 blue; 5 blue, 8 green, 1 red
    Game 44: 3 blue, 10 green, 1 red; 1 blue, 13 red, 3 green; 1 blue, 5 green, 16 red
    Game 45: 1 red, 1 green, 3 blue; 2 green, 1 red, 5 blue; 1 red, 2 blue, 1 green; 1 blue; 1 green, 5 blue; 1 blue
    Game 46: 8 green, 8 blue, 4 red; 10 green, 4 red, 7 blue; 2 red, 3 green, 14 blue
    Game 47: 3 green, 3 red; 5 green, 2 blue, 6 red; 3 blue, 5 red, 15 green; 2 green, 2 blue, 2 red
    Game 48: 11 blue, 12 green, 3 red; 8 blue, 3 red, 3 green; 1 green, 6 blue, 2 red
    Game 49: 3 blue, 17 green, 1 red; 4 red, 16 blue, 17 green; 1 green, 3 red, 5 blue; 14 blue, 1 red, 12 green
    Game 50: 2 blue, 5 red, 6 green; 8 blue, 11 green, 5 red; 2 green, 2 red, 6 blue
    Game 51: 1 green, 2 red; 4 green; 1 blue, 10 green
    Game 52: 8 blue, 9 red, 4 green; 2 green, 8 blue, 2 red; 1 red, 2 green, 1 blue; 2 blue, 8 green, 8 red; 4 red, 1 green, 9 blue; 11 blue, 4 green, 8 red
    Game 53: 1 green, 2 red; 3 blue, 1 green, 9 red; 5 blue, 11 red; 4 blue, 6 red, 1 green; 5 blue, 10 red; 5 blue, 5 red, 1 green
    Game 54: 1 blue, 8 green; 9 green, 1 red, 11 blue; 16 green, 8 blue; 5 green
    Game 55: 7 blue, 2 red, 1 green; 16 green, 19 blue, 5 red; 9 green, 3 blue, 7 red; 8 blue, 2 green, 4 red; 8 green, 15 blue, 5 red
    Game 56: 9 blue, 1 red, 4 green; 12 green, 12 blue; 1 green, 1 red, 5 blue
    Game 57: 1 green, 10 blue; 1 red, 9 blue; 10 blue, 1 red, 3 green
    Game 58: 6 red, 15 blue, 3 green; 13 blue, 5 red; 10 blue, 2 red; 5 red, 1 green, 14 blue
    Game 59: 7 red, 1 blue, 9 green; 4 green, 12 red, 2 blue; 6 green, 20 red, 1 blue; 4 blue, 9 red, 2 green; 8 red, 4 blue, 2 green
    Game 60: 11 red, 8 blue, 1 green; 18 green, 11 blue; 16 red, 10 blue, 7 green; 6 blue, 8 red; 7 red, 15 green, 4 blue
    Game 61: 1 blue, 1 green, 8 red; 3 red, 7 blue; 4 blue, 10 red; 1 green, 5 red, 8 blue; 10 red, 7 blue
    Game 62: 12 blue, 1 red, 1 green; 2 green, 1 red, 7 blue; 3 green, 18 blue; 11 blue, 4 green
    Game 63: 4 green, 4 red, 8 blue; 7 red, 5 blue, 5 green; 2 green, 20 blue, 4 red; 1 green, 4 blue, 3 red
    Game 64: 2 green, 2 red; 3 green, 2 blue; 12 green, 2 red, 4 blue; 5 red, 9 green, 8 blue; 7 blue, 6 green; 3 green, 5 red
    Game 65: 8 red, 2 green, 13 blue; 11 blue; 7 blue, 2 green; 12 blue, 1 green, 9 red
    Game 66: 1 blue, 3 red, 19 green; 3 red, 17 blue, 15 green; 9 green, 9 blue
    Game 67: 2 green, 7 blue, 1 red; 3 green, 1 red, 7 blue; 1 red, 6 green; 7 blue, 2 red, 10 green; 2 red, 5 green, 4 blue
    Game 68: 14 red, 10 green, 8 blue; 11 red, 1 blue, 6 green; 7 red, 7 green; 12 blue, 10 green, 3 red; 6 red, 12 blue, 10 green; 8 green, 14 red, 3 blue
    Game 69: 4 green, 8 red; 2 red, 15 green; 5 red, 1 blue, 12 green; 13 red, 6 green; 10 green, 13 red, 1 blue
    Game 70: 3 red, 10 blue, 3 green; 8 red, 11 blue, 11 green; 5 red, 13 green
    Game 71: 18 green, 3 red, 1 blue; 3 blue, 14 green, 2 red; 6 blue, 20 green, 4 red
    Game 72: 2 blue, 1 red; 2 blue, 3 green, 1 red; 4 blue, 2 red, 4 green
    Game 73: 11 red, 11 green; 5 green, 1 blue; 8 red, 7 green, 4 blue; 5 blue, 7 red, 12 green
    Game 74: 12 red, 12 green, 5 blue; 10 red, 7 blue, 15 green; 6 green, 19 red, 19 blue; 3 red, 7 blue, 16 green; 11 red, 14 green, 16 blue
    Game 75: 5 red, 17 green, 8 blue; 10 red, 8 blue, 19 green; 9 blue, 6 red, 18 green; 3 blue, 13 red, 12 green
    Game 76: 5 green, 2 red, 8 blue; 3 blue, 14 red, 2 green; 14 red, 1 blue; 3 green, 8 blue, 15 red; 11 red, 1 green; 11 red, 9 blue, 3 green
    Game 77: 3 blue, 2 red; 1 blue, 8 green, 11 red; 14 green, 14 red; 3 red, 5 green, 5 blue; 2 green, 16 blue, 3 red; 13 red, 7 green, 5 blue
    Game 78: 3 blue, 1 green, 1 red; 5 blue, 1 green, 1 red; 9 blue, 7 red, 1 green; 5 blue, 1 green, 5 red; 10 blue, 3 green, 7 red
    Game 79: 19 green, 17 blue, 4 red; 7 green, 7 red, 16 blue; 4 red, 10 green; 13 blue, 17 green, 2 red
    Game 80: 9 blue, 3 green; 15 blue, 1 red; 3 blue, 12 green, 2 red; 1 red, 14 green, 13 blue; 1 red, 10 blue, 16 green; 8 blue, 6 green, 2 red
    Game 81: 1 green, 3 red, 19 blue; 2 red, 1 green, 9 blue; 1 green, 2 red, 8 blue; 1 red, 1 green, 11 blue; 1 green, 3 red, 11 blue
    Game 82: 8 red, 1 blue, 4 green; 9 green, 3 blue, 4 red; 3 green, 3 blue, 18 red
    Game 83: 3 red, 13 blue, 16 green; 16 green, 2 blue; 14 green, 12 blue; 8 green, 14 blue, 4 red; 12 green, 4 blue; 20 green, 1 red
    Game 84: 4 green, 4 blue, 5 red; 6 red, 6 blue, 8 green; 5 blue, 12 green, 3 red; 5 red, 13 green; 6 blue, 1 green, 5 red
    Game 85: 10 green; 7 green, 1 blue; 5 red, 5 blue, 1 green; 2 green, 2 red, 3 blue; 3 red, 10 green, 3 blue; 1 blue, 1 red
    Game 86: 3 green, 1 red, 3 blue; 2 red, 2 green; 9 green, 2 blue, 3 red; 3 red, 3 blue, 4 green
    Game 87: 6 red, 4 green; 1 red, 3 green, 5 blue; 1 green, 7 blue, 4 red
    Game 88: 2 green, 4 red, 3 blue; 5 green, 1 blue; 3 red, 5 green, 2 blue; 1 green, 6 red, 1 blue; 7 red, 2 blue; 17 red, 13 green
    Game 89: 4 green, 2 blue, 6 red; 15 red, 7 green, 10 blue; 7 red, 9 blue, 4 green
    Game 90: 9 red, 17 blue; 1 green, 9 blue; 5 red, 8 blue; 3 blue, 9 red, 1 green; 17 blue, 1 red
    Game 91: 7 green, 3 red, 5 blue; 4 blue, 3 red, 9 green; 9 red, 7 blue, 7 green; 5 red, 6 blue, 3 green; 10 red, 2 green, 6 blue
    Game 92: 13 blue, 8 red; 7 green, 1 red, 8 blue; 5 blue, 4 red, 2 green; 9 red, 10 blue
    Game 93: 6 green; 1 blue, 16 green, 6 red; 5 green, 1 blue, 5 red; 5 red, 6 green; 16 green, 2 red, 1 blue; 11 green, 2 red
    Game 94: 9 blue, 4 green; 12 green, 17 blue; 4 green, 5 blue, 6 red; 2 red, 2 blue, 12 green
    Game 95: 5 red, 4 blue, 5 green; 2 blue, 4 green, 4 red; 4 blue, 2 red, 7 green; 1 green, 7 blue, 8 red
    Game 96: 7 blue, 6 green, 2 red; 3 green, 1 blue; 7 blue, 3 red, 5 green; 1 green, 5 blue; 6 blue, 2 red; 2 green, 1 red
    Game 97: 10 red, 1 green, 1 blue; 4 green, 11 red, 2 blue; 4 red, 1 blue, 4 green
    Game 98: 3 green, 4 blue, 7 red; 7 red, 8 green; 7 green, 16 red, 1 blue; 8 green, 2 blue, 4 red; 5 green, 3 blue, 18 red
    Game 99: 6 green, 12 red, 1 blue; 5 blue, 1 red, 7 green; 5 green, 7 red, 10 blue; 8 blue, 1 red, 7 green; 17 red, 4 blue, 9 green
    Game 100: 6 blue, 10 green; 3 green, 4 blue, 1 red; 7 blue, 1 red, 12 green";
    println!("{}", example_1(input, Config::new(14, 12, 13)));
    println!("{}", example_2(input));
}
