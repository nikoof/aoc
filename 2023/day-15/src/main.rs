use anyhow::{anyhow, Result};
use std::{env, fs, io::Read, str::FromStr};

fn main() -> Result<()> {
    let input_file = env::args().nth(1).unwrap_or("./day-15.in".to_owned());
    let mut input = String::new();
    fs::File::open(&input_file)?.read_to_string(&mut input)?;
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));

    Ok(())
}

fn part_one(input: &str) -> usize {
    input.split(",").map(|s| hash(s.trim()) as usize).sum()
}

fn part_two(input: &str) -> usize {
    let mut boxes: [Vec<Lens>; 256] = std::array::from_fn(|_| Vec::new());

    input
        .trim()
        .split(",")
        .map(|step| step.parse::<Operation>().unwrap())
        .for_each(|op| match op {
            Operation::RemoveLens { box_number, lens } => {
                if let Some(index) = boxes[box_number].iter().position(|l| l.label == lens.label) {
                    boxes[box_number].remove(index);
                }
            }
            Operation::AddLens { box_number, lens } => {
                if let Some(index) = boxes[box_number].iter().position(|l| l.label == lens.label) {
                    boxes[box_number][index] = lens;
                } else {
                    boxes[box_number].push(lens);
                }
            }
        });

    boxes
        .iter()
        .enumerate()
        .map(|(box_number, b)| {
            b.iter()
                .enumerate()
                .map(|(lens_number, l)| (box_number + 1) * (lens_number + 1) * l.focal_length)
                .sum::<usize>()
        })
        .sum()
}

struct Lens {
    label: String,
    focal_length: usize,
}

enum Operation {
    RemoveLens { box_number: usize, lens: Lens },
    AddLens { box_number: usize, lens: Lens },
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((label, _)) = s.split_once("-") {
            Ok(Operation::RemoveLens {
                box_number: hash(label),
                lens: Lens {
                    label: label.to_string(),
                    focal_length: 0,
                },
            })
        } else if let Some((label, focal_length)) = s.split_once("=") {
            Ok(Operation::AddLens {
                box_number: hash(label),
                lens: Lens {
                    label: label.to_string(),
                    focal_length: focal_length.parse()?,
                },
            })
        } else {
            Err(anyhow!("Invalid operation"))
        }
    }
}

fn hash(input: &str) -> usize {
    input
        .chars()
        .fold(0, |acc, ch| ((acc + ch as usize) * 17) % 256)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("rn=1", 30)]
    #[case("cm-", 253)]
    #[case("qp=3", 97)]
    #[case("cm=2", 47)]
    #[case("qp-", 14)]
    #[case("pc=4", 180)]
    #[case("ot=9", 9)]
    #[case("ab=5", 197)]
    #[case("pc-", 48)]
    #[case("pc=6", 214)]
    #[case("ot=7", 231)]
    fn test_hash(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(hash(input), expected);
    }

    #[test]
    fn test_part_one() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part_one(input), 1320);
    }

    #[test]
    fn test_part_two() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part_two(input), 145);
    }
}
