use anyhow::Result;
use itertools::*;
use std::{env, fs, io::Read};

fn main() -> Result<()> {
    let input_file = env::args().nth(1).unwrap_or("./day-11.in".to_owned());
    let mut input = String::new();
    fs::File::open(&input_file)?.read_to_string(&mut input)?;
    println!("Part 1: {}", solve(&input, 2));
    println!("Part 2: {}", solve(&input, 1000000));

    Ok(())
}

/*
* A bit of an explanation for me in the future.
* To deal with arbitrary (very large) scaling
* factors, I represent the universe as a grid of `Tile`s, where each Tile can be either a `Galaxy`
* or empty `Space`. Each variant stores it's "length".
* I first parse the input into a 2d-array of `Tile`s, then I expand it by updating each line and
* column that is comprised only of `Space`s with `Space`s with length == scaling_factor.
*
* Take for instance the following case and its representation:
* .#.      Space(1)  Galaxy(1) Space(1)
* ... ---> Space(1)  Space(1)  Space(1) <- This row should be expanded
* #..      Galaxy(1) Space(1)  Space(1)
*                                ^-------- This column should also be expanded
*
* Which, after expanding with a scaling factor of 2 becomes:
* Space(1)  Galaxy(1) Space(2)      .#..
* Space(2)  Space(2)  Space(2) ---> .... <-- These rows were expanded
* Galaxy(1) Space(1)  Space(2)      .... <-^
*                                   #...
*                                     ^^---- These columns were expanded
*
* The distance between the galaxies at coordinates (y1, x1) and (y2, x2) respectively is the sum dx + dy,
* where dx = sum(tiles between x1 and x2) and dy = sum(tiles between y1 and y2).
*/

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Tile {
    Galaxy(usize),
    Space(usize),
}

fn expand_universe(universe: &mut Vec<Vec<Tile>>, factor: usize) {
    for row in universe.iter_mut() {
        if row.iter().all(|tile| match tile {
            Tile::Space(_) => true,
            _ => false,
        }) {
            for tile in row.iter_mut() {
                *tile = Tile::Space(factor);
            }
        }
    }

    for j in 0..universe[0].len() {
        if universe.iter().all(|tile| match tile[j] {
            Tile::Space(_) => true,
            _ => false,
        }) {
            for i in 0..universe.len() {
                universe[i][j] = Tile::Space(factor);
            }
        }
    }
}

fn solve(input: &str, factor: usize) -> usize {
    let mut universe = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|character| {
                    match character {
                        '#' => Some(Tile::Galaxy(1)),
                        '.' => Some(Tile::Space(1)),
                        _ => None,
                    }
                    .unwrap()
                })
                .collect_vec()
        })
        .collect_vec();

    // Maybe there is a way to avoid mutating here...
    expand_universe(&mut universe, factor);

    let galaxies = universe
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, &col)| match col {
                    Tile::Galaxy(_) => true,
                    _ => false,
                })
                .map(move |(j, _)| (i, j))
                .collect_vec()
        })
        .flatten()
        .collect_vec();

    galaxies
        .iter()
        .tuple_combinations()
        .map(|(&(y1, x1), &(y2, x2))| {
            let dx: usize = universe[y1][x1.min(x2)..x1.max(x2)]
                .iter()
                .map(|tile| match tile {
                    Tile::Space(n) => n,
                    Tile::Galaxy(n) => n,
                })
                .sum();

            let dy: usize = (y1.min(y2)..y1.max(y2))
                .map(|y| match universe[y][x1] {
                    Tile::Galaxy(n) => n,
                    Tile::Space(n) => n,
                })
                .sum();

            dx + dy
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use rstest::rstest;

    #[rstest]
    #[case(indoc! {"
        .#.
        ...
        #..
    "}, 2, 4)]
    #[case(indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "}, 2, 374)]
    #[case(indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "}, 10, 1030)]
    #[case(indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "}, 100, 8410)]
    fn test_solution(#[case] input: &str, #[case] scaling_factor: usize, #[case] expected: usize) {
        assert_eq!(expected, solve(input, scaling_factor));
    }
}
