use std::fs;

use color_eyre::eyre::bail;
use geo::Coord;
use itertools::Itertools;

use crate::interval::Interval;
use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day11.txt";

type InputType = Vec<Vec<Spot>>;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Spot {
    Empty,
    Galaxy,
}

fn part1(input: &InputType) -> Result<(), Error> {
    let answer: u64 = {
        let empty_rows = find_empty_rows(&input);
        let empty_cols = find_empty_cols(&input);

        let galaxies = find_galaxies(&input);

        galaxies
            .into_iter()
            .tuple_combinations()
            .map(|(fst, snd)| {
                let delta_x = Interval::new(fst.x.min(snd.x), fst.x.max(snd.x));
                let delta_y = Interval::new(fst.y.min(snd.y), fst.y.max(snd.y));
                let delta_x = empty_cols.iter().filter(|&&x| delta_x.contains(x)).count();
                let delta_y = empty_rows.iter().filter(|&&y| delta_y.contains(y)).count();
                manhattan_distance(fst, snd) as u64 + delta_x as u64 + delta_y as u64
            })
            .sum()
    };

    println!("part1: {answer}");
    Ok(())
}

fn part2(input: &InputType) -> Result<(), Error> {
    let answer: u64 = {
        let expansion_rate = 1_000_000 - 1;

        let empty_rows = find_empty_rows(&input);
        let empty_cols = find_empty_cols(&input);

        let galaxies = find_galaxies(&input);

        galaxies
            .into_iter()
            .tuple_combinations()
            .map(|(fst, snd)| {
                let delta_x = Interval::new(fst.x.min(snd.x), fst.x.max(snd.x));
                let delta_y = Interval::new(fst.y.min(snd.y), fst.y.max(snd.y));
                let delta_x = empty_cols.iter().filter(|&&x| delta_x.contains(x)).count();
                let delta_y = empty_rows.iter().filter(|&&y| delta_y.contains(y)).count();
                manhattan_distance(fst, snd) as u64
                    + (delta_x as u64 * expansion_rate)
                    + (delta_y as u64 * expansion_rate)
            })
            .sum()
    };
    println!("part2: {answer}");
    Ok(())
}

fn find_empty_rows(spots: &[Vec<Spot>]) -> Vec<usize> {
    (0..spots.len())
        .filter(|&y| spots[y].iter().all(|&spot| spot == Spot::Empty))
        .collect()
}

fn find_empty_cols(spots: &[Vec<Spot>]) -> Vec<usize> {
    (0..spots[0].len())
        .filter(|&x| (0..spots.len()).all(|y| spots[y][x] == Spot::Empty))
        .collect()
}

fn find_galaxies(spots: &[Vec<Spot>]) -> Vec<Coord<usize>> {
    (0..spots.len())
        .cartesian_product(0..spots[0].len())
        .filter(|&(y, x)| spots[y][x] == Spot::Galaxy)
        .map(|(y, x)| Coord { x, y })
        .collect()
}

fn manhattan_distance(fst: Coord<usize>, snd: Coord<usize>) -> usize {
    snd.x.abs_diff(fst.x) + snd.y.abs_diff(fst.y)
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let input: InputType = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '.' => Ok(Spot::Empty),
                    '#' => Ok(Spot::Galaxy),
                    ch => bail!("invalid spot (found `{ch}`)"),
                })
                .collect::<Result<_, _>>()
        })
        .collect::<Result<_, _>>()?;

    measured!(part1(&input))?;
    measured!(part2(&input))?;

    Ok(())
}
