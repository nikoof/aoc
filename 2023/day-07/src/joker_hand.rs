use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::{cmp::Ordering, str::FromStr};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum JokerHand {
    FiveKind(u32),
    FourKind(u32),
    FullHouse(u32),
    ThreeKind(u32),
    TwoPair(u32),
    OnePair(u32),
    HighCard(u32),
}

impl JokerHand {
    pub fn order(&self) -> u32 {
        match self {
            JokerHand::FiveKind(_) => 7,
            JokerHand::FourKind(_) => 6,
            JokerHand::FullHouse(_) => 5,
            JokerHand::ThreeKind(_) => 4,
            JokerHand::TwoPair(_) => 3,
            JokerHand::OnePair(_) => 2,
            JokerHand::HighCard(_) => 1,
        }
    }

    pub fn value(&self) -> u32 {
        match *self {
            JokerHand::FiveKind(value) => value,
            JokerHand::FourKind(value) => value,
            JokerHand::FullHouse(value) => value,
            JokerHand::ThreeKind(value) => value,
            JokerHand::TwoPair(value) => value,
            JokerHand::OnePair(value) => value,
            JokerHand::HighCard(value) => value,
        }
    }
}

impl Ord for JokerHand {
    fn cmp(&self, other: &Self) -> Ordering {
        let res = self.order().cmp(&other.order());
        match res {
            Ordering::Equal => self.value().cmp(&other.value()),
            _ => res,
        }
    }
}

impl FromStr for JokerHand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 5 {
            return Err(anyhow!("Hand too long"));
        }

        let digits = s
            .chars()
            .map(|c| match c {
                'J' => Ok(0),
                character if character.is_digit(10) => character
                    .to_digit(10)
                    .map(|d| d - 1)
                    .ok_or(anyhow!("Invalid card label")),
                'T' => Ok(9),
                'Q' => Ok(10),
                'K' => Ok(11),
                'A' => Ok(12),
                _ => Err(anyhow!("Invalid card label")),
            })
            .collect::<Result<Vec<u32>>>()?;

        let numeric_representation = digits.iter().fold(0, |acc, digit| acc * 13 + digit);

        let mut freq = [0; 13];
        digits.iter().for_each(|&digit| freq[digit as usize] += 1);
        let pmax = freq[1..].iter().position_max().unwrap();
        freq[1..][pmax] += freq[0];

        freq.iter().enumerate().count();

        let freq_counts = (0..=5)
            .map(|digit_freq| freq[1..].into_iter().filter(|&&e| e == digit_freq).count())
            .enumerate()
            .map(|(_, e)| e)
            .collect_vec();

        if freq_counts[5] == 1 {
            Ok(JokerHand::FiveKind(numeric_representation))
        } else if freq_counts[4] == 1 {
            Ok(JokerHand::FourKind(numeric_representation))
        } else if freq_counts[3] == 1 {
            if freq_counts[2] == 1 {
                Ok(JokerHand::FullHouse(numeric_representation))
            } else {
                Ok(JokerHand::ThreeKind(numeric_representation))
            }
        } else if freq_counts[2] == 2 {
            Ok(JokerHand::TwoPair(numeric_representation))
        } else if freq_counts[2] == 1 {
            Ok(JokerHand::OnePair(numeric_representation))
        } else {
            Ok(JokerHand::HighCard(numeric_representation))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("23456", JokerHand::HighCard(33519))]
    #[case("KK678", JokerHand::OnePair(339268))]
    #[case("KK677", JokerHand::TwoPair(339267))]
    #[case("55522", JokerHand::FullHouse(123722))]
    #[case("55523", JokerHand::ThreeKind(123723))]
    #[case("AA8AA", JokerHand::FourKind(370447))]
    #[case("AAAAA", JokerHand::FiveKind(371292))]
    #[case("JJJAA", JokerHand::FiveKind(168))]
    #[case("JAAAA", JokerHand::FiveKind(28560))]
    #[case("2345J", JokerHand::OnePair(33514))]
    #[case("JJJJJ", JokerHand::FiveKind(0))]
    #[case("JJJJ3", JokerHand::FiveKind(2))]
    #[case("JJJ23", JokerHand::FourKind(15))]
    #[case("JJ234", JokerHand::ThreeKind(198))]
    fn test_hand_parse(#[case] h: &str, #[case] expected: JokerHand) {
        assert_eq!(h.parse::<JokerHand>().unwrap(), expected);
    }

    #[rstest]
    #[case("KK677", "KTJJT", Ordering::Less)]
    #[case("T55J5", "QQQJA", Ordering::Less)]
    #[case("KK677", "QQQJA", Ordering::Less)]
    #[case("QQQJA", "KTJJT", Ordering::Less)]
    #[case("22222", "KQJT9", Ordering::Greater)]
    fn test_hand_cmp(#[case] h1: JokerHand, #[case] h2: JokerHand, #[case] expected: Ordering) {
        assert_eq!(h1.cmp(&h2), expected);
    }
}
