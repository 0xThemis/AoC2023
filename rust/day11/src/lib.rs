use std::str::FromStr;

type Coords = (usize, usize);

#[derive(Debug, Clone, PartialEq)]
enum Kind {
    Galaxy(Coords),
    Empty,
}

impl Kind {
    fn new(coords: Coords, character: char) -> Self {
        if character == '#' {
            Kind::Galaxy(coords)
        } else {
            Kind::Empty
        }
    }
}

pub struct Observation {
    map: Vec<Vec<Kind>>,
}

impl FromStr for Observation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //let mut current_line = vec![];
        //for (x, line) in s.trim().lines().enumerate() {
        //    let mut has_galaxy = false;
        //    for (y, c) in line.trim().chars().enumerate() {
        //        let kind = Kind::new((x, y), c);
        //        has_galaxy |= matches!(kind, Kind::Galaxy(_));
        //        current_line.push(Kind::new((x, y), c));
        //    }
        //    if !has_galaxy {
        //        map.push(current_line.clone());
        //    }
        //    map.push(current_line.clone());
        //    current_line.clear();
        //}

        let mut observation = s
            .trim()
            .lines()
            .map(|line| line.trim().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let empty_space = vec!['.'; observation[0].len()];
        let expansion_x = observation
            .iter()
            .enumerate()
            .filter(|(_, line)| line.iter().all(|c| *c == '.'))
            .map(|(idx, _)| idx)
            .collect::<Vec<_>>();
        expansion_x.into_iter().for_each(|idx| {
            observation.insert(idx, empty_space.clone());
        });
        for i in 0..observation[0].len() {
            let mut has_galaxy = false;
            for j in 0..observation.len() {
                has_galaxy |= observation[j][i] == '#';
            }
            if !has_galaxy {
                for j in 0..observation.len() {
                    observation[j].insert(i, '.');
                }
            }
        }
        observation.iter().for_each(|row| {
            println!("{row:?}");
        });

        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....";
        input.parse::<Observation>().unwrap();
    }
}
