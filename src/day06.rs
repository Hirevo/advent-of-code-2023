use std::fs;

use chumsky::error::Simple;
use chumsky::primitive::{end, just};
use chumsky::{text, Parser};

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day06.txt";

type InputType = Vec<Race>;

#[derive(Debug, Clone, PartialEq)]
struct Race {
    time: u64,
    distance: u64,
}

fn part1(input: &InputType) -> Result<(), Error> {
    let answer: u64 = input.iter().fold(1, |acc, race| {
        let found = binary_search(1, race.time, |candidate| {
            (race.time - candidate) * candidate >= race.distance
        })
        .expect("could not find an answer");

        acc * match (race.time - found) * found == race.distance {
            true => race.time - found * 2 - 1,
            false => race.time - found * 2 + 1,
        }
    });

    println!("part1: {answer}");
    Ok(())
}

fn part2(input: &InputType) -> Result<(), Error> {
    let (time, distance) = input.iter().fold(
        (String::default(), String::default()),
        |(time, distance), race| {
            (
                format!("{time}{0}", race.time),
                format!("{distance}{0}", race.distance),
            )
        },
    );

    let time: u64 = time.parse()?;
    let distance: u64 = distance.parse()?;

    let answer = binary_search(1, time, |candidate| {
        (time - candidate) * candidate >= distance
    })
    .expect("could not find an answer");

    let answer = match (time - answer) * answer == distance {
        true => time - answer * 2 - 1,
        false => time - answer * 2 + 1,
    };

    println!("part2: {answer}");
    Ok(())
}

fn binary_search<F>(mut start: u64, mut end: u64, mut predicate: F) -> Option<u64>
where
    F: FnMut(u64) -> bool,
{
    loop {
        let mid = (end + start) / 2;
        if mid == 0 {
            break None;
        }

        match (predicate(mid), predicate(mid - 1)) {
            (true, false) | (false, true) => {
                break Some(mid);
            }
            (true, true) => {
                end = mid;
            }
            (false, false) => {
                start = mid;
            }
        }
    }
}

fn parser() -> impl Parser<char, InputType, Error = Simple<char>> {
    just("Time:")
        .ignore_then(
            text::int(10)
                .from_str()
                .unwrapped()
                .separated_by(just(' ').repeated().at_least(1))
                .at_least(1)
                .padded_by(just(' ').repeated()),
        )
        .then_ignore(just('\n'))
        .then(
            just("Distance:").ignore_then(
                text::int(10)
                    .from_str()
                    .unwrapped()
                    .separated_by(just(' ').repeated().at_least(1))
                    .at_least(1)
                    .padded_by(just(' ').repeated()),
            ),
        )
        .map(|(times, distances)| {
            times
                .into_iter()
                .zip(distances)
                .map(|(time, distance)| Race { time, distance })
                .collect()
        })
        .then_ignore(just('\n').or_not())
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
