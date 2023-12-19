use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{env, fs, io::Read};

fn main() -> Result<()> {
    let input_file = env::args().nth(1).unwrap_or("./day-16.in".to_owned());
    let mut input = String::new();
    fs::File::open(&input_file)?.read_to_string(&mut input)?;
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));

    Ok(())
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Step {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

impl Step {
    pub fn len(&self) -> usize {
        match *self {
            Step::Up(len) => len,
            Step::Down(len) => len,
            Step::Left(len) => len,
            Step::Right(len) => len,
        }
    }

    pub fn rotated_trig(&self, len: usize) -> Self {
        match self {
            Step::Up(_) => Self::Left(len),
            Step::Down(_) => Self::Right(len),
            Step::Left(_) => Self::Down(len),
            Step::Right(_) => Self::Up(len),
        }
    }

    pub fn rotated_antitrig(&self, len: usize) -> Self {
        match self {
            Step::Up(_) => Self::Right(len),
            Step::Down(_) => Self::Left(len),
            Step::Left(_) => Self::Up(len),
            Step::Right(_) => Self::Down(len),
        }
    }

    pub fn advanced(&self) -> Self {
        match self {
            Step::Up(len) => Step::Up(len + 1),
            Step::Down(len) => Step::Down(len + 1),
            Step::Left(len) => Step::Left(len + 1),
            Step::Right(len) => Step::Right(len + 1),
        }
    }
}

fn dijkstra<const MIN_STEP: usize, const MAX_STEP: usize>(
    grid: &Vec<Vec<usize>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<usize> {
    let mut distance: Vec<Vec<usize>> = vec![vec![usize::MAX; grid[0].len()]; grid.len()];
    distance[start.0][start.1] = 0;

    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), Step::Down(1), start));
    heap.push((Reverse(0), Step::Right(1), start));

    let mut visited = HashSet::new();

    while let Some((Reverse(cost), step, (row, col))) = heap.pop() {
        if (row, col) == end {
            return Some(distance[end.0][end.1]);
        }

        if visited.contains(&((row, col), step)) {
            continue;
        }

        visited.insert(((row, col), step));

        if let Some((next_row, next_col)) = match step {
            Step::Up(_) => row.checked_sub(1).map(|r| (r, col)),
            Step::Down(_) => (row + 1 < grid.len()).then_some((row + 1, col)),
            Step::Left(_) => col.checked_sub(1).map(|c| (row, c)),
            Step::Right(_) => (col + 1 < grid[0].len()).then_some((row, col + 1)),
        } {
            let next_cost = cost + grid[next_row][next_col];
            if step.len() >= MIN_STEP {
                if next_cost < distance[next_row][next_col] {
                    distance[next_row][next_col] = next_cost;
                    heap.push((
                        Reverse(next_cost),
                        step.rotated_trig(step.len() + 1),
                        (next_row, next_col),
                    ));
                    heap.push((
                        Reverse(next_cost),
                        step.rotated_antitrig(step.len() + 1),
                        (next_row, next_col),
                    ));
                }
            }

            if step.len() < MAX_STEP {
                if next_cost < distance[next_row][next_col] {
                    heap.push((Reverse(next_cost), step.advanced(), (next_row, next_col)));
                }
            }
        }
    }

    None
}

fn part_one(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as usize)
                .collect_vec()
        })
        .collect_vec();

    let start = (0, 0);
    let end = (grid.len() - 1, grid[0].len() - 1);

    dijkstra::<1, 3>(&grid, start, end).unwrap()
}

fn part_two(input: &str) -> usize {
    todo!()
}

mod tests {
    use super::*;
    use indoc::*;

    #[test]
    fn test_part_one() {
        let input = indoc! {"
            2413432311323
            3215453535623
            3255245654254
            3446585845452
            4546657867536
            1438598798454
            4457876987766
            3637877979653
            4654967986887
            4564679986453
            1224686865563
            2546548887735
            4322674655533
        "};

        assert_eq!(part_one(input), 102);
    }
}
