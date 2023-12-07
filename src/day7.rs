use core::fmt;
use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
    str::FromStr,
};

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
enum Card {
    #[default]
    A,
    K,
    Q,
    J,
    T,
    N(u8),
}

impl Card {
    fn strength(&self) -> u8 {
        match self {
            Card::A => 14,
            Card::K => 13,
            Card::Q => 12,
            Card::J => 11,
            Card::T => 10,
            Card::N(n) => *n,
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.strength().cmp(&other.strength())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.strength().cmp(&other.strength()))
    }
}

impl From<char> for Card {
    fn from(s: char) -> Card {
        match s {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::N(9),
            '8' => Card::N(8),
            '7' => Card::N(7),
            '6' => Card::N(6),
            '5' => Card::N(5),
            '4' => Card::N(4),
            '3' => Card::N(3),
            '2' => Card::N(2),
            n => panic!("Invalid character {n}"),
        }
    }
}

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
struct Cards([Card; 5]);

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Hand {
    cards: Cards,
    hand_type: HandType,
    bid: u64,
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cards: [Card; 5] = Default::default();
        for (i, c) in s.chars().enumerate() {
            cards[i] = Card::from(c);
            if i == 4 {
                break;
            }
        }
        let hand_type = determine_hand_type(&cards);

        let bid = s[6..].parse::<u64>().unwrap();

        Ok(Hand {
            cards: Cards(cards),
            hand_type,
            bid,
        })
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

fn determine_hand_type(cards: &[Card; 5]) -> HandType {
    let mut freq_map = HashMap::new();

    for card in cards {
        *freq_map.entry(card).or_insert(0) += 1;
    }

    let mut freqs: Vec<_> = freq_map.values().copied().collect();
    freqs.sort_unstable_by(|a, b| b.cmp(a));

    match freqs.as_slice() {
        [5, ..] => HandType::FiveOfAKind,
        [4, ..] => HandType::FourOfAKind,
        [3, 2, ..] | [2, 3, ..] => HandType::FullHouse,
        [3, ..] => HandType::ThreeOfAKind,
        [2, 2, ..] => HandType::TwoPair,
        [2, ..] => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

#[aoc(day7, part1)]
pub fn part_1(input: &str) -> u64 {
    input
        .lines()
        .map(|h| Hand::from_str(h).unwrap())
        .map(|hand| (hand, hand.bid))
        .collect::<BTreeMap<Hand, u64>>()
        .iter()
        .take(5)
        .enumerate()
        .map(|(index, (_, bid))| bid * (5 - index as u64))
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
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Card::A => write!(f, "A"),
            Card::K => write!(f, "K"),
            Card::Q => write!(f, "Q"),
            Card::J => write!(f, "J"),
            Card::T => write!(f, "T"),
            Card::N(n) => write!(f, "{}", n),
        }
    }
}

impl fmt::Display for Cards {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for card in self.0.iter() {
            write!(f, "{}", card)?;
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
