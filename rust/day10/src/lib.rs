use std::{cell::OnceCell, collections::HashSet, iter, str::FromStr};

use aoc_traits::AdventOfCodeDay;
use itertools::Itertools;

type Map = Vec<Vec<Pipe>>;

#[derive(Clone, PartialEq)]
pub enum Pipe {
    None,
    N2S,
    E2W,
    N2E,
    N2W,
    S2W,
    S2E,
    Start,
    Extension,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
pub struct Landscape {
    start_x: usize,
    start_y: usize,
    map: Map,
    pipes: OnceCell<Vec<Vec<usize>>>,
}

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
    direction: Direction,
}

impl Position {
    fn new(x: usize, y: usize, direction: Direction) -> Self {
        Self { x, y, direction }
    }
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            '|' => Pipe::N2S,
            '-' => Pipe::E2W,
            'L' => Pipe::N2E,
            'J' => Pipe::N2W,
            '7' => Pipe::S2W,
            'F' => Pipe::S2E,
            '.' => Pipe::None,
            'S' => Pipe::Start,
            '*' => Pipe::Extension,
            x => unreachable!("unknown '{x}'"),
        }
    }
}

impl std::fmt::Debug for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "."),
            Self::N2S => write!(f, "|"),
            Self::E2W => write!(f, "-"),
            Self::N2E => write!(f, "L"),
            Self::N2W => write!(f, "J"),
            Self::S2W => write!(f, "7"),
            Self::S2E => write!(f, "F"),
            Self::Start => write!(f, "S"),
            Self::Extension => write!(f, "*"),
        }
    }
}

impl FromStr for Landscape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start_x = 0;
        let mut start_y = 0;
        let pipes = s
            .trim()
            .lines()
            .enumerate()
            .map(|(x, l)| {
                iter::once('*')
                    .chain(l.trim().chars())
                    .chain(iter::once('*'))
                    .enumerate()
                    .map(|(y, l)| {
                        let pipe = Pipe::from(l);
                        if pipe == Pipe::Start {
                            start_x = x + 1;
                            start_y = y;
                        }
                        pipe
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let width = pipes[0].len();
        let mut map = vec![vec![Pipe::Extension; width]];
        map.extend(pipes);
        map.push(vec![Pipe::Extension; width]);
        Ok(Self {
            map,
            start_x,
            start_y,
            pipes: OnceCell::new(),
        })
    }
}

impl Position {
    fn transition(&mut self, landscape: &Landscape, pipe: &mut [Vec<usize>]) -> bool {
        pipe[self.x].push(self.y);
        let (next_direction, next_x, next_y) =
            match (self.direction, &landscape.map[self.x][self.y]) {
                (Direction::North, Pipe::N2S) => (Direction::North, self.x + 1, self.y),
                (Direction::South, Pipe::N2S) => (Direction::South, self.x - 1, self.y),
                (Direction::North, Pipe::N2E) => (Direction::West, self.x, self.y + 1),
                (Direction::East, Pipe::N2E) => (Direction::South, self.x - 1, self.y),
                (Direction::East, Pipe::E2W) => (Direction::East, self.x, self.y - 1),
                (Direction::West, Pipe::E2W) => (Direction::West, self.x, self.y + 1),
                (Direction::South, Pipe::S2E) => (Direction::West, self.x, self.y + 1),
                (Direction::East, Pipe::S2E) => (Direction::North, self.x + 1, self.y),
                (Direction::West, Pipe::S2W) => (Direction::North, self.x + 1, self.y),
                (Direction::South, Pipe::S2W) => (Direction::East, self.x, self.y - 1),
                (Direction::West, Pipe::N2W) => (Direction::South, self.x - 1, self.y),
                (Direction::North, Pipe::N2W) => (Direction::East, self.x, self.y - 1),
                (Direction::North, Pipe::Start) => (Direction::North, self.x + 1, self.y),
                (Direction::East, Pipe::Start) => (Direction::East, self.x, self.y - 1),
                (Direction::South, Pipe::Start) => (Direction::South, self.x - 1, self.y),
                (Direction::West, Pipe::Start) => (Direction::West, self.x, self.y + 1),
                (_, Pipe::None) => return false,
                (x, y) => unreachable!("got {x:?} and {y:?}"),
            };
        match (next_direction, &landscape.map[next_x][next_y]) {
            (Direction::North, Pipe::N2S)
            | (Direction::North, Pipe::N2E)
            | (Direction::North, Pipe::N2W)
            | (Direction::East, Pipe::E2W)
            | (Direction::East, Pipe::N2E)
            | (Direction::East, Pipe::S2E)
            | (Direction::West, Pipe::E2W)
            | (Direction::West, Pipe::S2W)
            | (Direction::West, Pipe::N2W)
            | (Direction::South, Pipe::S2W)
            | (Direction::South, Pipe::N2S)
            | (Direction::South, Pipe::S2E) => {
                self.direction = next_direction;
                self.x = next_x;
                self.y = next_y;
                true
            }
            (_, Pipe::Start) => {
                self.x = next_x;
                self.y = next_y;
                false
            }
            _ => {
                self.x = 0;
                self.y = 0;
                false
            }
        }
    }
}

fn try_direction(mut position: Position, landscape: &Landscape) -> Option<usize> {
    let mut pipes = vec![Vec::new(); landscape.map.len() - 1];
    let mut counter = 0;
    loop {
        counter += 1;
        if !position.transition(landscape, &mut pipes) {
            break;
        }
    }
    if landscape.map[position.x][position.y] == Pipe::Start {
        landscape.pipes.set(pipes).unwrap();
        Some(counter)
    } else {
        None
    }
}

fn solve_part1(landscape: &Landscape) -> usize {
    vec![
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ]
    .into_iter()
    .map(|direction| {
        try_direction(
            Position::new(landscape.start_x, landscape.start_y, direction),
            landscape,
        )
    })
    .find(|x| x.is_some())
    .unwrap()
    .unwrap()
        / 2
}

fn solve_part2(landscape: &Landscape) -> usize {
    todo!()
}

pub struct Day10Solver;

impl<'a> AdventOfCodeDay<'a> for Day10Solver {
    type ParsedInput = Landscape;

    type Part1Output = usize;

    type Part2Output = usize;

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
        let input = ".....
        .S-7.
        .|.|.
        .L-J.
        .....";
        let landscape = input.parse::<Landscape>().unwrap();
        assert_eq!(4, solve_part1(&landscape));
        assert_eq!(1, solve_part2(&landscape));
    }

    #[test]
    fn example_2() {
        let input = "
        FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L";
        let landscape = input.parse::<Landscape>().unwrap();
        assert_eq!(80, solve_part1(&landscape));
        assert_eq!(10, solve_part2(&landscape));
    }
    #[test]
    fn example_3() {
        let input = "...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........";
        let landscape = input.parse::<Landscape>().unwrap();
        solve_part1(&landscape);
        assert_eq!(4, solve_part2(&landscape));
    }
    #[test]
    fn test_mine() {
        let input = std::fs::read_to_string("challenge.txt").unwrap();
        let map = input.parse::<Landscape>().unwrap();
        assert_eq!(6909, solve_part1(&map));
        //assert_eq!(6909, solve_part2(&map));
    }

    #[test]
    fn test_simon() {
        let input = std::fs::read_to_string("simon.txt").unwrap();
        let map = input.parse::<Landscape>().unwrap();
        assert_eq!(6701, solve_part1(&map));
    }
}
