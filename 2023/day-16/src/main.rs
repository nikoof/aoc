use anyhow::{anyhow, Result};
use petgraph::graph::UnGraph;
use std::collections::HashSet;
use std::{env, fs, io::Read, str::FromStr};

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

#[derive(Debug, PartialEq, Eq)]
struct Grid(Vec<Vec<Tile>>);

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|line| line.chars().map(|ch| Tile::try_from(ch)).collect())
                .collect::<Result<Vec<Vec<Tile>>>>()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn beam_graph(grid: &Grid, start: (usize, usize, Direction)) -> UnGraph<(usize, usize), Direction> {
    let mut graph = UnGraph::default();

    let (row, col, dir) = start;
    let root = graph.add_node((row, col));
    let edge = graph.add_edge(root, root, dir);

    let mut leaves = vec![(edge, root)];
    let mut visited: HashSet<(usize, usize, Direction)> = HashSet::new();

    loop {
        let mut new_leaves = Vec::new();
        let mut visited_new_node = false;
        for &(edge, leaf) in &leaves {
            visited_new_node =
                visited_new_node || visited.insert((graph[leaf].0, graph[leaf].1, graph[edge]));

            let (row, col) = graph[leaf];
            let current_direction = graph[edge];

            use Direction::*;
            let new_directions = match grid.0[row][col] {
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

            for direction in new_directions {
                let new_position = match direction {
                    Up => row.checked_sub(1).map(|r| (r, col)),
                    Down => (row + 1 < grid.0.len()).then_some((row + 1, col)),
                    Left => col.checked_sub(1).map(|c| (row, c)),
                    Right => (col + 1 < grid.0[row].len()).then_some((row, col + 1)),
                };

                if let Some(new_position) = new_position {
                    if let Some(dest) = graph
                        .node_indices()
                        .find(|&index| graph[index] == new_position)
                    {
                        if let None = graph
                            .edge_indices()
                            .filter(|&e| graph.edge_endpoints(e).unwrap() == (leaf, dest))
                            .find(|&e| graph[e] == direction)
                        {
                            let new_edge = graph.add_edge(leaf, dest, direction);
                            new_leaves.push((new_edge, dest));
                        }
                    } else {
                        let new_node = graph.add_node(new_position);
                        let new_edge = graph.add_edge(leaf, new_node, direction);

                        new_leaves.push((new_edge, new_node));
                    }
                }
            }
        }

        if !visited_new_node {
            break;
        }

        leaves = new_leaves;
    }

    graph
}

fn part_one(input: &str) -> usize {
    let grid = Grid::from_str(input).unwrap();

    beam_graph(&grid, (0, 0, Direction::Right)).node_count()
}

fn part_two(input: &str) -> usize {
    let grid = Grid::from_str(input).unwrap();

    (0..grid.0.len())
        .map(|i| {
            beam_graph(&grid, (i, 0, Direction::Right))
                .node_count()
                .max(beam_graph(&grid, (i, grid.0[0].len() - 1, Direction::Left)).node_count())
        })
        .max()
        .unwrap()
        .max(
            (0..grid.0[0].len())
                .map(|j| {
                    beam_graph(&grid, (0, j, Direction::Down))
                        .node_count()
                        .max(beam_graph(&grid, (grid.0.len() - 1, j, Direction::Up)).node_count())
                })
                .max()
                .unwrap(),
        )
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
