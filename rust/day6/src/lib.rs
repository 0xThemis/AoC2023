use aoc_traits::AdventOfCodeDay;

pub type SheetOfPaper = Vec<BoatRace>;
pub type KerningSheetOfPaper = BoatRace;
#[derive(Debug)]
pub struct BoatRace {
    time: u64,
    distance: u64,
}

pub struct InputDay6 {
    challenge_1: SheetOfPaper,
    challenge_2: BoatRace,
}

fn day6_challenge1(input: &InputDay6) -> u32 {
    input
        .challenge_1
        .iter()
        .map(|race| {
            (1..race.time)
                .map(|press| ((press) * (race.time - press)))
                .filter(|distance| distance > &race.distance)
                .count() as u32
        })
        .product()
}
fn day6_challenge2(input: &InputDay6) -> u32 {
    let race = &input.challenge_2;
    (1..race.time)
        .map(|press| ((press) * (race.time - press)))
        .filter(|distance| distance > &race.distance)
        .count() as u32
}

impl From<(u64, u64)> for BoatRace {
    fn from((time, distance): (u64, u64)) -> Self {
        Self { time, distance }
    }
}

pub struct Day6Solver;

impl<'a> AdventOfCodeDay<'a> for Day6Solver {
    type ParsedInput = InputDay6;

    type Part1Output = u32;

    type Part2Output = u32;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        day6_challenge1(input)
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        day6_challenge2(input)
    }

    fn parse_input(input: &'a str) -> Self::ParsedInput {
        let mut lines = input.trim().lines();
        let time = lines.next().unwrap();
        let time = time[time.find(':').unwrap() + 1..]
            .trim()
            .split_ascii_whitespace();
        let distance = lines.next().unwrap();
        let distance = distance[distance.find(':').unwrap() + 1..]
            .trim()
            .split_ascii_whitespace();
        let mut kernel_time = String::new();
        let mut kernel_distance = String::new();
        let sheet_of_paper = time
            .zip(distance)
            .map(|(a, b)| {
                kernel_time.push_str(a);
                kernel_distance.push_str(b);
                BoatRace::from((a.parse().unwrap(), b.parse().unwrap()))
            })
            .collect::<SheetOfPaper>();
        Self::ParsedInput {
            challenge_1: sheet_of_paper,
            challenge_2: BoatRace::from((
                kernel_time.parse().unwrap(),
                kernel_distance.parse().unwrap(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "Time:      7  15   30
        Distance:  9  40  200";
        let sheet_of_paper = Day6Solver::parse_input(input);
        assert_eq!(288, Day6Solver::solve_part1(&sheet_of_paper));
        assert_eq!(71503, Day6Solver::solve_part2(&sheet_of_paper));
    }

    #[test]
    fn challenge() {
        let input = "Time:        46     80     78     66
        Distance:   214   1177   1402   1024";
        let sheet_of_paper = Day6Solver::parse_input(input);
        assert_eq!(512295, Day6Solver::solve_part1(&sheet_of_paper));
        assert_eq!(36530883, Day6Solver::solve_part2(&sheet_of_paper));
    }
}
