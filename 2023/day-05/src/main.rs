use anyhow::Result;
use std::{collections::HashMap, env, fs, io::Read, ops::Range};

fn main() -> Result<()> {
    let input_file = env::args().nth(1).unwrap_or("./day-05.in".to_owned());
    let mut input = String::new();
    fs::File::open(&input_file)?.read_to_string(&mut input)?;

    let input = "
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48";

    println!("Part 1: {}", part_one(&input));
    // println!("Part 2: {}", part_two(&input));

    Ok(())
}

fn parse_map(input: &str) -> HashMap<Range<usize>, Range<usize>> {
    input
        .lines()
        .skip(1)
        .map(|line| line.trim())
        .map(|line| {
            let [dest_start, src_start, len, ..] = line
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .take(3)
                .collect::<Vec<usize>>()[0..3]
            else {
                panic!("");
            };

            (src_start..src_start + len, dest_start..dest_start + len)
        })
        .collect()
}

fn part_one(input: &str) -> u32 {
    let lines = input
        .lines()
        .skip(3)
        .take_while(|line| line.trim() != "")
        .collect::<String>();

    println!("{}", lines);

    let seed_to_soil = parse_map(&lines);

    println!("{:?}", seed_to_soil);

    0
}

fn part_two(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("seed-to-soil map:\n50 98 2\n52 50 48",
           vec![(98..100, 50..52), (50..98, 52..100)])]
    #[case("soil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15",
           vec![(15..52, 0..37), (52..54, 37..39), (0..15, 39..54)])]
    #[case("fertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4\n",
           vec![(53..61, 49..57), (11..53, 0..42), (0..7, 42..49), (7..11, 57..61)])]
    fn test_parse_map(#[case] input: &str, #[case] expected: Vec<(Range<usize>, Range<usize>)>) {
        let expected = expected
            .into_iter()
            .collect::<HashMap<Range<usize>, Range<usize>>>();
        assert_eq!(expected, parse_map(input));
    }

    #[test]
    fn test_part_one() {
        let input = "
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
            ";
        assert_eq!(0, part_one(input));
    }
}
