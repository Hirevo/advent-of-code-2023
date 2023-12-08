use std::collections::HashMap;
use std::fs;

use chumsky::error::Simple;
use chumsky::primitive::{choice, end, just};
use chumsky::{text, Parser};

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day08.txt";

type InputType = Network;

#[derive(Debug, Clone, PartialEq)]
struct Network {
    instructions: Vec<Instruction>,
    nodes: HashMap<String, (String, String)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Left,
    Right,
}

fn part1(input: &InputType) -> Result<(), Error> {
    let answer: u64 = compute_path_length(input, "AAA", |node| node == "ZZZ");
    println!("part1: {answer}");
    Ok(())
}

fn part2(input: &InputType) -> Result<(), Error> {
    let answer: u64 = input
        .nodes
        .keys()
        .filter(|node| node.ends_with('A'))
        .map(|node| compute_path_length(input, node.as_str(), |node| node.ends_with('Z')))
        .reduce(least_common_multiple)
        .expect("empty iterator");

    println!("part2: {answer}");
    Ok(())
}

fn compute_path_length<'a, F>(network: &'a Network, mut start: &'a str, mut goal: F) -> u64
where
    F: FnMut(&str) -> bool,
{
    let mut steps = 0;

    while !goal(start) {
        match network.instructions[steps % network.instructions.len()] {
            Instruction::Left => {
                start = network.nodes[start].0.as_str();
            }
            Instruction::Right => {
                start = network.nodes[start].1.as_str();
            }
        }
        steps += 1;
    }

    steps as u64
}

fn greatest_common_divisor(mut fst: u64, mut snd: u64) -> u64 {
    while snd != 0 {
        let tmp = snd;
        snd = fst % snd;
        fst = tmp;
    }
    fst
}

fn least_common_multiple(fst: u64, snd: u64) -> u64 {
    (fst * snd) / greatest_common_divisor(fst, snd)
}

fn parser() -> impl Parser<char, InputType, Error = Simple<char>> {
    let instructions = choice((
        just('L').to(Instruction::Left),
        just('R').to(Instruction::Right),
    ))
    .repeated()
    .at_least(1);

    let nodes = text::ident()
        .then_ignore(just(" = "))
        .then(
            text::ident()
                .then_ignore(just(", "))
                .then(text::ident())
                .delimited_by(just('('), just(')')),
        )
        .separated_by(just('\n'))
        .at_least(1)
        .collect();

    instructions
        .then_ignore(just("\n\n"))
        .then(nodes)
        .map(|(instructions, nodes)| Network {
            instructions,
            nodes,
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
