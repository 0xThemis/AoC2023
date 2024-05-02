type Platform = Vec<Vec<Field>>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Field {
    Stone,
    Barricade,
    Empty,
}

impl From<char> for Field {
    fn from(c: char) -> Self {
        match c {
            'O' => Field::Stone,
            '#' => Field::Barricade,
            _ => Field::Empty,
        }
    }
}
fn transpose(matrix: &Platform) -> Platform {
    if matrix.is_empty() {
        return Vec::new();
    }

    let num_rows = matrix.len();
    let num_cols = matrix[0].len();

    (0..num_cols)
        .map(|col| (0..num_rows).map(|row| matrix[row][col].clone()).collect())
        .collect()
}

fn solve_part1(platform: &Platform) -> u32 {
    transpose(&platform)
        .into_iter()
        .map(|row| {
            row.into_iter()
                .take_while(|field| field != &Field::Barricade)
                .filter(|field| field == &Field::Stone)
                .enumerate()
                .map(|(idx, _)| {
                    println!("it is {}", idx);
                    10 - idx as u32
                })
                .sum::<u32>()
        })
        .sum()
}

fn parse(s: &str) -> Platform {
    s.trim()
        .lines()
        .map(|line| line.trim().chars().map(Field::from).collect::<Vec<_>>())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_1() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let platform = parse(input);
        println!("{}", solve_part1(&platform))
    }
}
