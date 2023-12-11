use std::collections::{HashMap, HashSet};
use std::fs;

use geo::Coord;

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day10.txt";

type InputType = Sketch;

#[derive(Debug, Clone, PartialEq)]
struct Sketch {
    nodes: Vec<Coord<u64>>,
    start: Coord<u64>,
}

fn part1(input: &InputType) -> Result<(), Error> {
    let answer = input.nodes.len() / 2;
    println!("part1: {answer}");
    Ok(())
}

fn part2(input: &InputType) -> Result<(), Error> {
    let answer: u64 = polygon_area(&input.nodes) - input.nodes.len() as u64 / 2 + 1;
    println!("part2: {answer}");
    Ok(())
}

#[rustfmt::skip]
fn link(
    input: &Vec<Vec<char>>,
    nodes: &mut HashMap<Coord<u64>, HashSet<Coord<u64>>>,
    coord: Coord<u64>,
    other: Coord<u64>,
    seen: &mut HashSet<Coord<u64>>,
) {
    if let Some(set) = nodes.get_mut(&coord) {
        set.insert(other);
    }
    if let Some(set) = nodes.get_mut(&other) {
        set.insert(coord);
    }

    if !seen.insert(other) {
        return;
    }

    match input[other.y as usize][other.x as usize] {
        '|' => {
            link(input, nodes, other, Coord { x: other.x, y: other.y - 1 }, seen);
            link(input, nodes, other, Coord { x: other.x, y: other.y + 1 }, seen);
        }
        '-' => {
            link(input, nodes, other, Coord { x: other.x - 1, y: other.y }, seen);
            link(input, nodes, other, Coord { x: other.x + 1, y: other.y }, seen);
        }
        'F' => {
            link(input, nodes, other, Coord { x: other.x, y: other.y + 1 }, seen);
            link(input, nodes, other, Coord { x: other.x + 1, y: other.y }, seen);
        }
        'L' => {
            link(input, nodes, other, Coord { x: other.x, y: other.y - 1 }, seen);
            link(input, nodes, other, Coord { x: other.x + 1, y: other.y }, seen);
        }
        'J' => {
            link(input, nodes, other, Coord { x: other.x, y: other.y - 1 }, seen);
            link(input, nodes, other, Coord { x: other.x - 1, y: other.y }, seen);
        }
        '7' => {
            link(input, nodes, other, Coord { x: other.x, y: other.y + 1 }, seen);
            link(input, nodes, other, Coord { x: other.x - 1, y: other.y }, seen);
        }
        _ => {}
    }
}

fn flatten_list(
    nodes: &HashMap<Coord<u64>, HashSet<Coord<u64>>>,
    start: Coord<u64>,
) -> Vec<Coord<u64>> {
    let mut list = vec![start];
    let mut last = start;
    let mut current = nodes[&start].iter().next().copied().unwrap();
    while current != start {
        list.push(current);
        let temp = nodes[&current]
            .iter()
            .find(|&&it| it != last)
            .copied()
            .unwrap();
        last = current;
        current = temp;
    }
    list
}

fn is_counter_clockwise(nodes: &[Coord<u64>]) -> bool {
    nodes.windows(2).fold(0i64, |acc, it| {
        acc + (it[1].x as i64 - it[0].x as i64) * (it[1].y as i64 + it[0].y as i64)
    }) < 0
}

fn polygon_area(nodes: &[Coord<u64>]) -> u64 {
    let (mut s1, mut s2) = nodes.windows(2).fold((0, 0), |(s1, s2), it| {
        (
            s1 + it[0].x as i64 * it[1].y as i64,
            s2 + it[0].y as i64 * it[1].x as i64,
        )
    });

    let first = nodes.first().unwrap();
    let last = nodes.last().unwrap();

    s1 += last.x as i64 * first.y as i64;
    s2 += first.x as i64 * last.y as i64;

    (s1 - s2).abs() as u64 / 2
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let input: InputType = {
        let mut start = Coord::default();
        let mut nodes = HashMap::new();
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        for (y, row) in grid.iter().enumerate() {
            for (x, ch) in row.iter().copied().enumerate() {
                let coord = Coord {
                    x: x as u64,
                    y: y as u64,
                };
                nodes.insert(coord, HashSet::default());
                if ch == 'S' {
                    start = coord;
                }
            }
        }

        let mut visited = HashSet::default();
        if let Some(y) = start.y.checked_sub(1) {
            let other = Coord { x: start.x, y };
            if let Some(ch) = grid
                .get(other.y as usize)
                .and_then(|row| row.get(other.x as usize))
            {
                if matches!(ch, '7' | 'F' | '|') {
                    link(&grid, &mut nodes, start, other, &mut visited);
                }
            }
        }
        if let Some(x) = start.x.checked_sub(1) {
            let other = Coord { x, y: start.y };
            if let Some(ch) = grid
                .get(other.y as usize)
                .and_then(|row| row.get(other.x as usize))
            {
                if matches!(ch, 'L' | 'F' | '-') {
                    link(&grid, &mut nodes, start, other, &mut visited);
                }
            }
        }
        if let Some(y) = (start.y < grid.len() as u64).then(|| start.y + 1) {
            let other = Coord { x: start.x, y };
            if let Some(ch) = grid
                .get(other.y as usize)
                .and_then(|row| row.get(other.x as usize))
            {
                if matches!(ch, 'L' | 'J' | '|') {
                    link(&grid, &mut nodes, start, other, &mut visited);
                }
            }
        }
        if let Some(x) = (start.x < grid[0].len() as u64).then(|| start.x + 1) {
            let other = Coord { x, y: start.y };
            if let Some(ch) = grid
                .get(other.y as usize)
                .and_then(|row| row.get(other.x as usize))
            {
                if matches!(ch, '7' | 'J' | '-') {
                    link(&grid, &mut nodes, start, other, &mut visited);
                }
            }
        }

        let mut nodes = flatten_list(&nodes, start);
        if !is_counter_clockwise(&nodes) {
            nodes.reverse();
        }

        Sketch { nodes, start }
    };

    measured!(part1(&input))?;
    measured!(part2(&input))?;

    Ok(())
}
