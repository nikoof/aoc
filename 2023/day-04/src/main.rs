use anyhow::Result;
use std::{collections::BTreeSet, env, fs, io::Read};

fn main() -> Result<()> {
    let input_file = env::args().nth(1).unwrap_or("./day-04.in".to_owned());
    let mut input = String::new();
    fs::File::open(&input_file)?.read_to_string(&mut input)?;
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));

    Ok(())
}

fn card_score(card: &str) -> u32 {
    let (_, numbers) = card
        .split_once(": ")
        .expect("line should have correct format");

    let (winning_numbers, given_numbers) = numbers
        .split_once(" | ")
        .expect("line should have correct format");

    let winning_numbers = winning_numbers
        .split_whitespace()
        .map(|num| {
            num.parse::<u32>()
                .expect(&format!("{num} should be a number"))
        })
        .collect::<BTreeSet<u32>>();

    let given_numbers = given_numbers
        .split_whitespace()
        .map(|num| {
            num.parse::<u32>()
                .expect(&format!("{num} should be a number"))
        })
        .collect::<BTreeSet<u32>>();

    winning_numbers.intersection(&given_numbers).count() as u32
}

fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(card_score)
        .map(|score| if score != 0 { 2u32.pow(score - 1) } else { 0 })
        .sum()
}

fn part_two(input: &str) -> u32 {
    let mut card_counts: Vec<u32> = vec![
        1;
        input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .count()
    ];

    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(card_score)
        .enumerate()
        .for_each(|(card, score)| {
            let card_count = card_counts[card];
            card_counts[card + 1..=card + score as usize]
                .iter_mut()
                .for_each(|count| *count = *count + card_count);
        });

    card_counts.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("Card 2: 41 48 83 86 17  | 83 86  6 31 17  9 48 53", 4)]
    #[case("Card 2: 13 32 20 16 61   |        61 30 68 82 17 32 24 19", 2)]
    #[case("Card 3:  1 21 53 59 44 | 69      82 63 72 16 21 14  1", 2)]
    #[case("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1)]
    #[case("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22       82 36", 0)]
    #[case("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0)]
    fn test_card_score(#[case] card: &str, #[case] expected: u32) {
        assert_eq!(expected, card_score(card));
    }

    #[test]
    fn test_part_one() {
        let input = r#"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "#;

        assert_eq!(13, part_one(&input));
    }

    #[test]
    fn test_part_two() {
        let input = r#"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "#;

        assert_eq!(30, part_two(&input));
    }
}
