use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::collections::HashMap;
use std::{collections::HashSet, convert::identity, env, fs, io::Read};

fn main() -> Result<()> {
    let input_file = env::args().nth(1).unwrap_or("./day-19.in".to_owned());
    let mut input = String::new();
    fs::File::open(&input_file)?.read_to_string(&mut input)?;
    println!("Part 1: {}", part_one(&input, 64));
    println!("Part 2: {}", part_two(&input, 26501365));

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

    for _ in 0..steps {
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
        println!("{:?}", heads);

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

    traverse(&map, (start_pos / width, start_pos % width), steps)
}

fn wrapping_traverse(
    map: &Vec<Vec<Tile>>,
    start_pos: (usize, usize),
    steps: usize,
    cache: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    if steps == 0 || map[start_pos.0][start_pos.1] == Tile::Rock {
        return 0;
    }

    if let Some(result) = cache.get(&(start_pos.0, start_pos.1, steps)) {
        return *result;
    }

    let (width, height) = (map[0].len(), map.len());
    let mut heads = HashSet::new();
    heads.insert(start_pos);

    let mut total = 0;

    for i in 1..=steps {
        let mut new_heads = HashSet::new();
        for &(row, col) in &heads {
            if row == 0 {
                total += wrapping_traverse(map, (height - 1, col), steps - i, cache);
                // total += wrapping_traverse(map, (row, col), steps - i, cache);
            } else {
                new_heads.insert((row - 1, col));
            }

            if row == height - 1 {
                total += wrapping_traverse(map, (0, col), steps - i, cache);
                // total += wrapping_traverse(map, (row, col), steps - i, cache);
            } else {
                new_heads.insert((row + 1, col));
            }

            if col == 0 {
                total += wrapping_traverse(map, (row, width - 1), steps - i, cache);
                // total += wrapping_traverse(map, (row, col), steps - i, cache);
            } else {
                new_heads.insert((row, col - 1));
            }

            if col == width - 1 {
                total += wrapping_traverse(map, (row, 0), steps - i, cache);
                // total += wrapping_traverse(map, (row, col), steps - i, cache);
            } else {
                new_heads.insert((row, col + 1));
            }
        }

        heads = new_heads
            .into_iter()
            .filter(|&(row, col)| map[row][col] != Tile::Rock)
            .collect();
    }

    cache.insert((start_pos.0, start_pos.1, steps), heads.len() + total);

    print!("{:?}\t{}", start_pos, steps);
    println!("\t{}", heads.len() + total);

    heads.len() + total
}

fn part_two(input: &str, steps: usize) -> usize {
    let map = parse_map(input).expect("Should have a valid input");

    let (width, height) = (map[0].len(), map.len());

    let start_pos = map
        .iter()
        .flatten()
        .position(|&tile| tile == Tile::Start)
        .expect("Should have a starting position");

    let start_pos = (start_pos / width, start_pos % width);

    let mut cache = HashMap::new();
    wrapping_traverse(&map, start_pos, steps, &mut cache)
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

    #[test]
    fn test1() {
        let example_map = indoc! {"
            ......................
            .....###.#......###.#.
            .###.##..#..###.##..#.
            ..#.#...#....#.#...#..
            ....#.#...S....#.#....
            .##...####..##...####.
            .##..#...#..##..#...#.
            .......##.........##..
            .##.#.####..##.#.####.
            .##..##.##..##..##.##.
            ......................
        "};

        println!("{}", part_one(example_map, 3));
        panic!()
    }

    #[rstest]
    #[case(EXAMPLE_MAP, 6, 16)]
    fn test_part_one(#[case] input: &str, #[case] steps: usize, #[case] expected: usize) {
        assert_eq!(part_one(input, steps), expected);
    }

    #[rstest]
    #[case(EXAMPLE_MAP, 6, 16)]
    #[case(EXAMPLE_MAP, 10, 50)]
    // #[case(EXAMPLE_MAP, 50, 1594)]
    // #[case(EXAMPLE_MAP, 100, 6536)]
    // #[case(EXAMPLE_MAP, 500, 167004)]
    // #[case(EXAMPLE_MAP, 1000, 668697)]
    // #[case(EXAMPLE_MAP, 5000, 16733044)]
    fn test_part_two(#[case] input: &str, #[case] steps: usize, #[case] expected: usize) {
        assert_eq!(part_two(input, steps), expected);
    }
}
