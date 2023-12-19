use anyhow::Result;
use glam::I64Vec2;
use std::{env, fs, io::Read};

fn main() -> Result<()> {
    let input_file = env::args().nth(1).unwrap_or("./day-16.in".to_owned());
    let mut input = String::new();
    fs::File::open(&input_file)?.read_to_string(&mut input)?;
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));

    Ok(())
}

fn part_one(input: &str) -> i64 {
    let mut current_vertex = I64Vec2::ZERO;
    let (perimeter, area) = input
        .lines()
        .map(|line| {
            let (direction, rest) = line.split_once(" ").unwrap();
            let (step_size, _color) = rest.split_once(" ").unwrap();

            let direction = match direction {
                "R" => Some((1, 0)),
                "L" => Some((-1, 0)),
                "U" => Some((0, 1)),
                "D" => Some((0, -1)),
                _ => None,
            }
            .unwrap();

            let step = step_size.parse::<i64>().unwrap();

            let next_vertex = current_vertex
                + I64Vec2 {
                    x: step * direction.0,
                    y: step * direction.1,
                };

            let retval = (
                step,
                (current_vertex.y + next_vertex.y) * (current_vertex.x - next_vertex.x),
            );

            current_vertex = next_vertex;

            retval
        })
        .fold((0, 0), |(step, trapezoid), (perimeter, area)| {
            (perimeter + step, area + trapezoid)
        });

    (area.abs() + perimeter) / 2 + 1
}

fn part_two(input: &str) -> i64 {
    let mut current_vertex = I64Vec2::ZERO;
    let (perimeter, area) = input
        .lines()
        .map(|line| {
            let (_, rest) = line.split_once(" ").unwrap();
            let (_, color) = rest.split_once(" ").unwrap();

            let color = color.trim_matches(|c| match c {
                '(' | ')' | '#' => true,
                _ => false,
            });

            let color = i64::from_str_radix(color, 16).unwrap();

            let direction = match color % 16 {
                0 => Some((1, 0)),
                1 => Some((0, -1)),
                2 => Some((-1, 0)),
                3 => Some((0, 1)),
                _ => None,
            }
            .unwrap();

            let step = color / 16;

            let next_vertex = current_vertex
                + I64Vec2 {
                    x: step * direction.0,
                    y: step * direction.1,
                };

            let retval = (
                step,
                (current_vertex.y + next_vertex.y) * (current_vertex.x - next_vertex.x),
            );

            current_vertex = next_vertex;

            retval
        })
        .fold((0, 0), |(step, trapezoid), (perimeter, area)| {
            (perimeter + step, area + trapezoid)
        });

    (area.abs() + perimeter) / 2 + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::*;

    #[test]
    fn test_part_one() {
        let input = indoc! {"
            R 6 (#70c710)
            D 5 (#0dc571)
            L 2 (#5713f0)
            D 2 (#d2c081)
            R 2 (#59c680)
            D 2 (#411b91)
            L 5 (#8ceee2)
            U 2 (#caa173)
            L 1 (#1b58a2)
            U 2 (#caa171)
            R 2 (#7807d2)
            U 3 (#a77fa3)
            L 2 (#015232)
            U 2 (#7a21e3)
        "};

        assert_eq!(part_one(&input), 62);
    }

    #[test]
    fn test_part_two() {
        let input = indoc! {"
            R 6 (#70c710)
            D 5 (#0dc571)
            L 2 (#5713f0)
            D 2 (#d2c081)
            R 2 (#59c680)
            D 2 (#411b91)
            L 5 (#8ceee2)
            U 2 (#caa173)
            L 1 (#1b58a2)
            U 2 (#caa171)
            R 2 (#7807d2)
            U 3 (#a77fa3)
            L 2 (#015232)
            U 2 (#7a21e3)
        "};

        assert_eq!(part_two(&input), 952408144115);
    }
}
