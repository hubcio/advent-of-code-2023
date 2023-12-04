use core::panic;
use std::collections::{HashSet, VecDeque};

use rayon::{iter::ParallelIterator, str::ParallelString};

struct ScratchLotteryCard {}

impl ScratchLotteryCard {
    fn calculate_points(input: &str) -> u32 {
        input
            .par_lines()
            .map(|line| {
                let colon_index = line.find(':').unwrap();
                let id_str = &line[5..colon_index];
                let _id = id_str.trim().parse::<usize>().unwrap();
                let lottery_str = &line[colon_index + 1..];

                let parts: Vec<&str> = lottery_str.split('|').collect();
                if parts.len() != 2 {
                    panic!("Input does not contain exactly one delimiter '|'");
                }

                let winning_numbers: HashSet<u32> = parts[0]
                    .split_whitespace()
                    .filter_map(|num| num.parse::<u32>().ok())
                    .collect();

                let mut score = 0;

                parts[1]
                    .split_whitespace()
                    .filter_map(|num| num.parse::<u32>().ok())
                    .for_each(|num| {
                        if winning_numbers.contains(&num) {
                            if score == 0 {
                                score = 1;
                            } else {
                                score *= 2;
                            }
                        }
                    });

                score
            })
            .sum()
    }
}

#[aoc(day4, part1, ParseAndStoreEverything)]
pub fn part1(input: &str) -> u32 {
    ScratchLotteryCard::calculate_points(input)
}

#[derive(Debug, Clone)]
struct LotteryCard {
    id: usize,
    winnings: usize,
    copies: usize,
}

fn calculate_total_for_card(cards: &[LotteryCard], index: usize) -> u32 {
    if index >= cards.len() {
        return 0;
    }

    let card = &cards[index];
    let mut total = 1;
    for n in 1..=card.winnings {
        if index + n < cards.len() {
            total += calculate_total_for_card(cards, index + n);
        }
    }

    total
}

#[aoc(day4, part2, Recurrence)]
pub fn part2_recurrence(input: &str) -> u32 {
    let cards: Vec<LotteryCard> = input
        .lines()
        .map(|line| {
            let colon_index = line.find(':').unwrap();
            let id_str = &line[5..colon_index];
            let id = id_str.trim().parse::<usize>().unwrap();
            let lottery_str = &line[colon_index + 1..];

            let parts: Vec<&str> = lottery_str.split('|').collect();

            let winning_numbers: HashSet<u32> = parts[0]
                .split_whitespace()
                .filter_map(|num| num.parse::<u32>().ok())
                .collect();

            let mut card = LotteryCard {
                id,
                winnings: 0,
                copies: 1,
            };

            parts[1]
                .split_whitespace()
                .filter_map(|num| num.parse::<u32>().ok())
                .for_each(|num| {
                    if winning_numbers.contains(&num) {
                        card.winnings += 1;
                    }
                });
            card
        })
        .collect();

    (0..cards.len())
        .map(|i| calculate_total_for_card(&cards, i))
        .sum()
}

#[aoc(day4, part2, Iterative)]
pub fn part2_iterative(input: &str) -> u32 {
    let mut cards: VecDeque<LotteryCard> = input
        .lines()
        .map(|line| {
            let colon_index = line.find(':').unwrap();
            let id_str = &line[5..colon_index];
            let id = id_str.trim().parse::<usize>().unwrap();
            let lottery_str = &line[colon_index + 1..];

            let parts: Vec<&str> = lottery_str.split('|').collect();

            let winning_numbers: HashSet<u32> = parts[0]
                .split_whitespace()
                .filter_map(|num| num.parse::<u32>().ok())
                .collect();

            let mut card = LotteryCard {
                id,
                winnings: 0,
                copies: 1,
            };

            parts[1]
                .split_whitespace()
                .filter_map(|num| num.parse::<u32>().ok())
                .for_each(|num| {
                    if winning_numbers.contains(&num) {
                        card.winnings += 1;
                    }
                });
            card
        })
        .collect();

    let mut total = 0;
    let initial_cards = cards.clone();

    while let Some(card) = cards.pop_front() {
        total += 1;
        for n in 1..=card.winnings {
            let next_card_index = card.id - 1 + n;
            if next_card_index < initial_cards.len() {
                cards.push_back(initial_cards[next_card_index].clone());
            }
        }
    }
    total
}

#[aoc(day4, part2, BetterIterative)]
pub fn part2_better_iterative(input: &str) -> u32 {
    let mut cards: Vec<LotteryCard> = input
        .lines()
        .map(|line| {
            let colon_index = line.find(':').unwrap();
            let id_str = &line[5..colon_index];
            let id = id_str.trim().parse::<usize>().unwrap();
            let lottery_str = &line[colon_index + 1..];

            let parts: Vec<&str> = lottery_str.split('|').collect();

            let winning_numbers: HashSet<u32> = parts[0]
                .split_whitespace()
                .filter_map(|num| num.parse::<u32>().ok())
                .collect();

            let mut card = LotteryCard {
                id,
                winnings: 0,
                copies: 1,
            };

            parts[1]
                .split_whitespace()
                .filter_map(|num| num.parse::<u32>().ok())
                .for_each(|num| {
                    if winning_numbers.contains(&num) {
                        card.winnings += 1;
                    }
                });
            card
        })
        .collect();

    for i in 0..cards.len() {
        let card = &cards[i];
        let winnings = card.winnings;
        for _ in 1..=card.copies {
            for n in 1..=winnings {
                let next_card_index = i + n;
                if next_card_index < cards.len() {
                    cards[next_card_index].copies += 1;
                }
            }
        }
    }

    cards.iter().map(|card| card.copies as u32).sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2_better_iterative, part2_iterative, part2_recurrence};

    static SAMPLE: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part1(SAMPLE), 13);
    }

    #[test]
    fn test_part_2_recurrence() {
        assert_eq!(part2_recurrence(SAMPLE), 30);
    }

    #[test]
    fn test_part_2_iterative() {
        assert_eq!(part2_iterative(SAMPLE), 30);
    }

    #[test]
    fn test_part_2_better_iterative() {
        assert_eq!(part2_better_iterative(SAMPLE), 30);
    }
}
