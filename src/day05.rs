use std::collections::HashMap;
use std::fs;

use chumsky::error::Simple;
use chumsky::primitive::{end, just};
use chumsky::{text, Parser};

use crate::interval::Interval;
use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day05.txt";

type InputType = Almanac;

#[derive(Debug, Clone, PartialEq)]
struct Almanac {
    seeds: Vec<u64>,
    maps: HashMap<String, (String, Vec<Mapping>)>,
}

#[derive(Debug, Clone, PartialEq)]
struct Mapping {
    src: Interval<u64>,
    dst: Interval<u64>,
}

fn part1(input: &InputType) -> Result<(), Error> {
    let answer: u64 = input
        .seeds
        .iter()
        .copied()
        .map(|mut value| {
            let mut label = "seed";
            while label != "location" {
                let (new_label, mappings) = &input.maps[label];
                label = new_label.as_str();
                value = mappings
                    .iter()
                    .find(|mapping| mapping.src.contains(value))
                    .map(|mapping| value - mapping.src.lo + mapping.dst.lo)
                    .unwrap_or(value);
            }
            value
        })
        .min()
        .expect("missing minimum seed");

    println!("part1: {answer}");
    Ok(())
}

fn part2(input: &InputType) -> Result<(), Error> {
    let answer: u64 = {
        let mut intervals: Vec<Interval<u64>> = Vec::default();
        let mut output: Vec<Interval<u64>> = input
            .seeds
            .chunks_exact(2)
            .map(|chunk| Interval::new(chunk[0], chunk[0] + chunk[1] - 1))
            .collect();

        let mut label = "seed";
        while label != "location" {
            std::mem::swap(&mut intervals, &mut output);
            output.clear();
            intervals.sort_unstable_by_key(|interval| interval.lo);

            let (new_label, mappings) = &input.maps[label];
            label = new_label.as_str();

            for mut interval in intervals.drain(..) {
                for mapping in mappings {
                    if interval.contains(mapping.src.lo - 1) {
                        let section = Interval::new(interval.lo, mapping.src.lo - 1);
                        output.push(section.clone());
                        interval = Interval::new(mapping.src.lo, interval.hi);
                    }
                    let Some(section) = interval.intersection(&mapping.src) else {
                        continue;
                    };
                    output.push(Interval::new(
                        section.lo - mapping.src.lo + mapping.dst.lo,
                        section.hi - mapping.src.lo + mapping.dst.lo,
                    ));
                    if section.hi + 1 > interval.hi {
                        break;
                    }
                    interval = Interval::new(section.hi + 1, interval.hi);
                }
            }

            output = simplify_intervals(output);
        }

        output
            .into_iter()
            .map(|interval| interval.lo)
            .min()
            .expect("missing minimum seed")
    };

    println!("part2: {answer}");
    Ok(())
}

fn simplify_intervals(mut intervals: Vec<Interval<u64>>) -> Vec<Interval<u64>> {
    intervals.sort_unstable_by_key(|interval| interval.lo);

    let Some((current, rest)) = intervals.split_first() else {
        return intervals;
    };

    let mut current = current.clone();
    let mut merged = Vec::new();

    for it in rest {
        if it.lo <= current.hi {
            current = current.union(it);
        } else {
            merged.push(current);
            current = it.clone();
        }
    }

    merged.push(current);
    merged
}

fn parser() -> impl Parser<char, InputType, Error = Simple<char>> {
    let seeds = just("seeds:")
        .ignore_then(just(' ').repeated())
        .ignore_then(
            text::int(10)
                .from_str()
                .unwrapped()
                .separated_by(just(' '))
                .at_least(1),
        );

    let maps = text::ident()
        .then_ignore(just("-to-"))
        .then(text::ident())
        .then_ignore(just(" map:\n"))
        .then(
            text::int(10)
                .from_str()
                .unwrapped()
                .then_ignore(just(' '))
                .then(text::int(10).from_str().unwrapped().then_ignore(just(' ')))
                .then(text::int(10).from_str().unwrapped())
                .map(|((dst, src), len): ((u64, u64), u64)| {
                    let src = Interval::new(src, src + len - 1);
                    let dst = Interval::new(dst, dst + len - 1);
                    Mapping { src, dst }
                })
                .separated_by(just('\n'))
                .at_least(1)
                .map(|mut mappings| {
                    mappings.sort_unstable_by_key(|mapping| mapping.src.lo);
                    mappings
                }),
        )
        .map(|((from, to), mappings)| (from, (to, mappings)))
        .separated_by(just("\n\n"))
        .at_least(1)
        .collect();

    seeds
        .then_ignore(just("\n\n"))
        .then(maps)
        .then_ignore(just('\n').or_not())
        .then_ignore(end())
        .map(|(seeds, maps)| Almanac { seeds, maps })
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
