use anyhow::Result;
use indoc::indoc;
use itertools::*;
use std::collections::BTreeMap;
use std::{env, fs, io::Read};

fn main() -> Result<()> {
    let input_file = env::args().nth(1).unwrap_or("./day-08.in".to_owned());
    let mut input = String::new();
    fs::File::open(&input_file)?.read_to_string(&mut input)?;
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));

    Ok(())
}

fn part_one(input: &str) -> usize {
    let (directions, nodes) = input.split_once("\n\n").unwrap();

    let nodes = nodes
        .lines()
        .map(|line| {
            let (src, dest) = line.split_once(" = ").unwrap();
            let dest = dest
                .strip_prefix("(")
                .map(|s| s.strip_suffix(")"))
                .flatten()
                .unwrap()
                .split_once(", ")
                .unwrap();

            (src, dest)
        })
        .collect::<BTreeMap<&str, (&str, &str)>>();

    directions
        .chars()
        .cycle()
        .fold_while((0, "AAA"), |(count, current_node), direction| {
            if current_node == "ZZZ" {
                FoldWhile::Done((count, current_node))
            } else {
                let new_node = match direction {
                    'L' => nodes[current_node].0,
                    'R' => nodes[current_node].1,
                    _ => panic!(),
                };
                FoldWhile::Continue((count + 1, new_node))
            }
        })
        .into_inner()
        .0
}

fn part_two(input: &str) -> usize {
    let (directions, nodes) = input.split_once("\n\n").unwrap();

    let nodes = nodes
        .lines()
        .map(|line| {
            let (src, dest) = line.split_once(" = ").unwrap();
            let dest = dest
                .strip_prefix("(")
                .map(|s| s.strip_suffix(")"))
                .flatten()
                .unwrap()
                .split_once(", ")
                .unwrap();

            (src, dest)
        })
        .collect::<BTreeMap<&str, (&str, &str)>>();

    nodes
        .keys()
        .filter(|node| node.ends_with("A"))
        .map(|node| node.to_owned())
        .map(|node| {
            directions
                .chars()
                .cycle()
                .fold_while((0, node), |(count, current_node), direction| {
                    if current_node.ends_with("Z") {
                        FoldWhile::Done((count, current_node))
                    } else {
                        let new_node = match direction {
                            'L' => nodes[current_node].0,
                            'R' => nodes[current_node].1,
                            _ => panic!(),
                        };
                        FoldWhile::Continue((count + 1, &new_node))
                    }
                })
                .into_inner()
                .0
        })
        .fold(1, |acc, period| num::integer::lcm(acc, period))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use rstest::rstest;

    #[rstest]
    #[case(indoc!{"
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
    "}, 6)]
    #[case(indoc!{"
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)
    "}, 2)]
    fn test_part_one(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(part_one(input), expected);
    }

    #[rstest]
    #[case(indoc!{"
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
    "}, 6)]
    fn test_part_two(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(part_two(input), expected);
    }
}
