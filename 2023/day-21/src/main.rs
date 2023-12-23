use anyhow::{anyhow, Result};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    convert::identity,
    env, fs,
    io::Read,
};

/* Reference:
* https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
*
* I was so sure that there is a nice geometric solution but I thought it wouldn't work for the general
* case. It turns out that it works for the input. Advent of Code moment. */

fn main() -> Result<()> {
    let input_file = env::args().nth(1).unwrap_or("./day-19.in".to_owned());
    let mut input = String::new();
    fs::File::open(&input_file)?.read_to_string(&mut input)?;
    println!("Part 1: {}", part_one(&input, 64));
    println!("Part 2: {}", part_two(&input));

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Start,
    Space,
    Rock,
}

fn parse_map(input: &str) -> Result<Vec<Vec<Tile>>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| {
                    match ch {
                        '.' => Some(Tile::Space),
                        '#' => Some(Tile::Rock),
                        'S' => Some(Tile::Start),
                        _ => None,
                    }
                    .ok_or(anyhow!("Invalid input"))
                })
                .collect()
        })
        .collect()
}

fn traverse(map: &Vec<Vec<Tile>>, start_pos: (usize, usize), steps: usize) -> usize {
    let (width, height) = (map[0].len(), map.len());
    let mut heads = HashSet::new();
    heads.insert(start_pos);

    for _ in 1..=steps {
        let mut new_heads = HashSet::new();
        for &(row, col) in &heads {
            new_heads.extend(
                [
                    row.checked_sub(1).map(|r| (r, col)),
                    (row + 1 < height).then_some((row + 1, col)),
                    col.checked_sub(1).map(|c| (row, c)),
                    (col + 1 < width).then_some((row, col + 1)),
                ]
                .into_iter()
                .filter_map(identity)
                .filter(|&(row, col)| map[row][col] != Tile::Rock),
            );
        }
        heads = new_heads;
    }

    heads.len()
}

fn part_one(input: &str, steps: usize) -> usize {
    let map = parse_map(input).expect("Should have a valid input");

    let (width, _height) = (map[0].len(), map.len());

    let start_pos = map
        .iter()
        .flatten()
        .position(|&tile| tile == Tile::Start)
        .expect("Should have a starting position");

    let start_pos = (start_pos / width, start_pos % width);

    traverse(&map, start_pos, steps)
}

fn part_two(input: &str) -> usize {
    let map = parse_map(input).expect("Should have a valid input");

    let (width, height) = (map[0].len(), map.len());

    let start_pos = map
        .iter()
        .flatten()
        .position(|&tile| tile == Tile::Start)
        .expect("Should have a starting position");

    let start_pos = (start_pos / width, start_pos % width);

    let mut frontier = VecDeque::<(usize, (usize, usize))>::new();
    let mut visited = HashMap::new();

    frontier.push_back((0, start_pos));

    while let Some((dist, coord)) = frontier.pop_front() {
        if visited.contains_key(&coord) {
            continue;
        }

        visited.insert(coord, dist);

        let (row, col) = coord;
        frontier.extend(
            [
                row.checked_sub(1).map(|r| (r, col)),
                (row + 1 < height).then_some((row + 1, col)),
                col.checked_sub(1).map(|c| (row, c)),
                (col + 1 < width).then_some((row, col + 1)),
            ]
            .into_iter()
            .filter_map(identity)
            .filter(|&(row, col)| map[row][col] != Tile::Rock)
            .map(|position| (dist + 1, position)),
        );
    }

    let even_corners = visited
        .values()
        .filter(|v| **v % 2 == 0 && **v > 65)
        .count();
    let odd_corners = visited
        .values()
        .filter(|v| **v % 2 == 1 && **v > 65)
        .count();

    let n = ((26501365 - (height / 2)) / height) as usize;
    assert_eq!(n, 202300);

    let even = n * n;
    let odd = (n + 1) * (n + 1);

    odd * visited.values().filter(|v| **v % 2 == 1).count()
        + even * visited.values().filter(|v| **v % 2 == 0).count()
        - ((n + 1) * odd_corners)
        + (n * even_corners)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::*;
    use rstest::*;

    const EXAMPLE_MAP: &str = indoc! {"
        ...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........
    "};

    #[rstest]
    #[case(EXAMPLE_MAP, 6, 16)]
    fn test_part_one(#[case] input: &str, #[case] steps: usize, #[case] expected: usize) {
        assert_eq!(part_one(input, steps), expected);
    }
}
