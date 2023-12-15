use anyhow::{anyhow, Result};
use itertools::repeat_n;
use std::collections::HashMap;
use std::{env, fs, io::Read, str::FromStr};

/* Taken from https://github.com/andypymont/advent2023-rust/blob/main/src/bin/12.rs
* This is what I get for not learning DP... */

fn main() -> Result<()> {
    let input_file = env::args().nth(1).unwrap_or("./day-12.in".to_owned());
    let mut input = String::new();
    fs::File::open(&input_file)?.read_to_string(&mut input)?;
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));

    Ok(())
}

fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<Record>().unwrap())
        .map(|rr| rr.arrangements())
        .sum()
}
fn part_two(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<Record>().unwrap().unfold())
        .map(|rr| rr.arrangements())
        .sum()
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug, PartialEq)]
struct Record {
    springs: Vec<Spring>,
    group_lengths: Vec<usize>,
}

impl FromStr for Record {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springs, group_lengths) = s.split_once(" ").ok_or(anyhow!(
            "Expected space-separated springs and group lengths"
        ))?;

        let springs = springs
            .chars()
            .map(|ch| match ch {
                '.' => Ok(Spring::Operational),
                '#' => Ok(Spring::Damaged),
                '?' => Ok(Spring::Unknown),
                _ => Err(anyhow!("Unknown spring state: {}", ch)),
            })
            .collect::<Result<Vec<Spring>>>()?;

        let group_lengths = group_lengths
            .split(',')
            .map(|length| length.parse::<usize>().map_err(|e| anyhow!("{}", e)))
            .collect::<Result<Vec<usize>>>()?;

        Ok(Self {
            springs,
            group_lengths,
        })
    }
}

impl Record {
    pub fn unfold(self) -> Self {
        let springs: Vec<Spring> = repeat_n(vec![self.springs, vec![Spring::Unknown]], 5)
            .flatten()
            .flatten()
            .collect();

        let group_lengths = repeat_n(self.group_lengths, 5).flatten().collect();

        Self {
            springs: springs[..springs.len() - 1].to_vec(),
            group_lengths,
        }
    }

    pub fn arrangements(&self) -> usize {
        let mut memo = HashMap::new();
        self.partial_arrangements(&mut memo, 0, 0)
    }

    fn partial_arrangements(
        &self,
        cache: &mut HashMap<(usize, usize), usize>,
        spring_index: usize,
        group_index: usize,
    ) -> usize {
        if let Some(result) = cache.get(&(spring_index, group_index)) {
            return *result;
        }

        let consume_group = self.group_lengths.get(group_index).map_or(0, |length| {
            if spring_index + length > self.springs.len() {
                return 0;
            }

            if self.springs[spring_index..spring_index + length]
                .iter()
                .any(|&e| e == Spring::Operational)
            {
                return 0;
            }

            if let Some(Spring::Damaged) = self.springs.get(spring_index + length) {
                return 0;
            }

            self.partial_arrangements(cache, spring_index + length + 1, group_index + 1)
        });

        let consume_element = match self.springs.get(spring_index) {
            None => (group_index >= self.group_lengths.len()) as usize,
            Some(Spring::Damaged) => 0,
            Some(_) => self.partial_arrangements(cache, spring_index + 1, group_index),
        };

        let result = consume_group + consume_element;
        cache.insert((spring_index, group_index), result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 4)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 1)]
    #[case("????.######..#####. 1,6,5", 4)]
    #[case("?###???????? 3,2,1", 10)]
    fn test_record_arrangements(#[case] record: Record, #[case] expected: usize) {
        assert_eq!(record.arrangements(), expected);
    }

    #[rstest]
    #[case(".# 1", ".#?.#?.#?.#?.# 1,1,1,1,1")]
    #[case(".??..??...?##. 1,1,3", ".??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##. 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3")]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", "?#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#? 1,3,1,6,1,3,1,6,1,3,1,6,1,3,1,6,1,3,1,6")]
    #[case("????.#...#... 4,1,1", "????.#...#...?????.#...#...?????.#...#...?????.#...#...?????.#...#... 4,1,1,4,1,1,4,1,1,4,1,1,4,1,1")]
    #[case("????.######..#####. 1,6,5", "????.######..#####.?????.######..#####.?????.######..#####.?????.######..#####.?????.######..#####. 1,6,5,1,6,5,1,6,5,1,6,5,1,6,5")]
    #[case("?###???????? 3,2,1", "?###??????????###??????????###??????????###??????????###???????? 3,2,1,3,2,1,3,2,1,3,2,1,3,2,1")]
    fn test_record_unfold(#[case] record: Record, #[case] expected: Record) {
        assert_eq!(record.unfold(), expected);
    }
}
