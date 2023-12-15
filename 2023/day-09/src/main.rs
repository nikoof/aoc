use anyhow::Result;
use itertools::*;
use std::{env, fs, io::Read};

fn main() -> Result<()> {
    let input_file = env::args().nth(1).unwrap_or("./day-09.in".to_owned());
    let mut input = String::new();
    fs::File::open(&input_file)?.read_to_string(&mut input)?;
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));

    Ok(())
}

fn extrapolate_sequence(sequence: &[i32]) -> i32 {
    let mut sequence = sequence.to_owned();
    let mut terms = vec![sequence[sequence.len() - 1]];
    loop {
        sequence = sequence
            .iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect_vec();

        terms.push(sequence[sequence.len() - 1]);

        if sequence.iter().all_equal() {
            break;
        }
    }

    terms.iter().sum()
}

fn part_one(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect_vec()
        })
        .map(|v| extrapolate_sequence(&v))
        .sum()
}

fn part_two(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .rev()
                .collect_vec()
        })
        .map(|v| extrapolate_sequence(&v))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part_one() {
        let input = indoc! {"
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        "};

        assert_eq!(114, part_one(input));
    }

    #[test]
    fn test_part_two() {
        let input = indoc! {"
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        "};

        assert_eq!(2, part_two(input));
    }
}
