use aoc_traits::AdventOfCodeDay;
use itertools::Itertools;

fn hash(s: &str) -> u32 {
    s.chars().map(|c| c as u8).fold(0_u32, |mut acc, c| {
        acc += c as u32;
        acc *= 17;
        acc %= 256;
        acc
    })
}

fn put_in_box(instruction: &str, boxes: &mut [Vec<(String, u32)>]) {
    let mut chars = instruction.chars();
    let label = chars
        .peeking_take_while(|c| *c != '=' && *c != '-')
        .collect::<String>();
    let hash = label.chars().map(|c| c as u8).fold(0_usize, |mut acc, c| {
        acc += c as usize;
        acc *= 17;
        acc %= 256;
        acc
    });
    let current_box = boxes.get_mut(hash).unwrap();
    match chars.next().unwrap() {
        '=' => {
            let lens = chars.next().unwrap().to_digit(10).unwrap();
            if let Some(pos) = current_box.iter().position(|x| *x.0 == label) {
                *current_box.get_mut(pos).unwrap() = (label, lens);
            } else {
                current_box.push((label, lens));
            }
        }
        '-' => {
            if let Some(pos) = current_box.iter().position(|x| *x.0 == label) {
                current_box.remove(pos);
            }
        }
        _ => unreachable!(),
    }
}

fn solve_part1(s: &str) -> u32 {
    s.trim().split(',').map(hash).sum::<u32>()
}

fn solve_part2(s: &str) -> u32 {
    let mut boxes = std::iter::repeat(vec![]).take(256).collect::<Vec<_>>();
    s.trim().split(',').for_each(|s| put_in_box(s, &mut boxes));
    boxes
        .into_iter()
        .enumerate()
        .map(|(idx, b)| {
            b.into_iter()
                .enumerate()
                .map(|(slot, (_, lens))| (idx as u32 + 1) * (slot as u32 + 1) * lens)
                .sum::<u32>()
        })
        .sum()
}

pub struct Day15Solver;

impl<'a> AdventOfCodeDay<'a> for Day15Solver {
    type ParsedInput = &'a str;

    type Part1Output = u32;

    type Part2Output = u32;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        solve_part1(input)
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        solve_part2(input)
    }

    fn parse_input(input: &'a str) -> Self::ParsedInput {
        input
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            1320,
            solve_part1("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7")
        );
        let challenge = fs::read_to_string("input.txt").unwrap();
        assert_eq!(511343, solve_part1(&challenge));
    }

    #[test]
    fn example_2() {
        let challenge = fs::read_to_string("input.txt").unwrap();
        assert_eq!(294474, solve_part2(&challenge));
        assert_eq!(
            145,
            solve_part2("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7")
        );
    }
}
