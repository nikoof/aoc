use std::{env, fs, io::Read};

const DIGITS: [(&str, char); 9] = [
    ("one", '1'),
    ("two", '2'),
    ("three", '3'),
    ("four", '4'),
    ("five", '5'),
    ("six", '6'),
    ("seven", '7'),
    ("eight", '8'),
    ("nine", '9'),
];

fn main() -> anyhow::Result<()> {
    let input_file = env::args().nth(1).unwrap_or("./day-01.in".to_owned());
    let mut input: String = String::new();
    fs::File::open(input_file)?.read_to_string(&mut input)?;
    println!("Part 1: {}", part_one(&input)?);
    println!("Part 2: {}", part_two(&input)?);
    Ok(())
}

fn parse_line(line: &str, english_numerals: bool) -> Option<u32> {
    let mut index = 0;
    let line_iter = std::iter::from_fn(move || {
        let reduced_line = &line[index..];

        if english_numerals {
            for (s, d) in DIGITS {
                if reduced_line.starts_with(s) {
                    index += 1;
                    return Some(d);
                }
            }
        }

        index += 1;
        reduced_line.chars().next()
    });

    let mut it = line_iter.filter_map(|c| c.to_digit(10));
    let first = it.next().unwrap();
    let last = match it.last() {
        Some(d) => d,
        None => first,
    };

    Some(10 * first + last)
}

fn part_one(input: &str) -> anyhow::Result<u32> {
    Ok(input
        .lines()
        .filter_map(|line| parse_line(line, false))
        .sum())
}

fn part_two(input: &str) -> anyhow::Result<u32> {
    Ok(input
        .lines()
        .filter_map(|line| parse_line(line, true))
        .sum())
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("1abc2", 12)]
    #[case("pqr3stu8vwx", 38)]
    #[case("a1b2c3d4e5f", 15)]
    #[case("treb7uchet", 77)]
    fn test_parse_line_no_numerals(#[case] line: &str, #[case] expected: u32) {
        assert_eq!(expected, parse_line(line, false).unwrap());
    }

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let input = r#"1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet"#;
        assert_eq!(142, part_one(input)?);
        Ok(())
    }

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    /* All overlapping English numerals should be included in the parsed output */
    #[case("twothreesixthreecvsskxq3threefoureight", 28)]
    fn test_parse_line(#[case] line: &str, #[case] expected: u32) {
        assert_eq!(expected, parse_line(line, true).unwrap());
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let input = r#"two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
            twothreesixthreecvsskxq3threefoureight"#;

        assert_eq!(309, part_two(input)?);
        Ok(())
    }
}
