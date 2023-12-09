use rayon::{iter::ParallelIterator, str::ParallelString};
use smallvec::SmallVec;

#[aoc(day9, part1)]
fn extrapolate(input: &str) -> i64 {
    input
        .par_lines()
        .map(|line| {
            let mut nums: Vec<i64> = line
                .split_ascii_whitespace()
                .map(|b| atoi::atoi::<i64>(b.as_bytes()).unwrap())
                .collect();

            (0..nums.len())
                .rev()
                .map(|len| {
                    (0..len).for_each(|i| {
                        let slice = &mut nums[i..=i + 1];
                        slice[0] = slice[1] - slice[0];
                    });
                    nums[len]
                })
                .sum::<i64>()
        })
        .sum()
}

#[aoc(day9, part2)]
fn extrapolate_backwards(input: &str) -> i64 {
    input
        .par_lines()
        .map(|line| {
            let mut nums: SmallVec<[i64; 20]> = line
                .split_ascii_whitespace()
                .map(|b| atoi::atoi::<i64>(b.as_bytes()).unwrap())
                .collect();

            nums.reverse();

            (0..nums.len())
                .rev()
                .map(|len| {
                    (0..len).for_each(|i| {
                        let slice = &mut nums[i..=i + 1];
                        slice[0] = slice[1] - slice[0];
                    });
                    nums[len]
                })
                .sum::<i64>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

    static SAMPLE_1: &str = r#"0 3 6 9 12 15"#;
    static SAMPLE_2: &str = r#"1 3 6 10 15 21"#;
    static SAMPLE_3: &str = r#"10 13 16 21 30 45"#;

    #[test]
    fn test_extrapolate() {
        assert_eq!(extrapolate(SAMPLE), 114);
    }

    #[test]
    fn test_extrapolate_1() {
        assert_eq!(extrapolate_backwards(SAMPLE_1), -3);
    }

    #[test]
    fn test_extrapolate_2() {
        assert_eq!(extrapolate_backwards(SAMPLE_2), 0);
    }

    #[test]
    fn test_extrapolate_3() {
        assert_eq!(extrapolate_backwards(SAMPLE_3), 5);
    }

    #[test]
    fn test_extrapolate_backwards() {
        assert_eq!(extrapolate_backwards(SAMPLE), 2);
    }
}
