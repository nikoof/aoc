use anyhow::Result;
use itertools::*;
use std::{env, fs, io::Read};

fn main() -> Result<()> {
    let input_file = env::args().nth(1).unwrap_or("./day-08.in".to_owned());
    let mut input = String::new();
    fs::File::open(&input_file)?.read_to_string(&mut input)?;
    println!("Part 1: {}", part_one(&input));
    // println!("Part 2: {}", part_two(&input));

    Ok(())
}

fn expand_universe(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let grid = grid
        .into_iter()
        .map(|line| {
            if line.iter().all(|&e| e == '.') {
                vec![line.clone(), line.clone()]
            } else {
                vec![line.clone()]
            }
        })
        .flatten()
        .collect_vec();

    let transpose = (0..grid[0].len())
        .map(|i| grid.iter().map(|row| row[i]).collect_vec())
        .map(|line| {
            if line.iter().all(|&e| e == '.') {
                vec![line.clone(), line.clone()]
            } else {
                vec![line.clone()]
            }
        })
        .flatten()
        .collect_vec();

    (0..transpose[0].len())
        .map(|i| transpose.iter().map(|row| row[i]).collect_vec())
        .collect_vec()
}

fn part_one(input: &str) -> isize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let grid = expand_universe(grid);

    // println!("{}", input);
    // println!(
    //     "{}",
    //     expand_galaxy(grid)
    //         .into_iter()
    //         .map(|line| line.iter().join(""))
    //         .join("\n")
    // );

    let galaxies = grid
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, &col)| col == '#')
                .map(move |(j, _)| (i as isize, j as isize))
                .collect_vec()
        })
        .flatten()
        .collect_vec();

    galaxies
        .iter()
        .tuple_combinations()
        .map(|((y1, x1), (y2, x2))| (x2 - x1).abs() + (y2 - y1).abs())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use rstest::rstest;

    #[rstest]
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
    "}, 374)]
    fn test_part_one(#[case] input: &str, #[case] expected: isize) {
        assert_eq!(expected, part_one(input));
    }

    // #[test]
    // fn test_part_two() {
    //     let input = indoc! {"
    //         0 3 6 9 12 15
    //         1 3 6 10 15 21
    //         10 13 16 21 30 45
    //     "};
    //
    //     assert_eq!(2, part_two(input));
    // }
}
