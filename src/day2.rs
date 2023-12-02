use rayon::prelude::*;

#[derive(Debug, PartialEq)]
enum Color {
    Red(usize),
    Green(usize),
    Blue(usize),
}

impl Color {
    fn from(s: &str) -> Option<Self> {
        let mut parts = s.split_whitespace();
        let number = parts.next()?.parse::<usize>().ok()?;
        match parts.next()?.chars().next()? {
            'r' => Some(Color::Red(number)),
            'g' => Some(Color::Green(number)),
            'b' => Some(Color::Blue(number)),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct SubGame {
    reds: Option<Color>,
    greens: Option<Color>,
    blues: Option<Color>,
}

impl From<&str> for SubGame {
    fn from(s: &str) -> Self {
        let mut reds = None;
        let mut greens = None;
        let mut blues = None;
        for cube in s.split(',') {
            if let Some(cube) = Color::from(cube) {
                match cube {
                    Color::Red(_) => reds = Some(cube),
                    Color::Green(_) => greens = Some(cube),
                    Color::Blue(_) => blues = Some(cube),
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
    cube_subsets: [Option<SubGame>; 5],
}

impl From<&str> for GameData {
    fn from(line: &str) -> Self {
        let colon_index = line.find(':').unwrap();
        let id_str = &line[5..colon_index];
        let id = id_str.trim().parse::<usize>().unwrap();
        let cube_data_str = &line[colon_index + 1..];

        let mut cube_subsets = [None, None, None, None, None];
        for (index, cube_subset_str) in cube_data_str.split(';').enumerate() {
            if index < 5 {
                cube_subsets[index] = Some(SubGame::from(cube_subset_str));
            } else {
                break;
            }
        }
        GameData {
            id,
            cube_subsets,
        }
    }
}

impl GameData {
    fn check(&self, available_cubes: &AvailableCubes) -> usize {
        for cube_subset in self.cube_subsets.iter().filter_map(|cs| cs.as_ref()) {
            if !self.has_enough_cubes(cube_subset, available_cubes) {
                return 0;
            }
        }
        self.id
    }

    fn has_enough_cubes(&self, cube_subset: &SubGame, available_cubes: &AvailableCubes) -> bool {
        match &cube_subset.reds {
            Some(Color::Red(number)) if available_cubes.red < *number => return false,
            _ => (),
        }
        match &cube_subset.greens {
            Some(Color::Green(number)) if available_cubes.green < *number => return false,
            _ => (),
        }
        match &cube_subset.blues {
            Some(Color::Blue(number)) if available_cubes.blue < *number => return false,
            _ => (),
        }
        true
    }

    fn calculate_minimum_cubes_mul(&self) -> usize {
        let mut reds = 0;
        let mut greens = 0;
        let mut blues = 0;

        for cube_subset in self.cube_subsets.iter().filter_map(|cube_subset| cube_subset.as_ref()) {
            if let Some(Color::Red(number)) = cube_subset.reds {
                if reds < number {
                    reds = number;
                }
            }

            if let Some(color) = &cube_subset.greens {
                if let Color::Green(number) = color {
                    if greens < *number {
                        greens = *number;
                    }
                }
            }

            if let Some(color) = &cube_subset.blues {
                if let Color::Blue(number) = color {
                    if blues < *number {
                        blues = *number;
                    }
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
        .par_lines()
        .map(|line| {
            let game_data = GameData::from(line);
            game_data.check(&available_cubes)
        }).sum::<usize>()
}

#[aoc(day2, part2, FirstTry)]
pub fn part2_first(input: &str) -> usize {
    input
        .par_lines()
        .map(|line| {
            let game_data = GameData::from(line);
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
