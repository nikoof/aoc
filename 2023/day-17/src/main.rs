use anyhow::Result;
use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap};
use std::{cmp::Ordering, env, fs, io::Read};

/* With heavy inspiration from https://gist.github.com/icub3d/ff31909ccb22fa16e3717cf72a59028e
* Couldn't for the life of me figure out why my first attempts were not working. At least now I
* know how to implement Dijkstra's algorithm... */

fn main() -> Result<()> {
    let input_file = env::args().nth(1).unwrap_or("./day-16.in".to_owned());
    let mut input = String::new();
    fs::File::open(&input_file)?.read_to_string(&mut input)?;
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn apply(
        &self,
        (row, col): (usize, usize),
        (row_bound, col_bound): (usize, usize),
    ) -> Option<(usize, usize)> {
        match self {
            Direction::Up => row.checked_sub(1).map(|r| (r, col)),
            Direction::Down => (row + 1 < row_bound).then_some((row + 1, col)),
            Direction::Left => col.checked_sub(1).map(|c| (row, c)),
            Direction::Right => (col + 1 < col_bound).then_some((row, col + 1)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    position: (usize, usize),
    direction: Direction,
    len: usize,
}

impl Node {
    pub fn new(position: (usize, usize), direction: Direction, len: usize) -> Self {
        Self {
            position,
            direction,
            len,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Step {
    node: Node,
    cost: usize,
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn neighbors<const MIN_STEP: usize, const MAX_STEP: usize>(
    node: &Node,
    grid: &Vec<Vec<usize>>,
) -> Vec<Node> {
    [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]
    .into_iter()
    .filter_map(|direction| {
        if direction != node.direction.opposite() {
            let new_position = direction.apply(node.position, (grid.len(), grid[0].len()));
            if direction != node.direction && node.len >= MIN_STEP {
                Some(Node::new(new_position?, direction, 1))
            } else if direction == node.direction && node.len < MAX_STEP {
                Some(Node::new(new_position?, direction, node.len + 1))
            } else {
                None
            }
        } else {
            None
        }
    })
    .collect_vec()
}

fn dijkstra<const MIN_STEP: usize, const MAX_STEP: usize>(
    costs: &Vec<Vec<usize>>,
    start: (usize, usize),
    target: (usize, usize),
) -> Option<usize> {
    let mut distances = HashMap::new();
    distances.insert(Node::new(start, Direction::Down, 0), 0);
    distances.insert(Node::new(start, Direction::Right, 0), 0);

    let mut frontier = BinaryHeap::new();
    frontier.push(Step {
        cost: 0,
        node: Node::new(start, Direction::Down, 0),
    });
    frontier.push(Step {
        cost: 0,
        node: Node::new(start, Direction::Right, 0),
    });

    while let Some(current_step) = frontier.pop() {
        if current_step.node.position == target && current_step.node.len >= MIN_STEP {
            return Some(current_step.cost);
        }

        for neighbor in neighbors::<MIN_STEP, MAX_STEP>(&current_step.node, costs) {
            let tentative_cost =
                current_step.cost + costs[neighbor.position.0][neighbor.position.1];

            if let Some(&best_cost) = distances.get(&neighbor) {
                if tentative_cost >= best_cost {
                    continue;
                }
            }

            distances.insert(neighbor, tentative_cost);
            frontier.push(Step {
                node: neighbor,
                cost: tentative_cost,
            });
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

    dijkstra::<4, 10>(&grid, start, end).unwrap()
}

#[cfg(test)]
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

    #[test]
    fn test_part_two() {
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

        assert_eq!(part_two(input), 94);
    }
}
