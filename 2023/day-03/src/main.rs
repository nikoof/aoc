use anyhow::Result;
use std::{collections::HashMap, env, fs, io::Read};

fn main() -> Result<()> {
    let input_file = env::args().nth(1).unwrap_or("./day-03.in".to_owned());
    let mut input = String::new();
    fs::File::open(&input_file)?.read_to_string(&mut input)?;
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));

    Ok(())
}

/* I am not proud of the following code at all. I apologise to
 * whomever may be trying to read it in the future. */

fn part_one(input: &str) -> u32 {
    let schematic = input
        .lines()
        .map(|line| line.trim())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut sum = 0;
    for (i, row) in schematic.iter().enumerate() {
        let mut j = 0;
        while j < row.len() {
            if row[j].is_digit(10) {
                let start = j;
                let end = j + row[j..].iter().take_while(|ch| ch.is_digit(10)).count();

                if start < end {
                    let is_part_num = schematic
                        [i.saturating_sub(1)..=(i + 1).min(schematic.len() - 1)]
                        .iter()
                        .any(|r| {
                            r[start.saturating_sub(1)..=end.min(row.len() - 1)]
                                .iter()
                                .any(|&ch| !ch.is_digit(10) && ch != '.')
                        });

                    if is_part_num {
                        sum += row[start..end]
                            .iter()
                            .collect::<String>()
                            .parse::<u32>()
                            .unwrap();
                    }
                    j = end;
                }
            }

            j += 1;
        }
    }

    sum
}

fn part_two(input: &str) -> u32 {
    let schematic = input
        .lines()
        .map(|line| line.trim())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    for (i, row) in schematic.iter().enumerate() {
        let mut j = 0;
        while j < row.len() {
            if row[j].is_digit(10) {
                let start = j;
                let end = j + row[j..].iter().take_while(|ch| ch.is_digit(10)).count();

                if start < end {
                    let gear_pos = schematic
                        [i.saturating_sub(1)..=(i + 1).min(schematic.len() - 1)]
                        .iter()
                        .enumerate()
                        .filter_map(|(k, r)| {
                            r[start.saturating_sub(1)..=end.min(row.len() - 1)]
                                .iter()
                                .position(|&ch| ch == '*')
                                .map(|p| (i.saturating_sub(1) + k, start.saturating_sub(1) + p))
                        })
                        .collect::<Vec<(usize, usize)>>();

                    if !gear_pos.is_empty() {
                        let num = row[start..end]
                            .iter()
                            .collect::<String>()
                            .parse::<u32>()
                            .unwrap();

                        if let Some(mut old) = gears.insert(gear_pos[0], vec![num]) {
                            old.push(num);
                            gears.insert(gear_pos[0], old);
                        }
                    }

                    j = end;
                }
            }

            j += 1;
        }
    }

    let mut sum = 0;
    for (_, v) in gears {
        if v.len() == 2 {
            sum += v[0] * v[1];
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598.."#;

        assert_eq!(4361, part_one(input));
    }

    #[test]
    fn test_part_two() {
        let input = r#"467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598.."#;

        assert_eq!(467835, part_two(input));
    }
}
