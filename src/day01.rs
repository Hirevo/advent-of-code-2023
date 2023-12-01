use std::fs;

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day01.txt";

type InputType<'a> = Vec<&'a str>;

static DIGITS_TABLE: &[[&str; 2]] = &[
    ["0", "zero"],
    ["1", "one"],
    ["2", "two"],
    ["3", "three"],
    ["4", "four"],
    ["5", "five"],
    ["6", "six"],
    ["7", "seven"],
    ["8", "eight"],
    ["9", "nine"],
];

fn part1(input: &InputType) -> Result<(), Error> {
    let answer: usize = input
        .iter()
        .filter_map(|line| {
            let fst = line.find(|ch: char| ch.is_ascii_digit())?;
            let snd = line.rfind(|ch: char| ch.is_ascii_digit())?;
            Some(
                usize::from(line.as_bytes()[fst] - b'0') * 10
                    + usize::from(line.as_bytes()[snd] - b'0'),
            )
        })
        .sum();

    println!("part1: {answer}");
    Ok(())
}

fn part2(input: &InputType) -> Result<(), Error> {
    let answer: usize = input
        .iter()
        .filter_map(|line| {
            let fst = (0..line.len()).find_map(|idx| {
                DIGITS_TABLE.iter().position(|candidates| {
                    candidates
                        .iter()
                        .any(|candidate| line[idx..].starts_with(candidate))
                })
            })?;
            let snd = (0..line.len()).rev().find_map(|idx| {
                DIGITS_TABLE.iter().position(|candidates| {
                    candidates
                        .iter()
                        .any(|candidate| line[idx..].starts_with(candidate))
                })
            })?;
            Some(fst * 10 + snd)
        })
        .sum();

    println!("part2: {answer}");
    Ok(())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let input: InputType = input.lines().collect();

    measured!(part1(&input))?;
    measured!(part2(&input))?;

    Ok(())
}
