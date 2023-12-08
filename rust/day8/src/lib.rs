use std::{cell::RefCell, collections::HashMap, rc::Rc, str::FromStr};

use aoc_traits::AdventOfCodeDay;

type ChildNode = Rc<RefCell<Node>>;
#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

pub struct Map {
    directions: Vec<Direction>,
    aaa: ChildNode,
}

#[derive(Debug)]
pub struct Node {
    name: String,
    left: Option<ChildNode>,
    right: Option<ChildNode>,
}

impl From<String> for Node {
    fn from(name: String) -> Self {
        Self {
            name,
            left: None,
            right: None,
        }
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        }
    }
}

fn insert_node(key: String, map: &mut HashMap<String, ChildNode>) -> ChildNode {
    map.entry(key.clone())
        .or_insert(Rc::new(RefCell::new(Node::from(key))))
        .clone()
}
fn solve_part1(map: &Map) -> usize {
    let mut counter = 0;
    let directions = map.directions.len();
    let mut current_node = map.aaa.clone();
    loop {
        current_node = match map.directions.get(counter % directions).unwrap() {
            Direction::Left => current_node.borrow().left.clone().unwrap(),
            Direction::Right => current_node.borrow().right.clone().unwrap(),
        };
        counter += 1;
        if current_node.borrow().name == "ZZZ" {
            break;
        }
    }
    counter
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.trim().lines();
        let directions = lines
            .next()
            .unwrap()
            .chars()
            .map(Direction::from)
            .collect::<Vec<_>>();
        lines.next(); // skip new line
        let mut nodes = HashMap::new();
        for line in lines {
            let mut split = line.split('=');
            let current_node = split.next().unwrap().trim();
            let mut split = split.next().unwrap().split_ascii_whitespace();
            let left_node = insert_node(split.next().unwrap()[1..4].to_owned(), &mut nodes);
            let right_node = insert_node(split.next().unwrap()[0..3].to_owned(), &mut nodes);
            let current_node = insert_node(current_node.to_owned(), &mut nodes);
            current_node.borrow_mut().left = Some(left_node);
            current_node.borrow_mut().right = Some(right_node);
        }
        Ok(Self {
            directions,
            aaa: nodes.get(&"AAA".to_owned()).unwrap().clone(),
        })
    }
}

pub struct Day8Solver;

impl<'a> AdventOfCodeDay<'a> for Day8Solver {
    type ParsedInput = Map;

    type Part1Output = usize;

    type Part2Output = usize;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        solve_part1(input)
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        //solve_part2(input)
        1
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
        let input = "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)";
        let map = input.parse::<Map>().unwrap();
        assert_eq!(6, solve_part1(&map));
    }

    #[test]
    fn challenge_1() {
        let input = std::fs::read_to_string("challenge.txt").unwrap();
        let map = input.parse::<Map>().unwrap();
        assert_eq!(12169, solve_part1(&map));
    }
}
