use std::str::FromStr;

use aoc_traits::AdventOfCodeDay;

#[derive(Debug)]
pub struct Measuring {
    solution_1: i32,
    solution_2: i32,
}

fn approximate(history: &[i32]) -> (i32, i32) {
    let mut extensions = vec![history.to_vec()];
    let mut last_elements = vec![*history.iter().next_back().unwrap()];
    let mut first_elements = vec![*history.first().unwrap()];
    loop {
        let next_row = extensions
            .iter()
            .next_back()
            .unwrap()
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect::<Vec<_>>();
        last_elements.push(*next_row.iter().next_back().unwrap());
        first_elements.push(*next_row.first().unwrap());
        if next_row.iter().all(|a| *a == 0) {
            extensions.push(next_row);
            break;
        }
        extensions.push(next_row);
    }
    (
        last_elements.into_iter().reduce(|a, b| a + b).unwrap(),
        first_elements
            .into_iter()
            .rev()
            .reduce(|a, b| b - a)
            .unwrap(),
    )
}

fn solve_part1(measuring: &Measuring) -> i32 {
    measuring.solution_1
}

fn solve_part2(measuring: &Measuring) -> i32 {
    measuring.solution_2
}

impl FromStr for Measuring {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let measuring = s
            .trim()
            .lines()
            .map(|s| {
                s.split_ascii_whitespace()
                    .map(|c| c.parse().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let (first, second): (Vec<i32>, Vec<i32>) =
            measuring.iter().map(|x| approximate(x)).unzip();
        Ok(Self {
            solution_1: first.into_iter().sum(),
            solution_2: second.into_iter().sum::<i32>(),
        })
    }
}

pub struct Day9Solver;

impl<'a> AdventOfCodeDay<'a> for Day9Solver {
    type ParsedInput = Measuring;

    type Part1Output = i32;

    type Part2Output = i32;

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
    fn test() {
        let input = "10 13 16 21 30 45";
        let measuring = input.parse::<Measuring>().unwrap();
        assert_eq!(114, solve_part1(&measuring));
    }

    #[test]
    fn example_1() {
        let input = "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";
        let measuring = input.parse::<Measuring>().unwrap();
        assert_eq!(114, solve_part1(&measuring));
        assert_eq!(2, solve_part2(&measuring));
    }

    #[test]
    fn challenge_2() {
        let input = std::fs::read_to_string("challenge.txt").unwrap();
        let map = input.parse::<Measuring>().unwrap();
        assert_eq!(1647269739, solve_part1(&map));
        assert_eq!(864, solve_part2(&map));
    }

    #[test]
    fn daniel() {
        let input = std::fs::read_to_string("daniel.txt").unwrap();
        let map = input.parse::<Measuring>().unwrap();
        assert_eq!(1916822650, solve_part1(&map));
        assert_eq!(966, solve_part2(&map));
    }
}
