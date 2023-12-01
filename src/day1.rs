#[aoc(day1, part1, FirstSolutionButProbablyBad)]
pub fn part1_bad(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let first_digit = line.chars().find(|c| c.is_ascii_digit()).unwrap();
            let last_digit = line.chars().rev().find(|c| c.is_ascii_digit()).unwrap();
            let mut number = String::new();
            number.push(first_digit);
            number.push(last_digit);
            number.parse::<u32>().unwrap()
        })
        .sum()
}

#[aoc(day1, part1, Maybe)]
pub fn part1_maybe(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let first_digit = line.chars().find(|c| c.is_ascii_digit());
            let last_digit = line.chars().rev().find(|c| c.is_ascii_digit());

            match (first_digit, last_digit) {
                (Some(f), Some(l)) if f != l => {
                    Some(f.to_digit(10).unwrap() * 10 + l.to_digit(10).unwrap())
                }
                (Some(d), _) | (_, Some(d)) => {
                    let digit = d.to_digit(10).unwrap();
                    Some(digit * 10 + digit)
                }
                _ => None,
            }
        })
        .sum()
}

#[aoc(day1, part1, HorribleShit)]
pub fn part1_shit(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut first_digit: Option<char> = None;
            let mut last_digit: Option<char> = None;
            for character in line.chars() {
                if character.is_ascii_digit() && first_digit.is_none() {
                    first_digit = Some(character);
                }
                if character.is_ascii_digit() {
                    last_digit = Some(character);
                }
            }

            let mut number = String::new();
            number.push(first_digit.unwrap());
            number.push(last_digit.unwrap());
            number.parse::<u32>().unwrap()
        })
        .sum()
}

#[derive(Debug, PartialEq)]
enum Number {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Number {
    fn from(s: &str) -> Option<Self> {
        match s {
            s if s.contains('1') || s.contains("one") => Some(Number::One),
            s if s.contains('2') || s.contains("two") => Some(Number::Two),
            s if s.contains('3') || s.contains("three") => Some(Number::Three),
            s if s.contains('4') || s.contains("four") => Some(Number::Four),
            s if s.contains('5') || s.contains("five") => Some(Number::Five),
            s if s.contains('6') || s.contains("six") => Some(Number::Six),
            s if s.contains('7') || s.contains("seven") => Some(Number::Seven),
            s if s.contains('8') || s.contains("eight") => Some(Number::Eight),
            s if s.contains('9') || s.contains("nine") => Some(Number::Nine),
            _ => None,
        }
    }

    fn as_digit_char(&self) -> char {
        match self {
            Number::One => '1',
            Number::Two => '2',
            Number::Three => '3',
            Number::Four => '4',
            Number::Five => '5',
            Number::Six => '6',
            Number::Seven => '7',
            Number::Eight => '8',
            Number::Nine => '9',
        }
    }
}

#[aoc(day1, part2, FirstSolutionButProbablyBad)]
pub fn part2_first(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut first_digit = String::new();
            let mut first_digit_value = '0';
            for c in line.chars() {
                first_digit.push(c);
                let number = Number::from(&first_digit);
                if number.is_some() {
                    first_digit_value = number.unwrap().as_digit_char();
                    break;
                }
            }

            let mut last_digit = String::new();
            let mut last_digit_value = '0';
            for c in line.chars().rev() {
                last_digit.push(c);
                let last_digit = last_digit.chars().rev().collect::<String>();
                let number = Number::from(&last_digit);
                if number.is_some() {
                    last_digit_value = number.unwrap().as_digit_char();
                    break;
                }
            }

            let mut number = String::new();
            number.push(first_digit_value);
            number.push(last_digit_value);
            number.parse::<u32>().unwrap()
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::{part1_bad, part1_maybe, part1_shit, part2_first};

    static SAMPLE_1: &str = r#"1abc2
    pqr3stu8vwx
    a1b2c3d4e5fd
    treb7uchet"#;

    static SAMPLE_2: &str = r#"two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen"#;

    #[test]
    fn test_part_1_bad() {
        assert_eq!(part1_bad(SAMPLE_1), 142);
    }

    #[test]
    fn test_part_1_maybe() {
        assert_eq!(part1_maybe(SAMPLE_1), 142);
    }
    #[test]
    fn test_part_1_shit() {
        assert_eq!(part1_shit(SAMPLE_1), 142);
    }

    #[test]
    fn test_part_2_first() {
        assert_eq!(part2_first(SAMPLE_2), 281);
    }
}
