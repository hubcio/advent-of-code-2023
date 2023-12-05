use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::ops::Range;
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct SeedsToPlant(Vec<u64>);

impl FromStr for SeedsToPlant {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(SeedsToPlant(
            s.split(':')
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        ))
    }
}

impl SeedsToPlant {
    fn into_rangesets(self) -> Vec<Range<u64>> {
        self.0
            .chunks(2)
            .map(|chunk| chunk[0]..(chunk[0] + chunk[1]))
            .collect()
    }
}

#[derive(Debug)]
struct Mapping {
    destination: Range<u64>,
    source: Range<u64>,
}

impl FromStr for Mapping {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let parts: Vec<u64> = s
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        Ok(Mapping {
            destination: (parts[0]..(parts[0] + parts[2])),
            source: (parts[1]..(parts[1] + parts[2])),
        })
    }
}

#[derive(Debug)]
struct Map {
    mappings: Vec<Mapping>,
}

impl FromStr for Map {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(Map {
            mappings: s
                .lines()
                .skip(1)
                .map(|line| line.parse().unwrap())
                .collect(),
        })
    }
}

impl Map {
    fn map_output(&self, input: u64) -> u64 {
        let mut input = input;
        for mapping in &self.mappings {
            if mapping.source.contains(&input) {
                input = input + mapping.destination.start - mapping.source.start;
                break;
            }
        }

        input
    }

    fn reverse_map_output(&self, input: u64) -> u64 {
        let mut input = input;
        for mapping in self.mappings.iter().rev() {
            if mapping.destination.contains(&input) {
                input = input - mapping.destination.start + mapping.source.start;
                break;
            }
        }
        input
    }
}

#[aoc(day5, part1)]
pub fn part_1(input: &str) -> u64 {
    let mut sections = input.split("\n\n");
    let seeds = SeedsToPlant::from_str(sections.next().unwrap()).unwrap();
    let maps: Vec<Map> = sections.map(|section| section.parse().unwrap()).collect();

    seeds
        .0
        .iter()
        .map(|seed| {
            maps.iter()
                .fold(*seed, |output, map| map.map_output(output))
        })
        .min()
        .unwrap()
}

#[aoc(day5, part2, BruteForce)]
pub fn part_2_brute_force(input: &str) -> u64 {
    let mut sections = input.split("\n\n");

    let seeds = SeedsToPlant::from_str(sections.next().unwrap())
        .unwrap()
        .into_rangesets();

    let maps: Vec<Map> = sections.map(|section| section.parse().unwrap()).collect();
    seeds
        .iter()
        .map(|range| {
            range
                .clone()
                .into_par_iter()
                .map(|seed| maps.iter().fold(seed, |output, map| map.map_output(output)))
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

#[aoc(day5, part2, BruteForce2)]
pub fn part_2_brute_force_2(input: &str) -> u64 {
    let mut sections = input.split("\n\n");

    let seeds = SeedsToPlant::from_str(sections.next().unwrap())
        .unwrap()
        .into_rangesets();

    let mut maps: Vec<Map> = sections.map(|section| section.parse().unwrap()).collect();
    maps.reverse();

    let result = Arc::new(AtomicU64::new(u64::MAX));
    let found = Arc::new(Mutex::new(false));

    (0u64..0xFFFF).into_par_iter().for_each_with(
        (result.clone(), found.clone()),
        |(result, found), location| {
            let seed = maps
                .iter()
                .fold(location, |input, map| map.reverse_map_output(input));

            if seeds.iter().any(|range| range.contains(&seed)) {
                let mut found = found.lock().unwrap();
                if !*found {
                    *found = true;
                    result.store(location, Ordering::SeqCst);
                }
            }
        },
    );

    result.load(Ordering::SeqCst)
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(SAMPLE), 35);
    }

    #[test]
    fn test_part_2_brute_force() {
        assert_eq!(part_2_brute_force(SAMPLE), 46);
    }

    #[test]
    fn test_part_2_brute_force_2() {
        assert_eq!(part_2_brute_force_2(SAMPLE), 46);
    }
}
