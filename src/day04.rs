use std::collections::BTreeSet;
use std::fs;

use chumsky::error::Simple;
use chumsky::primitive::{end, just};
use chumsky::{text, Parser};

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day04.txt";

type InputType = Vec<Card>;

#[derive(Debug, Clone, PartialEq)]
struct Card {
    id: usize,
    winning: BTreeSet<u32>,
    numbers: Vec<u32>,
}

impl Card {
    pub fn score(&self) -> usize {
        self.numbers
            .iter()
            .filter(|number| self.winning.contains(number))
            .count()
    }
}

fn part1(input: &InputType) -> Result<(), Error> {
    let answer: u64 = input
        .iter()
        .map(|card| {
            let score = card.score() as u32;
            score
                .checked_sub(1)
                .map(|score| 2u64.pow(score))
                .unwrap_or(0)
        })
        .sum();

    println!("part1: {answer}");
    Ok(())
}

fn part2(input: &InputType) -> Result<(), Error> {
    let mut state = vec![1usize; input.len()];

    for (idx, card) in input.iter().enumerate() {
        let score = card.score();
        let count = state[idx];
        for slot in &mut state[idx + 1..idx + 1 + score] {
            *slot += count;
        }
    }

    let answer: usize = state.into_iter().sum();

    println!("part2: {answer}");
    Ok(())
}

fn parser() -> impl Parser<char, InputType, Error = Simple<char>> {
    let numbers_list = text::int(10)
        .from_str()
        .unwrapped()
        .separated_by(just(' ').repeated().at_least(1))
        .at_least(1)
        .padded_by(just(' ').repeated());

    text::int(10)
        .from_str()
        .unwrapped()
        .padded_by(just(' ').repeated())
        .delimited_by(just("Card"), just(':'))
        .then(
            numbers_list
                .collect()
                .then_ignore(just('|'))
                .then(numbers_list),
        )
        .map(|(id, (winning, numbers))| Card {
            id,
            winning,
            numbers,
        })
        .separated_by(text::newline())
        .allow_trailing()
        .then_ignore(end())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let input: InputType = parser()
        .parse(input.as_str())
        .expect("could not parse input");

    measured!(part1(&input))?;
    measured!(part2(&input))?;

    Ok(())
}
