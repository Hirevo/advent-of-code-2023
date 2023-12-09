use std::fs;

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day09.txt";

type InputType = Vec<Vec<i64>>;

fn part1(input: &InputType) -> Result<(), Error> {
    let answer: i64 = input
        .iter()
        .map(|sequence| extrapolate_forward(sequence))
        .sum();

    println!("part1: {answer}");
    Ok(())
}

fn part2(input: &InputType) -> Result<(), Error> {
    let answer: i64 = input
        .iter()
        .map(|sequence| extrapolate_backward(sequence))
        .sum();

    println!("part2: {answer}");
    Ok(())
}

fn extrapolate_forward(sequence: &[i64]) -> i64 {
    let next_sequence: Vec<_> = sequence.windows(2).map(|pair| pair[1] - pair[0]).collect();
    if next_sequence.iter().copied().all(|it| it == 0) {
        sequence.last().copied().expect("empty sequence")
    } else {
        sequence.last().expect("empty sequence") + extrapolate_forward(&next_sequence)
    }
}

fn extrapolate_backward(sequence: &[i64]) -> i64 {
    let next_sequence: Vec<_> = sequence.windows(2).map(|pair| pair[1] - pair[0]).collect();
    if next_sequence.iter().copied().all(|it| it == 0) {
        sequence.first().copied().expect("empty sequence")
    } else {
        sequence.first().expect("empty sequence") - extrapolate_backward(&next_sequence)
    }
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let input: InputType = input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|number| number.parse())
                .collect::<Result<_, _>>()
        })
        .collect::<Result<_, _>>()?;

    measured!(part1(&input))?;
    measured!(part2(&input))?;

    Ok(())
}
