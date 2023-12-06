use std::fs;

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/dayXX.txt";

type InputType = String;

fn part1(_input: &InputType) -> Result<(), Error> {
    let answer: u64 = todo!();
    println!("part1: {answer}");
    Ok(())
}

fn part2(_input: &InputType) -> Result<(), Error> {
    let answer: u64 = todo!();
    println!("part2: {answer}");
    Ok(())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let input: InputType = todo!();

    measured!(part1(&input))?;
    measured!(part2(&input))?;

    Ok(())
}
