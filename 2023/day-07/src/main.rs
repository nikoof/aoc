use anyhow::Result;
use hand::Hand;
use itertools::Itertools;
use joker_hand::JokerHand;
use std::{env, fs, io::Read};

mod hand;
mod joker_hand;

fn main() -> Result<()> {
    let input_file = env::args().nth(1).unwrap_or("./day-07.in".to_owned());
    let mut input = String::new();
    fs::File::open(&input_file)?.read_to_string(&mut input)?;
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));

    Ok(())
}

fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(" ").unwrap();

            (hand.parse::<Hand>().unwrap(), bid.parse::<usize>().unwrap())
        })
        .sorted_by(|(h1, _), (h2, _)| h1.cmp(h2))
        .enumerate()
        .map(|(i, (_, b))| (i + 1) * b)
        .sum()
}

fn part_two(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(" ").unwrap();

            (
                hand.parse::<JokerHand>().unwrap(),
                bid.parse::<usize>().unwrap(),
            )
        })
        .sorted_by(|(h1, _), (h2, _)| h1.cmp(h2))
        .enumerate()
        .map(|(i, (_, b))| (i + 1) * b)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part_one() {
        let input = indoc! {"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "};

        assert_eq!(6440, part_one(&input));
    }

    #[test]
    fn test_part_two() {
        let input = indoc! {"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "};

        assert_eq!(5905, part_two(&input));
    }
}
