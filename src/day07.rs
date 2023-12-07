use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

use color_eyre::eyre::bail;

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day07.txt";

type InputType = Vec<Hand>;

#[derive(Debug, Clone, PartialEq)]
struct Hand {
    cards: [Card; 5],
    bid: u64,
}

impl Hand {
    pub fn evaluate_part1(&self) -> u64 {
        let counts = {
            let mut counts: Vec<usize> = self
                .cards
                .iter()
                .copied()
                .fold(HashMap::<Card, usize>::default(), |mut acc, card| {
                    *acc.entry(card).or_default() += 1;
                    acc
                })
                .into_values()
                .collect();
            counts.sort_unstable_by(|a, b| b.cmp(a));
            counts
        };

        let hand_type = match counts.as_slice() {
            [5] => HandType::FiveOfAKind,
            [4, 1] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [2, 2, 1] => HandType::TwoPairs,
            [2, 1, 1, 1] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => unreachable!(),
        };

        hand_type.value()
    }

    pub fn evaluate_part2(&self) -> u64 {
        let counts = {
            let mut jokers = 0;
            let mut counts: Vec<usize> = self
                .cards
                .iter()
                .copied()
                .fold(HashMap::<Card, usize>::default(), |mut acc, card| {
                    if card == Card::Joker {
                        jokers += 1;
                    } else {
                        *acc.entry(card).or_default() += 1;
                    }
                    acc
                })
                .into_values()
                .collect();
            counts.sort_unstable_by(|a, b| b.cmp(a));
            if let Some(count) = counts.get_mut(0) {
                *count += jokers;
            } else {
                counts.push(jokers);
            }
            counts
        };

        let hand_type = match counts.as_slice() {
            [5] => HandType::FiveOfAKind,
            [4, 1] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [2, 2, 1] => HandType::TwoPairs,
            [2, 1, 1, 1] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => unreachable!(),
        };

        hand_type.value()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard,
}

impl HandType {
    pub fn value(self) -> u64 {
        match self {
            HandType::FiveOfAKind => 7,
            HandType::FourOfAKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeOfAKind => 4,
            HandType::TwoPairs => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Valet,
    Queen,
    King,
    As,
    Joker,
}

impl Card {
    pub fn value_part1(self) -> u64 {
        match self {
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::Valet => 10,
            Card::Joker => 11,
            Card::Queen => 12,
            Card::King => 13,
            Card::As => 14,
        }
    }

    pub fn value_part2(self) -> u64 {
        match self {
            Card::Joker => 1,
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::Valet => 10,
            Card::Queen => 11,
            Card::King => 12,
            Card::As => 13,
        }
    }
}

impl TryFrom<char> for Card {
    type Error = Error;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            '2' => Ok(Card::Two),
            '3' => Ok(Card::Three),
            '4' => Ok(Card::Four),
            '5' => Ok(Card::Five),
            '6' => Ok(Card::Six),
            '7' => Ok(Card::Seven),
            '8' => Ok(Card::Eight),
            '9' => Ok(Card::Nine),
            'T' => Ok(Card::Valet),
            'Q' => Ok(Card::Queen),
            'K' => Ok(Card::King),
            'A' => Ok(Card::As),
            'J' => Ok(Card::Joker),
            ch => {
                bail!("unrecognized card `{ch}`");
            }
        }
    }
}

fn part1(input: &InputType) -> Result<(), Error> {
    let answer: u64 = {
        let mut ranks: Vec<_> = input
            .iter()
            .map(|hand| (hand, hand.evaluate_part1()))
            .collect();
        ranks.sort_unstable_by(|(hand_a, value_a), (hand_b, value_b)| {
            if value_a != value_b {
                return value_a.cmp(value_b);
            }

            let cards_a = hand_a.cards.iter().copied().map(Card::value_part1);
            let cards_b = hand_b.cards.iter().copied().map(Card::value_part1);
            for (card_a, card_b) in cards_a.zip(cards_b) {
                if card_a != card_b {
                    return card_a.cmp(&card_b);
                }
            }

            Ordering::Equal
        });

        ranks
            .iter()
            .enumerate()
            .map(|(rank, (hand, _))| hand.bid * (rank as u64 + 1))
            .sum()
    };

    println!("part1: {answer}");
    Ok(())
}

fn part2(input: &InputType) -> Result<(), Error> {
    let answer: u64 = {
        let mut ranks: Vec<_> = input
            .iter()
            .map(|hand| (hand, hand.evaluate_part2()))
            .collect();
        ranks.sort_unstable_by(|(hand_a, value_a), (hand_b, value_b)| {
            if value_a != value_b {
                return value_a.cmp(value_b);
            }

            let cards_a = hand_a.cards.iter().copied().map(Card::value_part2);
            let cards_b = hand_b.cards.iter().copied().map(Card::value_part2);
            for (card_a, card_b) in cards_a.zip(cards_b) {
                if card_a != card_b {
                    return card_a.cmp(&card_b);
                }
            }

            Ordering::Equal
        });

        ranks
            .iter()
            .enumerate()
            .map(|(rank, (hand, _))| hand.bid * (rank as u64 + 1))
            .sum()
    };

    println!("part2: {answer}");
    Ok(())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let input: InputType = input
        .lines()
        .filter_map(|line| {
            let (hand, bid) = line.split_once(' ')?;
            let cards = hand
                .chars()
                .map(|ch| Card::try_from(ch).ok())
                .collect::<Option<Vec<_>>>()?
                .try_into()
                .ok()?;
            let bid = bid.parse().ok()?;
            Some(Hand { cards, bid })
        })
        .collect();

    measured!(part1(&input))?;
    measured!(part2(&input))?;

    Ok(())
}
