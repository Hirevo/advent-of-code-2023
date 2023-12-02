use std::collections::HashMap;
use std::fs;

use chumsky::error::Simple;
use chumsky::primitive::just;
use chumsky::{text, Parser};

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day02.txt";

type InputType = Vec<Game>;

#[derive(Debug, Clone, PartialEq)]
struct Game {
    id: usize,
    rounds: Vec<HashMap<String, u64>>,
}

impl Game {
    pub fn is_possible(&self, bag: &HashMap<String, u64>) -> bool {
        self.rounds.iter().all(|round| {
            round
                .iter()
                .all(|(key, value)| bag.get(key).map_or(false, |allowed| value <= allowed))
        })
    }

    pub fn power(&self) -> u64 {
        self.rounds
            .iter()
            .fold(HashMap::<String, u64>::new(), |mut acc, round| {
                for (key, value) in round {
                    acc.entry(key.to_string())
                        .and_modify(|old| {
                            *old = (*value).max(*old);
                        })
                        .or_insert(*value);
                }
                acc
            })
            .values()
            .copied()
            .reduce(|acc, it| acc * it)
            .unwrap_or(0)
    }
}

fn part1(input: &InputType) -> Result<(), Error> {
    let bag = {
        let mut bag = HashMap::new();
        bag.insert("red".to_string(), 12);
        bag.insert("green".to_string(), 13);
        bag.insert("blue".to_string(), 14);
        bag
    };

    let answer: usize = input
        .iter()
        .filter(|game| game.is_possible(&bag))
        .map(|game| game.id)
        .sum();

    println!("part1: {answer}");
    Ok(())
}

fn part2(input: &InputType) -> Result<(), Error> {
    let answer: u64 = input.iter().map(|game| game.power()).sum();

    println!("part2: {answer}");
    Ok(())
}

fn parser() -> impl Parser<char, InputType, Error = Simple<char>> {
    text::int(10)
        .from_str()
        .unwrapped()
        .delimited_by(just("Game "), just(": "))
        .then(
            text::int(10)
                .from_str()
                .unwrapped()
                .then_ignore(text::whitespace())
                .then(text::ident())
                .separated_by(just(", "))
                .at_least(1)
                .map(|rounds| rounds.into_iter().map(|(a, b)| (b, a)).collect())
                .separated_by(just("; "))
                .at_least(1),
        )
        .map(|(id, rounds)| Game { id, rounds })
        .separated_by(text::newline())
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
