use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fmt;

const ADJACENCY_CORDS: &[isize] = &[-1, 0, 1];

#[derive(Debug, Hash, Eq, PartialEq)]
struct Coords {
    x: usize,
    y: usize,
}

impl fmt::Display for Coords {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Default)]
struct Numbers(HashMap<Coords, u32>);

#[derive(Debug, Default)]
struct Symbols(HashMap<Coords, char>);

#[derive(Debug)]
struct Grid {
    numbers: Numbers,
    symbols: Symbols,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut numbers = Numbers::default();
        let mut symbols = Symbols::default();

        for (y, line) in input.lines().enumerate() {
            let mut current_number = String::new();
            let mut start_x = 0;

            for (x, ch) in line.chars().enumerate() {
                match ch {
                    '0'..='9' => {
                        if current_number.is_empty() {
                            start_x = x;
                        }
                        current_number.push(ch);
                    }
                    '.' => {
                        if !current_number.is_empty() {
                            if let Ok(num) = current_number.parse::<u32>() {
                                for offset in 0..current_number.len() {
                                    let x = start_x + offset;
                                    numbers.0.insert(Coords { x, y }, num);
                                }
                            }
                            current_number.clear();
                        }
                    }
                    _ => {
                        // Symbol other than '.'
                        symbols.0.insert(Coords { x, y }, ch);

                        if !current_number.is_empty() {
                            if let Ok(num) = current_number.parse::<u32>() {
                                for offset in 0..current_number.len() {
                                    let x = start_x + offset;
                                    numbers.0.insert(Coords { x, y }, num);
                                }
                            }
                            current_number.clear();
                        }
                    }
                }
            }

            // Line ends with number
            if !current_number.is_empty() {
                if let Ok(num) = current_number.parse::<u32>() {
                    for offset in 0..current_number.len() {
                        numbers.0.insert(
                            Coords {
                                x: start_x + offset,
                                y,
                            },
                            num,
                        );
                    }
                }
            }
        }
        Self { numbers, symbols }
    }

    /// Iterate over symbols and find adjacent numbers (even diagonally), sum them up and return
    fn find_sum_of_numbers_adjacent_to_symbols(&self) -> u32 {
        self.symbols
            .0
            .par_iter()
            .map(|(coords, _)| {
                let mut adjacent_numbers: HashSet<u32> = HashSet::new();
                for &dy in ADJACENCY_CORDS {
                    for &dx in ADJACENCY_CORDS {
                        if dy == 0 && dx == 0 {
                            continue;
                        }
                        let coords = Coords {
                            x: ((coords.x as isize) + dx) as usize,
                            y: ((coords.y as isize) + dy) as usize,
                        };
                        if let Some(&number) = self.numbers.0.get(&coords) {
                            adjacent_numbers.insert(number);
                        }
                    }
                }
                adjacent_numbers.iter().sum::<u32>()
            })
            .sum()
    }

    /// Iterate over '*' symbols and find the ones that are adjacent to exactly
    /// two numbers and multiply them, return sum of all such products
    fn find_sum_of_gear_ratios(&self) -> u32 {
        self.symbols
            .0
            .par_iter()
            .filter_map(|(coords, ch)| {
                if *ch == '*' {
                    let mut adjacent_numbers: HashSet<u32> = HashSet::new();

                    for &dy in ADJACENCY_CORDS {
                        for &dx in ADJACENCY_CORDS {
                            if dy == 0 && dx == 0 {
                                continue;
                            }
                            let coords = Coords {
                                x: ((coords.x as isize) + dx) as usize,
                                y: ((coords.y as isize) + dy) as usize,
                            };
                            if let Some(&number) = self.numbers.0.get(&coords) {
                                adjacent_numbers.insert(number);
                            }
                        }
                    }
                    if adjacent_numbers.len() == 2 {
                        let mut adjacent_numbers = adjacent_numbers.iter();
                        let first = adjacent_numbers.next().unwrap();
                        let second = adjacent_numbers.next().unwrap();
                        Some(first * second)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .sum()
    }
}

#[aoc(day3, part1, ParseAndStoreEverything)]
pub fn part1(input: &str) -> u32 {
    Grid::new(input).find_sum_of_numbers_adjacent_to_symbols()
}

#[aoc(day3, part2, ParseAndStoreEverything)]
pub fn part2(input: &str) -> u32 {
    Grid::new(input).find_sum_of_gear_ratios()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    static SAMPLE: &str = r#"467..114..
...*.....
..35..633.
......#..
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    static SAMPLE_LONGER: &str = r#"..............423....688..934............970................................95.728..........896...113..................153..972.............
...122..................*.....*..........................919..509*..........&...@.........../...........................+.......*...........
....+..........259....698..373.992.52.674.........................781...22........130.584.....-...%399.......777.................266........"#;

    static SAMPLE_LONGBOI: &str = r#"............&...425+..-.............207........*.....%...360...........190..........69.....................43..........$....................
.....................116....................127.214.........*..181........@..%...49*....444....57.........*.....................715.........
............$....*.............-..859.................=..248......*..........82........*.......@.......246..376.........#........*..........
...726...898...-..772...223.712....%........%464.....342.........442..447.........461..102.....................*355...537....313..596...584."#;

    #[test]
    fn test_part_1() {
        assert_eq!(part1(SAMPLE), 4361);
    }

    #[test]
    fn test_part_1_long() {
        assert_eq!(part1(SAMPLE_LONGER), 6700);
    }

    #[test]
    fn test_part_1_longboi() {
        assert_eq!(part1(SAMPLE_LONGBOI), 10021);
    }

    #[test]
    fn test_part_2_first() {
        assert_eq!(part2(SAMPLE), 467835);
    }
}

impl fmt::Display for Numbers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (coords, number) in &self.0 {
            writeln!(f, "{} -> {}", coords, number)?;
        }
        Ok(())
    }
}

impl fmt::Display for Symbols {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (coords, ch) in &self.0 {
            writeln!(f, "{coords} -> '{ch}'")?;
        }
        Ok(())
    }
}
