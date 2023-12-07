use core::{fmt, panic};
use std::{cmp::Ordering, collections::BTreeSet};

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Copy, Clone)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Cards([u8; 5]);

impl From<(&str, bool)> for Cards {
    fn from((line, use_wildcard): (&str, bool)) -> Self {
        let mut cards = Cards([0; 5]);
        line.chars().take(5).enumerate().for_each(|(index, ch)| {
            cards.0[index] = if use_wildcard {
                match ch {
                    'J' => 0,
                    '2' => 1,
                    '3' => 2,
                    '4' => 3,
                    '5' => 4,
                    '6' => 5,
                    '7' => 6,
                    '8' => 7,
                    '9' => 8,
                    'T' => 9,
                    'Q' => 10,
                    'K' => 11,
                    'A' => 12,
                    _ => panic!("Invalid face"),
                }
            } else {
                match ch {
                    '2' => 0,
                    '3' => 1,
                    '4' => 2,
                    '5' => 3,
                    '6' => 4,
                    '7' => 5,
                    '8' => 6,
                    '9' => 7,
                    'T' => 8,
                    'J' => 9,
                    'Q' => 10,
                    'K' => 11,
                    'A' => 12,
                    _ => panic!("Invalid face"),
                }
            };
        });
        cards
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Hand {
    cards: Cards,
    hand_type: HandType,
    bid: u64,
}

impl From<(&str, bool)> for Hand {
    fn from(s: (&str, bool)) -> Self {
        let cards = Cards::from((s.0, s.1));
        let hand_type = if s.1 {
            determine_hand_type_with_joker(&cards)
        } else {
            determine_hand_type(&cards)
        };

        let bid = s.0[6..].parse::<u64>().unwrap();

        Hand {
            cards,
            hand_type,
            bid,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type != other.hand_type {
            return self.hand_type.cmp(&other.hand_type);
        }

        for (card_self, card_other) in self.cards.0.iter().zip(other.cards.0.iter()) {
            let order = card_other.cmp(card_self);
            if order != Ordering::Equal {
                return order;
            }
        }

        Ordering::Equal
    }
}

fn determine_hand_type(cards: &Cards) -> HandType {
    let mut frequencies = [0; 13];
    for card in &cards.0 {
        frequencies[*card as usize] += 1;
    }

    let (mut max_freq, mut second_max_freq) = (0, 0);

    for &freq in frequencies.iter() {
        if freq > max_freq {
            second_max_freq = max_freq;
            max_freq = freq;
        } else if freq > second_max_freq {
            second_max_freq = freq;
        }
    }
    match (max_freq, second_max_freq) {
        (5, _) => HandType::FiveOfAKind,
        (4, _) => HandType::FourOfAKind,
        (3, 2) | (2, 3) => HandType::FullHouse,
        (3, _) => HandType::ThreeOfAKind,
        (2, 2) => HandType::TwoPair,
        (2, _) => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

fn determine_hand_type_with_joker(cards: &Cards) -> HandType {
    let mut frequencies = [0; 13];
    let mut joker_count = 0;

    for card in &cards.0 {
        if *card == 0 {
            joker_count += 1;
        } else {
            frequencies[*card as usize] += 1;
        }
    }

    let (mut max_freq, mut second_max_freq) = (0, 0);
    for &freq in frequencies.iter() {
        if freq > max_freq {
            second_max_freq = max_freq;
            max_freq = freq;
        } else if freq > second_max_freq {
            second_max_freq = freq;
        }
    }

    let hand_type = match (max_freq, second_max_freq) {
        (5, _) => HandType::FiveOfAKind,
        (4, _) => HandType::FourOfAKind,
        (3, 2) | (2, 3) => HandType::FullHouse,
        (3, _) => HandType::ThreeOfAKind,
        (2, 2) => HandType::TwoPair,
        (2, _) => HandType::OnePair,
        _ => HandType::HighCard,
    };

    match (joker_count, hand_type) {
        (5, _) | (4, _) => HandType::FiveOfAKind,
        (3, HandType::OnePair) => HandType::FiveOfAKind,
        (3, _) => HandType::FourOfAKind,
        (2, HandType::ThreeOfAKind) => HandType::FiveOfAKind,
        (2, HandType::OnePair) => HandType::FourOfAKind,
        (2, _) => HandType::ThreeOfAKind,
        (1, HandType::FourOfAKind) => HandType::FiveOfAKind,
        (1, HandType::ThreeOfAKind) => HandType::FourOfAKind,
        (1, HandType::TwoPair) => HandType::FullHouse,
        (1, HandType::OnePair) => HandType::ThreeOfAKind,
        (1, _) => HandType::OnePair,
        (0, _) => hand_type,
        _ => panic!("Invalid hand type"),
    }
}

#[aoc(day7, part1)]
pub fn part_1(input: &str) -> u64 {
    input
        .lines()
        .map(|hand_str| Hand::from((hand_str, false)))
        .collect::<BTreeSet<Hand>>()
        .iter()
        .rev()
        .enumerate()
        .map(|(index, hand)| hand.bid * (index as u64 + 1))
        .sum()
}

#[aoc(day7, part2)]
pub fn part_2(input: &str) -> u64 {
    input
        .lines()
        .map(|hand_str| Hand::from((hand_str, true)))
        .collect::<BTreeSet<Hand>>()
        .iter()
        .rev()
        .enumerate()
        .map(|(index, hand)| hand.bid * (index as u64 + 1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(SAMPLE), 6440);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(SAMPLE), 5905);
    }
}

impl fmt::Display for Cards {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for card in self.0.iter() {
            write!(f, "{} ", card)?;
        }
        Ok(())
    }
}

impl fmt::Display for HandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandType::FiveOfAKind => write!(f, "FiveOfAKind"),
            HandType::FourOfAKind => write!(f, "FourOfAKind"),
            HandType::FullHouse => write!(f, "FullHouse"),
            HandType::ThreeOfAKind => write!(f, "ThreeOfAKind"),
            HandType::TwoPair => write!(f, "TwoPair"),
            HandType::OnePair => write!(f, "OnePair"),
            HandType::HighCard => write!(f, "HighCard"),
        }
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} | {} | {}", self.cards, self.bid, self.hand_type)
    }
}
