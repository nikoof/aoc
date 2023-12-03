use anyhow::Result;
use std::{env, fs, io::Read};

fn main() -> Result<()> {
    let input_file = env::args().nth(1).unwrap_or("./day-02.in".to_owned());
    let mut input = String::new();
    fs::File::open(&input_file)?.read_to_string(&mut input)?;

    println!("Part 1: {}", part_one(&input, (12, 13, 14)));
    println!("Part 2: {}", part_two(&input));

    Ok(())
}

fn process_bunch(bunch: &str) -> (u32, u32, u32) {
    let (mut r, mut g, mut b) = (0, 0, 0);
    bunch.split(", ").for_each(|item| {
        let (count, color) = item.split_once(" ").unwrap();
        let count = count.parse::<u32>().unwrap();
        match color {
            "red" => r += count,
            "green" => g += count,
            "blue" => b += count,
            _ => (),
        }
    });
    (r, g, b)
}

fn min_cube_set(line: &str) -> (u32, u32, u32) {
    let (_, data) = line.split_once(": ").expect("split at :");
    data.split("; ")
        .map(process_bunch)
        .fold((0, 0, 0), |(acc_r, acc_g, acc_b), (r, g, b)| {
            (acc_r.max(r), acc_g.max(g), acc_b.max(b))
        })
}

fn part_one(input: &str, (red_cubes, green_cubes, blue_cubes): (u32, u32, u32)) -> u32 {
    input
        .lines()
        .map(min_cube_set)
        .enumerate()
        .filter(|&(_, (r, g, b))| r <= red_cubes && g <= green_cubes && b <= blue_cubes)
        .fold(0, |acc, (i, _)| acc + (i + 1) as u32)
}

fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(min_cube_set)
        .map(|(r, g, b)| r * g * b)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("3 blue, 4 red, 4 red", (8, 0, 3))]
    #[case("8 green, 6 blue, 20 red", (20, 8, 6))]
    #[case("1 green, 3 red, 6 blue, 3 green, 6 red, 3 green", (9, 7, 6))]
    fn test_process_bunch(#[case] bunch: &str, #[case] expected: (u32, u32, u32)) {
        assert_eq!(process_bunch(bunch), expected);
    }

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", (4, 2, 6))]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", (1, 3, 4))]
    #[case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", (20, 13, 6))]
    #[case("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", (14, 3, 15))]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", (6, 3, 2))]
    fn test_min_cube_set(#[case] line: &str, #[case] expected: (u32, u32, u32)) {
        assert_eq!(min_cube_set(line), expected);
    }

    #[test]
    fn test_part_one() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

        assert_eq!(8, part_one(input, (12, 13, 14)));
    }

    #[test]
    fn test_part_two() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

        assert_eq!(2286, part_two(input));
    }
}
