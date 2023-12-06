use std::str::FromStr;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

// th = time spent holding button (ms)
// tr = total race time (ms)
// v = velocity (mm/s)
// d = distance (mm)
// v = th (time spent holding button is equal to velocity)
// d = th * (tr - th)

#[derive(Debug)]
struct Race {
    total_race_time_ms: u64,
    record_distance_mm: u64,
}

impl Race {
    pub fn calculate_number_of_ways_to_beat_the_record(&self) -> u64 {
        (1..self.total_race_time_ms)
            .filter(|th| self.calculate_distance(*th) > self.record_distance_mm)
            .count() as u64
    }

    pub fn calculate_number_of_ways_to_beat_the_record_parallel(&self) -> u64 {
        (1..self.total_race_time_ms)
            .into_par_iter()
            .filter_map(|th| {
                let distance = self.calculate_distance(th);
                if distance > self.record_distance_mm {
                    Some(1)
                } else {
                    None
                }
            })
            .count() as u64
    }

    pub fn calculate_number_of_ways_to_beat_the_record_parallel_midpoint(&self) -> u64 {
        let midpoint = self.total_race_time_ms / 2;

        let count_up_to_midpoint = (1..=midpoint)
            .into_par_iter()
            .filter_map(|th| {
                let distance = self.calculate_distance(th);
                if distance > self.record_distance_mm {
                    Some(1)
                } else {
                    None
                }
            })
            .count() as u64;

        if self.total_race_time_ms % 2 == 0 {
            2 * count_up_to_midpoint - 1
        } else {
            2 * count_up_to_midpoint
        }
    }

    pub fn calculate_number_of_ways_to_beat_the_record_equation(&self) -> u64 {
        // y = ax^2 + bx + c
        // d = −th^2 + tr * th
        // −th^2 + tr * th > record distance

        let a = -1;
        let b = self.total_race_time_ms as i64;
        let c = -(self.record_distance_mm as i64);

        let discriminant = b.pow(2) - 4 * a * c;
        if discriminant < 0 {
            return 0;
        }

        let sqrt_discriminant = (discriminant as f64).sqrt() as i64;
        let root1 = (-b - sqrt_discriminant) / (2 * a);
        let root2 = (-b + sqrt_discriminant) / (2 * a);

        (root1 - root2) as u64
    }

    fn calculate_distance(&self, th: u64) -> u64 {
        th * (self.total_race_time_ms - th)
    }
}

impl FromStr for Race {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let lines: Vec<&str> = s.lines().collect();

        let time = lines[0][11..].replace(' ', "").parse::<u64>().unwrap();
        let distance = lines[1][11..].replace(' ', "").parse::<u64>().unwrap();

        Ok(Race {
            total_race_time_ms: time,
            record_distance_mm: distance,
        })
    }
}

struct Races(Vec<Race>);

impl Races {
    pub fn mul_ways_of_beating_records(&self) -> u64 {
        self.0
            .iter()
            .map(|r| r.calculate_number_of_ways_to_beat_the_record())
            .product()
    }
}

impl FromStr for Races {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let lines: Vec<&str> = s.lines().collect();

        let times: Vec<u64> = lines[0][11..]
            .split_whitespace()
            .map(|num| num.parse::<u64>().unwrap())
            .collect();

        let distances: Vec<u64> = lines[1][11..]
            .split_whitespace()
            .map(|num| num.parse::<u64>().unwrap())
            .collect();

        let races = times
            .into_iter()
            .zip(distances)
            .map(|(time, distance)| Race {
                total_race_time_ms: time,
                record_distance_mm: distance,
            })
            .collect();

        Ok(Races(races))
    }
}

#[aoc(day6, part1)]
pub fn part_1(input: &str) -> u64 {
    let races = Races::from_str(input).unwrap();
    races.mul_ways_of_beating_records()
}

#[aoc(day6, part2, Parallel)]
pub fn part_2_parallel(input: &str) -> u64 {
    let race = Race::from_str(input).unwrap();
    race.calculate_number_of_ways_to_beat_the_record_parallel()
}

#[aoc(day6, part2, Midpoint)]
pub fn part_2_midpoint(input: &str) -> u64 {
    let race = Race::from_str(input).unwrap();
    race.calculate_number_of_ways_to_beat_the_record_parallel_midpoint()
}

#[aoc(day6, part2, Equation)]
pub fn part_2_equation(input: &str) -> u64 {
    let race = Race::from_str(input).unwrap();
    race.calculate_number_of_ways_to_beat_the_record_equation()
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = r#"Time:      7  15   30
Distance:  9  40  200
    "#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(SAMPLE), 288);
    }

    #[test]
    fn test_part_2_parallel() {
        assert_eq!(part_2_parallel(SAMPLE), 71503);
    }

    #[test]
    fn test_part_2_midpoint() {
        assert_eq!(part_2_midpoint(SAMPLE), 71503);
    }

    #[test]
    fn test_part_2_equation() {
        assert_eq!(part_2_equation(SAMPLE), 71503);
    }
}
