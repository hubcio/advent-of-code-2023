use rayon::prelude::*;
use std::{collections::HashMap, str::FromStr};

use crate::helpers::lcm;

#[derive(Debug)]
struct Map {
    directions: Vec<usize>,
    nodes: Vec<[usize; 2]>,
    start_indices: Vec<usize>,
    end_indices: Vec<usize>,
}

impl Map {
    fn instructions(&self, start: usize, end: Option<usize>) -> usize {
        let number_of_directions = self.directions.len();
        let mut count = 0;
        let mut index = start;

        if let Some(end_index) = end {
            while index != end_index {
                index = self.nodes[index][self.directions[count % number_of_directions]];
                count += 1;
            }
        } else {
            while !self.end_indices.contains(&index) {
                index = self.nodes[index][self.directions[count % number_of_directions]];
                count += 1;
            }
        }

        count
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let directions = lines
            .next()
            .unwrap()
            .chars()
            .map(|d| (d == 'R') as usize)
            .collect();

        let mut node_names: Vec<&str> = lines
            .skip(1)
            .map(|l| l.split_once(" =").unwrap().0)
            .collect();

        node_names.sort_unstable();

        let mut start_indices = Vec::new();
        let mut end_indices = Vec::new();
        let mut idx_by_node: HashMap<&str, usize> = HashMap::new();

        node_names.iter().enumerate().for_each(|(i, n)| {
            if n.ends_with('A') {
                start_indices.push(i);
            } else if n.ends_with('Z') {
                end_indices.push(i);
            }
            idx_by_node.insert(n, i);
        });

        let mut nodes = vec![[0, 0]; node_names.len()];

        for l in s.lines().skip(2) {
            let (from, rest) = l.split_once(" = (").unwrap();
            let (l, rr) = rest.split_once(", ").unwrap();
            let r = &rr[0..3];
            nodes[idx_by_node[from]] = [idx_by_node[l], idx_by_node[r]];
        }

        Ok(Map {
            directions,
            nodes,
            start_indices,
            end_indices,
        })
    }
}

#[aoc(day8, part1)]
fn camels_map(input: &str) -> usize {
    let map = Map::from_str(input).unwrap();

    map.instructions(0, Some(map.nodes.len() - 1))
}

#[aoc(day8, part2)]
fn ghosts_map(input: &str) -> usize {
    let map = Map::from_str(input).unwrap();

    map.start_indices
        .par_iter()
        .map(|&start| map.instructions(start, None))
        .reduce(|| 1, lcm)
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_1: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

    static SAMPLE_2: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

    static SAMPLE_3: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

    #[test]
    fn test_camels_map_1() {
        assert_eq!(camels_map(SAMPLE_1), 2);
    }

    #[test]
    fn test_camels_map_2() {
        assert_eq!(camels_map(SAMPLE_2), 6 + 1);
    }

    #[test]
    fn test_ghosts_map() {
        assert_eq!(ghosts_map(SAMPLE_3), 6 + 1);
    }
}
