use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::{cmp::Ordering, str::FromStr};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum Hand {
    FiveKind(u32),
    FourKind(u32),
    FullHouse(u32),
    ThreeKind(u32),
    TwoPair(u32),
    OnePair(u32),
    HighCard(u32),
}

impl Hand {
    pub fn order(&self) -> u32 {
        match self {
            Hand::FiveKind(_) => 7,
            Hand::FourKind(_) => 6,
            Hand::FullHouse(_) => 5,
            Hand::ThreeKind(_) => 4,
            Hand::TwoPair(_) => 3,
            Hand::OnePair(_) => 2,
            Hand::HighCard(_) => 1,
        }
    }

    pub fn value(&self) -> u32 {
        match *self {
            Hand::FiveKind(value) => value,
            Hand::FourKind(value) => value,
            Hand::FullHouse(value) => value,
            Hand::ThreeKind(value) => value,
            Hand::TwoPair(value) => value,
            Hand::OnePair(value) => value,
            Hand::HighCard(value) => value,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let res = self.order().cmp(&other.order());
        match res {
            Ordering::Equal => self.value().cmp(&other.value()),
            _ => res,
        }
    }
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 5 {
            return Err(anyhow!("Hand too long"));
        }

        let digits = s
            .chars()
            .map(|c| match c {
                character if character.is_digit(10) => character
                    .to_digit(10)
                    .map(|d| d - 2)
                    .ok_or(anyhow!("Invalid card label")),
                'T' => Ok(8),
                'J' => Ok(9),
                'Q' => Ok(10),
                'K' => Ok(11),
                'A' => Ok(12),
                _ => Err(anyhow!("Invalid card label")),
            })
            .collect::<Result<Vec<u32>>>()?;

        let numeric_representation = digits.iter().fold(0, |acc, digit| acc * 13 + digit);

        let mut freq = [0; 13];
        digits.iter().for_each(|&digit| freq[digit as usize] += 1);

        let freq_counts = (0..=5)
            .map(|digit_freq| freq.into_iter().filter(|&e| e == digit_freq).count())
            .collect_vec();

        if freq_counts[5] == 1 {
            Ok(Hand::FiveKind(numeric_representation))
        } else if freq_counts[4] == 1 {
            Ok(Hand::FourKind(numeric_representation))
        } else if freq_counts[3] == 1 {
            if freq_counts[2] == 1 {
                Ok(Hand::FullHouse(numeric_representation))
            } else {
                Ok(Hand::ThreeKind(numeric_representation))
            }
        } else if freq_counts[2] == 2 {
            Ok(Hand::TwoPair(numeric_representation))
        } else if freq_counts[2] == 1 {
            Ok(Hand::OnePair(numeric_representation))
        } else {
            Ok(Hand::HighCard(numeric_representation))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("23456", Hand::HighCard(2578))]
    #[case("KK678", Hand::OnePair(339085))]
    #[case("KK677", Hand::TwoPair(339084))]
    #[case("55522", Hand::FullHouse(92781))]
    #[case("55523", Hand::ThreeKind(92782))]
    #[case("AA8AA", Hand::FourKind(370278))]
    #[case("AAAAA", Hand::FiveKind(371292))]
    fn test_hand_parse(#[case] h: &str, #[case] expected: Hand) {
        assert_eq!(h.parse::<Hand>().unwrap(), expected);
    }

    #[rstest]
    #[case("KK677", "KTJJT", Ordering::Greater)]
    #[case("T55J5", "QQQJA", Ordering::Less)]
    #[case("KK677", "QQQJA", Ordering::Less)]
    #[case("QQQJA", "KTJJT", Ordering::Greater)]
    #[case("22222", "KQJT9", Ordering::Greater)]
    fn test_hand_cmp(#[case] h1: Hand, #[case] h2: Hand, #[case] expected: Ordering) {
        assert_eq!(h1.cmp(&h2), expected);
    }
}
