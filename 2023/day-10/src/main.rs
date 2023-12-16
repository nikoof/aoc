use anyhow::Result;
use itertools::*;
use std::collections::BTreeSet;
use std::{env, fs, io::Read};

fn main() -> Result<()> {
    let input_file = env::args().nth(1).unwrap_or("./day-08.in".to_owned());
    let mut input = String::new();
    fs::File::open(&input_file)?.read_to_string(&mut input)?;
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));

    Ok(())
}

fn find_cycle(
    tiles: &Vec<Vec<char>>,
    current_tile: (usize, usize),
    previous_tile: Option<(usize, usize)>,
    visited: &mut BTreeSet<(usize, usize)>,
    cycle: &mut Vec<(usize, usize)>,
) -> bool {
    visited.insert(current_tile);
    cycle.push(current_tile);

    let (x, y) = current_tile;
    let neighbors = match tiles[y][x] {
        '|' => vec![(x, y.saturating_sub(1)), (x, (y + 1).min(tiles[0].len()))],
        '-' => vec![(x.saturating_sub(1), y), ((x + 1).min(tiles[0].len()), y)],
        'L' => vec![(x, y.saturating_sub(1)), ((x + 1).min(tiles[0].len()), y)],
        'J' => vec![(x, y.saturating_sub(1)), (x - 1, y)],
        '7' => vec![(x, (y + 1).min(tiles[0].len())), (x.saturating_sub(1), y)],
        'F' => vec![
            (x, (y + 1).min(tiles[0].len())),
            ((x + 1).min(tiles[0].len()), y),
        ],
        _ => vec![],
    };

    for neighbor in neighbors {
        if !visited.contains(&neighbor) {
            return find_cycle(tiles, neighbor, Some(current_tile), visited, cycle);
        } else if let Some(parent) = previous_tile {
            if parent != neighbor {
                cycle.push(neighbor);
                return true;
            }
        }
    }

    cycle.pop();
    false
}

fn get_cycle(mut tiles: Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let start = tiles.iter().flatten().position(|&c| c == 'S').unwrap();
    let (x, y) = (start % tiles[0].len(), start / tiles[0].len());

    let up = match tiles[y.saturating_sub(1)][x] {
        '|' | 'F' | '7' => true,
        _ => false,
    };
    let down = match tiles[(y + 1).min(tiles.len())][x] {
        '|' | 'L' | 'J' => true,
        _ => false,
    };
    let left = match tiles[y][x.saturating_sub(1)] {
        '-' | 'F' | 'L' => true,
        _ => false,
    };
    let right = match tiles[y][(x + 1).min(tiles[0].len())] {
        '-' | '7' | 'J' => true,
        _ => false,
    };

    let s = match (up, down, left, right) {
        (true, true, false, false) => Some('|'),
        (true, false, true, false) => Some('J'),
        (true, false, false, true) => Some('L'),
        (false, true, true, false) => Some('F'),
        (false, true, false, true) => Some('7'),
        (false, false, true, true) => Some('-'),
        _ => None,
    }
    .unwrap();

    tiles[y][x] = s;

    let mut path: Vec<(usize, usize)> = Vec::new();
    let mut visited: BTreeSet<(usize, usize)> = BTreeSet::new();

    find_cycle(&tiles, (x, y), None, &mut visited, &mut path);

    path
}

fn shoelace(points: &Vec<(usize, usize)>) -> usize {
    let n = points.len();

    let points = points
        .iter()
        .map(|&(y, x)| (y as isize, x as isize))
        .collect_vec();

    ((0..n)
        .map(|i| (points[i].0 + points[(i + 1) % n].0) * (points[i].1 - points[(i + 1) % n].1))
        .sum::<isize>()
        / 2)
    .abs() as usize
}

fn part_one(input: &str) -> usize {
    let tiles = input
        .lines()
        .map(|line| line.trim().chars().collect_vec())
        .collect_vec();

    get_cycle(tiles).len() / 2
}

fn part_two(input: &str) -> usize {
    let tiles = input
        .lines()
        .map(|line| line.trim().chars().collect_vec())
        .collect_vec();

    let cycle = get_cycle(tiles);

    shoelace(&cycle) + 1 - cycle.len() / 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use rstest::rstest;

    #[rstest]
    #[case(indoc! {"
        -L|F7
        7S-7|
        L|7||
        -L-J|
        L|-JF
    "}, 4)]
    #[case(indoc! {"
        7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ
    "}, 8)]
    #[case(indoc! {"
        |S-7L
        |L-J-
    "}, 3)]
    fn test_part_one(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(expected, part_one(input));
    }

    #[rstest]
    #[case(indoc! {"
        ...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........
    "}, 4)]
    #[case(indoc! {"
        FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L
    "}, 10)]
    fn test_part_two(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(expected, part_two(input));
    }
}
