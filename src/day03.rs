use std::fs;

use geo::{Coord, Intersects, Rect};

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day03.txt";

type InputType = Schematic;

#[derive(Debug, Clone, PartialEq)]
struct PartNumber {
    value: u64,
    span: Rect<usize>,
}

#[derive(Debug, Clone, PartialEq)]
struct PartSymbol {
    symbol: char,
    coord: Coord<usize>,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct Schematic {
    numbers: Vec<PartNumber>,
    symbols: Vec<PartSymbol>,
}

fn part1(input: &InputType) -> Result<(), Error> {
    let answer: u64 = input
        .numbers
        .iter()
        .filter(|number| {
            let aabb = Rect::new(
                Coord {
                    x: number.span.min().x.saturating_sub(1),
                    y: number.span.min().y.saturating_sub(1),
                },
                Coord {
                    x: number.span.max().x.saturating_add(1),
                    y: number.span.max().y.saturating_add(1),
                },
            );

            input
                .symbols
                .iter()
                .any(|symbol| aabb.intersects(&symbol.coord))
        })
        .map(|number| number.value)
        .sum();

    println!("part1: {answer}");
    Ok(())
}

fn part2(input: &InputType) -> Result<(), Error> {
    let answer: u64 = input
        .symbols
        .iter()
        .filter(|symbol| symbol.symbol == '*')
        .filter_map(|symbol| {
            let adjacent: Vec<_> = input
                .numbers
                .iter()
                .filter(|number| {
                    let aabb = Rect::new(
                        Coord {
                            x: number.span.min().x.saturating_sub(1),
                            y: number.span.min().y.saturating_sub(1),
                        },
                        Coord {
                            x: number.span.max().x.saturating_add(1),
                            y: number.span.max().y.saturating_add(1),
                        },
                    );

                    aabb.intersects(&symbol.coord)
                })
                .map(|number| number.value)
                .collect();

            (adjacent.len() == 2).then(|| adjacent.into_iter().fold(1, |acc, it| acc * it))
        })
        .sum();

    println!("part2: {answer}");
    Ok(())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let input: InputType = {
        let mut schematic = Schematic::default();
        for (y, row) in input.lines().enumerate() {
            let mut x = 0;
            while let Some(rest) = row.get(x..) {
                let coord = Coord { x, y };
                match rest.chars().next() {
                    Some('.') => {
                        x += 1;
                    }
                    Some(ch) if ch.is_ascii_digit() => {
                        let len = rest
                            .bytes()
                            .take_while(|byte| byte.is_ascii_digit())
                            .count();
                        let value: u64 = rest[..len].parse()?;
                        let span = Rect::new(coord, Coord { x: x + len - 1, y });
                        schematic.numbers.push(PartNumber { value, span });
                        x += len;
                    }
                    Some(symbol) => {
                        schematic.symbols.push(PartSymbol { symbol, coord });
                        x += 1;
                    }
                    None => {
                        break;
                    }
                }
            }
        }
        schematic
    };

    measured!(part1(&input))?;
    measured!(part2(&input))?;

    Ok(())
}
