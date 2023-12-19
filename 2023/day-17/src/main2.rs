use anyhow::Result;
use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{cmp::Ordering, env, fs, io::Read};

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
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl Direction {
    fn trigonometric_rotation(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn antitrigonometric_rotation(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
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

#[derive(Debug, Clone, Copy, Eq, Hash)]
struct Step {
    position: (usize, usize),
    direction: Direction,
    len: usize,
    cost: usize,
}

impl PartialEq for Step {
    fn eq(&self, other: &Self) -> bool {
        (self.position, self.direction, self.len).eq(&(other.position, other.direction, other.len))
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn dijkstra<const MIN_STEP: usize, const MAX_STEP: usize>(
    costs: &Vec<Vec<usize>>,
    start: (usize, usize),
    target: (usize, usize),
) -> Option<usize> {
    let mut distance = vec![vec![[usize::MAX; 4]; costs[0].len()]; costs.len()];
    distance[0][0] = [0; 4];

    let mut frontier: BinaryHeap<Step> = BinaryHeap::new();

    frontier.push(Step {
        position: start,
        direction: Direction::Right,
        len: 0,
        cost: 0,
    });
    frontier.push(Step {
        position: start,
        direction: Direction::Down,
        len: 0,
        cost: 0,
    });

    let mut visited: HashSet<Step> = HashSet::new();

    let mut prev: HashMap<Step, Step> = HashMap::new();

    while let Some(current_step) = frontier.pop() {
        if visited.contains(&current_step) {
            continue;
        }

        visited.insert(current_step);

        if current_step.position == target && current_step.len >= MIN_STEP - 1 {
            let mut g = costs
                .iter()
                .clone()
                .map(|line| {
                    line.iter()
                        .map(|&d| char::from_digit(d as u32, 10).unwrap())
                        .collect_vec()
                })
                .collect_vec();

            let mut step = current_step;
            while let Some(s) = prev.get(&step) {
                // println!("{:?}", step);
                g[step.position.0][step.position.1] = match step.direction {
                    Direction::Up => '^',
                    Direction::Down => 'v',
                    Direction::Left => '<',
                    Direction::Right => '>',
                };
                step = *s;
            }
            g[step.position.0][step.position.1] = match step.direction {
                Direction::Up => '^',
                Direction::Down => 'v',
                Direction::Left => '<',
                Direction::Right => '>',
            };

            println!(
                "{}",
                g.iter()
                    .map(|line| line.iter().map(|d| format!("{}", d)).join(""))
                    .join("\n")
            );
            return Some(current_step.cost);
        }

        if current_step.len >= MIN_STEP - 1 {
            let new_direction = current_step.direction.trigonometric_rotation();
            if let Some((next_row, next_col)) =
                new_direction.apply(current_step.position, (costs.len(), costs[0].len()))
            {
                let new_cost = current_step.cost + costs[next_row][next_col];
                if new_cost < distance[next_row][next_col][new_direction as usize] {
                    distance[next_row][next_col][new_direction as usize] = new_cost;
                    let left_step = Step {
                        position: (next_row, next_col),
                        direction: new_direction,
                        len: 0,
                        cost: new_cost,
                    };

                    prev.insert(left_step.clone(), current_step.clone());
                    frontier.push(left_step);
                }
            }
        }

        if current_step.len >= MIN_STEP - 1 {
            let new_direction = current_step.direction.antitrigonometric_rotation();
            if let Some((next_row, next_col)) =
                new_direction.apply(current_step.position, (costs.len(), costs[0].len()))
            {
                let new_cost = current_step.cost + costs[next_row][next_col];
                if new_cost < distance[next_row][next_col][new_direction as usize] {
                    distance[next_row][next_col][new_direction as usize] = new_cost;
                    let right_step = Step {
                        position: (next_row, next_col),
                        direction: current_step.direction.antitrigonometric_rotation(),
                        len: 0,
                        cost: new_cost,
                    };
                    prev.insert(right_step.clone(), current_step.clone());
                    frontier.push(right_step);
                }
            }
        }
        if current_step.len < MAX_STEP - 1 {
            if let Some((next_row, next_col)) = current_step
                .direction
                .apply(current_step.position, (costs.len(), costs[0].len()))
            {
                let new_cost = current_step.cost + costs[next_row][next_col];
                if new_cost < distance[next_row][next_col][current_step.direction as usize] {
                    distance[next_row][next_col][current_step.direction as usize] = new_cost;
                    let forward_step = Step {
                        position: (next_row, next_col),
                        direction: current_step.direction,
                        len: current_step.len + 1,
                        cost: current_step.cost + costs[next_row][next_col],
                    };

                    prev.insert(forward_step.clone(), current_step.clone());
                    frontier.push(forward_step);
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

        // 1055 too low

        assert_eq!(part_one(input), 102);
        panic!()
    }
}
