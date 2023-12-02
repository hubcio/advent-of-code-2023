#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn from(s: &str) -> Option<Self> {
        match s {
            "red" => Some(Color::Red),
            "green" => Some(Color::Green),
            "blue" => Some(Color::Blue),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Cube {
    color_type: Color,
    number: usize,
}

impl Cube {
    fn from(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 2 {
            eprintln!("Invalid color: {}", s);
            return None;
        }

        let number = parts[0].parse::<usize>();
        if number.is_err() {
            eprintln!("Invalid number: {}", parts[0]);
            return None;
        }

        let color_type = Color::from(parts[1]);
        if color_type.is_none() {
            eprintln!("Invalid color type: {}", parts[1]);
            return None;
        }
        Some(Cube {
            color_type: color_type.unwrap(),
            number: number.unwrap(),
        })
    }
}

#[derive(Debug)]
struct SubGame {
    reds: Option<Cube>,
    greens: Option<Cube>,
    blues: Option<Cube>,
}

impl From<&str> for SubGame {
    fn from(s: &str) -> Self {
        let mut reds = None;
        let mut greens = None;
        let mut blues = None;
        for cube in s.split(',') {
            if let Some(cube) = Cube::from(cube) {
                match cube.color_type {
                    Color::Red => reds = Some(cube),
                    Color::Green => greens = Some(cube),
                    Color::Blue => blues = Some(cube),
                }
            }
        }
        SubGame {
            reds,
            greens,
            blues,
        }
    }
}

struct AvailableCubes {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug)]
struct GameData {
    id: usize,
    cube_subsets: Vec<SubGame>,
}

impl From<(&str, usize)> for GameData {
    fn from(game_data: (&str, usize)) -> Self {
        let line = game_data.0.find(':').map(|i| &game_data.0[i + 1..]).unwrap();

        let mut cube_subsets = vec![];
        for cube_subset_str in line.split(';') {
            cube_subsets.push(SubGame::from(cube_subset_str));
        }
        GameData {
            id: game_data.1,
            cube_subsets,
        }
    }
}

impl GameData {
    fn check(&self, available_cubes: &AvailableCubes) -> usize {
        for cube_subset in &self.cube_subsets {
            if let Some(cube) = &cube_subset.reds {
                if cube.number > available_cubes.red {
                    return 0;
                }
            }

            if let Some(cube) = &cube_subset.greens {
                if cube.number > available_cubes.green {
                    return 0;
                }
            }

            if let Some(cube) = &cube_subset.blues {
                if cube.number > available_cubes.blue {
                    return 0;
                }
            }
        }
        self.id
    }

    fn calculate_minimum_cubes_mul(&self) -> usize {
        let mut reds = 0;
        let mut greens = 0;
        let mut blues = 0;
        for cube_subset in &self.cube_subsets {
            if let Some(cube) = &cube_subset.reds {
                if reds < cube.number {
                    reds = cube.number;
                }
            }

            if let Some(cube) = &cube_subset.greens {
                if greens < cube.number {
                    greens = cube.number;
                }
            }

            if let Some(cube) = &cube_subset.blues {
                if blues < cube.number {
                    blues = cube.number;
                }
            }
        }

        reds * greens * blues
    }
}

#[aoc(day2, part1, FirstTry)]
pub fn part1_first(input: &str) -> usize {
    let available_cubes = AvailableCubes {
        red: 12,
        green: 13,
        blue: 14,
    };

    input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let game_data = GameData::from((line, index + 1));
            game_data.check(&available_cubes)
        }).sum::<usize>()
}

#[aoc(day2, part2, FirstTry)]
pub fn part2_first(input: &str) -> usize {
    input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let game_data = GameData::from((line, index + 1));
            game_data.calculate_minimum_cubes_mul()
        }).sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::{part1_first, part2_first};

    static SAMPLE: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn test_part_1_first() {
        assert_eq!(part1_first(SAMPLE), 8);
    }

    #[test]
    fn test_part_2_first() {
        assert_eq!(part2_first(SAMPLE), 2286);
    }
}
