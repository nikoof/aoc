use anyhow::Result;
use itertools::Itertools;
use std::{env, fs, io::Read, iter::zip, ops::Not};

fn main() -> Result<()> {
    let input_file = env::args().nth(1).unwrap_or("./day-08.in".to_owned());
    let mut input = String::new();
    fs::File::open(&input_file)?.read_to_string(&mut input)?;
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Reflection {
    Row(usize),
    Column(usize),
}

fn transpose<T>(pattern: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Copy,
{
    (0..pattern[0].len())
        .map(|col| {
            pattern
                .into_iter()
                .map(|row| *row.get(col).unwrap())
                .collect()
        })
        .collect_vec()
}

fn reflections<T>(pattern: &Vec<Vec<T>>) -> Vec<Reflection>
where
    T: Copy + Eq + PartialEq,
{
    let mut reflects = Vec::new();
    for row in 1..pattern.len() {
        let above = &pattern[..row];
        let below = &pattern[row..];

        if zip(above.iter().rev(), below.iter())
            .fold(true, |acc, (a_row, b_row)| acc && a_row == b_row)
        {
            reflects.push(Reflection::Row(row))
        }
    }

    let transposition = transpose(pattern);
    for col in 1..transposition.len() {
        let above = &transposition[..col];
        let below = &transposition[col..];

        if zip(above.iter().rev(), below.iter())
            .fold(true, |acc, (a_row, b_row)| acc && a_row == b_row)
        {
            reflects.push(Reflection::Column(col))
        }
    }

    reflects
}

fn unsmudged_reflection<T>(pattern: &mut Vec<Vec<T>>) -> Option<Reflection>
where
    T: Copy + Eq + PartialEq + Not<Output = T> + std::fmt::Debug,
{
    let smudged_reflections = reflections(&pattern);

    // yikes... 😬
    for i in 0..pattern.len() {
        for j in 0..pattern[i].len() {
            pattern[i][j] = !pattern[i][j];

            if let Some(reflect) = reflections(&pattern)
                .iter()
                .find(|&r| r != smudged_reflections.first().unwrap())
            {
                return Some(*reflect);
            }

            pattern[i][j] = !pattern[i][j];
        }
    }

    None
}

fn part_one(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|pattern| {
            pattern
                .lines()
                .map(|line| line.chars().collect_vec())
                .collect_vec()
        })
        .map(|pattern| match reflections(&pattern).first() {
            Some(Reflection::Row(row)) => 100 * row,
            Some(Reflection::Column(col)) => *col,
            None => panic!("{:?}", pattern),
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|pattern| {
            pattern
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|ch| match ch {
                            '#' => true,
                            '.' => false,
                            _ => panic!(),
                        })
                        .collect_vec()
                })
                .collect_vec()
        })
        .map(|mut pattern| unsmudged_reflection(&mut pattern))
        .map(|reflect| match reflect {
            Some(Reflection::Row(row)) => 100 * row,
            Some(Reflection::Column(col)) => col,
            None => panic!(),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::*;
    use rstest::*;

    #[rstest]
    #[case(
        indoc! {"
        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
    "}, vec![Reflection::Row(4)])]
    #[case(
        indoc! {"
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.
    "}, vec![Reflection::Column(5)])]
    #[case(
        indoc! {"
        #...#..##
        #.#.#..##
        ###..##..
        #..##.###
        ..#.#.#..
        #####..##
        .#....#..
    "}, vec![Reflection::Column(8)])]
    fn test_reflection(#[case] pattern: &str, #[case] expected: Vec<Reflection>) {
        assert_eq!(
            reflections(&pattern.lines().map(|line| line.chars().collect()).collect()),
            expected
        );
    }

    #[rstest]
    #[case(
        indoc! {"
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.
    "}, Some(Reflection::Row(3)))]
    #[case(
        indoc! {"
        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
    "}, Some(Reflection::Row(1)))]
    #[case(
        indoc! {"
        ###.###.#.#
        ...##......
        ..#########
        ###.#......
        ##....#####
        ......#..##
        ###....#...
        ...##....##
        ##...#..###
    "}, Some(Reflection::Column(10)))]
    fn test_unsmudged_reflection(#[case] pattern: &str, #[case] expected: Option<Reflection>) {
        assert_eq!(
            unsmudged_reflection(
                &mut pattern
                    .lines()
                    .map(|line| {
                        line.chars()
                            .map(|ch| match ch {
                                '#' => true,
                                '.' => false,
                                _ => panic!(),
                            })
                            .collect_vec()
                    })
                    .collect_vec()
            ),
            expected
        );
    }

    #[test]
    fn test_part_one() {
        let input = indoc! {"
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.

            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
        "};

        assert_eq!(part_one(input), 405);
    }

    #[test]
    fn test_part_two() {
        let input = indoc! {"
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.

            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
        "};

        assert_eq!(part_two(input), 400);
    }
}
