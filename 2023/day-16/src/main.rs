use anyhow::{anyhow, Result};
use petgraph::graph::UnGraph;
use std::collections::HashSet;
use std::{env, fs, io::Read};

/* This solution is an abomination, and is probably way slower than it should be. But at least now
* I have an inkling of how to use petgraph...
*
* Basically what I do is:
*   Parse the input into an undirected graph representing all of the beams. The nodes contain the
*   coordinates (row, column) in the grid where the beam passes, and the edges coontain the
*   direction the beam takes to get to the next node.
*   Let's say we start on the edge of the grid at coordinates (r, c) with direction d. We keep
*   track of a list of `leaf` nodes, that is, the nodes from which we continue to expand the graph,
*   as well as a HashSet that contains the tiles we visited, and the direction we came from.
*   Nodes in the visisted set are never added to the leaves list.
*
* This runs in ~20s for Part 2, which is a bit embarassing. Maybe I'll find a way to optimize this
* (or maybe a better solution altogether).
*/

fn main() -> Result<()> {
    let input_file = env::args().nth(1).unwrap_or("./day-16.in".to_owned());
    let mut input = String::new();
    fs::File::open(&input_file)?.read_to_string(&mut input)?;
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));

    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Space,
    LeftMirror,
    RightMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Space),
            '|' => Ok(Self::VerticalSplitter),
            '-' => Ok(Self::HorizontalSplitter),
            '/' => Ok(Self::RightMirror),
            '\\' => Ok(Self::LeftMirror),
            _ => Err(anyhow!("Invalid tile")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn beam_graph(
    grid: &Vec<Vec<Tile>>,
    start: (usize, usize, Direction),
) -> UnGraph<(usize, usize), Direction> {
    let mut beam = UnGraph::default();

    let (row, col, dir) = start;
    let root = beam.add_node((row, col));
    let edge = beam.add_edge(root, root, dir);

    let mut beam_heads = vec![(edge, root)];
    let mut visited: HashSet<(usize, usize, Direction)> = HashSet::new();

    loop {
        let mut new_heads = Vec::new();
        let mut visited_new_node = false;
        for &(came_from, head) in &beam_heads {
            let (row, col) = beam[head];
            let current_direction = beam[came_from];

            visited_new_node = visited_new_node || visited.insert((row, col, current_direction));

            use Direction::*;
            let new_directions = match grid[row][col] {
                Tile::Space => vec![current_direction],
                Tile::LeftMirror => vec![match current_direction {
                    Up => Left,
                    Down => Right,
                    Left => Up,
                    Right => Down,
                }],
                Tile::RightMirror => vec![match current_direction {
                    Up => Right,
                    Down => Left,
                    Left => Down,
                    Right => Up,
                }],
                Tile::VerticalSplitter => match current_direction {
                    Up | Down => vec![current_direction],
                    Left | Right => vec![Up, Down],
                },
                Tile::HorizontalSplitter => match current_direction {
                    Up | Down => vec![Left, Right],
                    Left | Right => vec![current_direction],
                },
            };

            for new_direction in new_directions {
                if let Some(next_position) = match new_direction {
                    Up => row.checked_sub(1).map(|r| (r, col)),
                    Down => (row + 1 < grid.len()).then_some((row + 1, col)),
                    Left => col.checked_sub(1).map(|c| (row, c)),
                    Right => (col + 1 < grid[row].len()).then_some((row, col + 1)),
                } {
                    if let Some(new_head) = beam
                        .node_indices()
                        .find(|&index| beam[index] == next_position)
                    {
                        if !beam
                            .edges_connecting(head, new_head)
                            .any(|edge| edge.weight() == &new_direction)
                        {
                            let new_edge = beam.add_edge(head, new_head, new_direction);
                            new_heads.push((new_edge, new_head));
                        }
                    } else {
                        let new_node = beam.add_node(next_position);
                        let new_edge = beam.add_edge(head, new_node, new_direction);

                        new_heads.push((new_edge, new_node));
                    }
                }
            }
        }

        beam_heads = new_heads;

        if !visited_new_node {
            break;
        }
    }

    beam
}

fn part_one(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().map(|ch| Tile::try_from(ch)).collect())
        .collect::<Result<Vec<Vec<Tile>>>>()
        .unwrap();

    beam_graph(&grid, (0, 0, Direction::Right)).node_count()
}

fn part_two(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().map(|ch| Tile::try_from(ch)).collect())
        .collect::<Result<Vec<Vec<Tile>>>>()
        .unwrap();

    let (width, height) = (grid[0].len(), grid.len());
    let vertical = (0..grid.len())
        .map(|i| {
            let left = beam_graph(&grid, (i, 0, Direction::Right)).node_count();
            let right = beam_graph(&grid, (i, width - 1, Direction::Left)).node_count();

            left.max(right)
        })
        .max()
        .unwrap();

    let horizontal = (0..grid[0].len())
        .map(|j| {
            let up = beam_graph(&grid, (0, j, Direction::Down)).node_count();
            let down = beam_graph(&grid, (height - 1, j, Direction::Up)).node_count();

            up.max(down)
        })
        .max()
        .unwrap();

    vertical.max(horizontal)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::*;

    #[test]
    fn test_part_one() {
        let input = indoc! {r#"
            .|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|....
        "#};

        assert_eq!(part_one(input), 46);
    }

    #[test]
    fn test_part_two() {
        let input = indoc! {r#"
            .|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|....
        "#};

        assert_eq!(part_two(input), 51);
    }
}
