use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::{
    env,
    fmt::{Debug, Display},
    fs,
    io::Read,
    str::FromStr,
};

fn main() -> Result<()> {
    let input_file = env::args().nth(1).unwrap_or("./day-14.in".to_owned());
    let mut input = String::new();
    fs::File::open(&input_file)?.read_to_string(&mut input)?;
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));

    Ok(())
}

fn part_one(input: &str) -> usize {
    let mut platform: Platform = input.parse().unwrap();
    platform.move_rocks(Direction::North);
    platform.load()
}

fn part_two(input: &str) -> usize {
    let mut platform: Platform = input.parse().unwrap();
    platform.cycle(1_000_000_000);
    platform.load()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    RoundedRock,
    CubeRock,
    Space,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Platform {
    platform: Vec<Vec<Tile>>,
}

impl FromStr for Platform {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let platform = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|t| match t {
                        'O' => Ok(Tile::RoundedRock),
                        '#' => Ok(Tile::CubeRock),
                        '.' => Ok(Tile::Space),
                        _ => Err(anyhow!("Invalid character in platform representation")),
                    })
                    .collect()
            })
            .collect::<Result<Vec<Vec<Tile>>>>()?;

        Ok(Self { platform })
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.platform
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|tile| match tile {
                            Tile::CubeRock => '#',
                            Tile::RoundedRock => 'O',
                            Tile::Space => '.',
                        })
                        .join("")
                })
                .join("\n")
        )
    }
}

impl Debug for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n{}",
            self.platform
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|tile| match tile {
                            Tile::CubeRock => '#',
                            Tile::RoundedRock => 'O',
                            Tile::Space => '.',
                        })
                        .join("")
                })
                .join("\n")
        )
    }
}

impl Platform {
    pub fn move_rocks(&mut self, direction: Direction) {
        match direction {
            Direction::North => {
                for col in 0..self.platform[0].len() {
                    for row in 0..self.platform.len() {
                        if self.platform[row][col] == Tile::RoundedRock {
                            let distance = (0..row)
                                .rev()
                                .take_while(|&i| self.platform[i][col] == Tile::Space)
                                .count();
                            self.platform[row][col] = Tile::Space;
                            self.platform[row - distance][col] = Tile::RoundedRock;
                        }
                    }
                }
            }
            Direction::South => {
                for col in 0..self.platform[0].len() {
                    for row in (0..self.platform.len()).rev() {
                        if self.platform[row][col] == Tile::RoundedRock {
                            let distance = (row + 1..self.platform.len())
                                .take_while(|&i| self.platform[i][col] == Tile::Space)
                                .count();
                            self.platform[row][col] = Tile::Space;
                            self.platform[row + distance][col] = Tile::RoundedRock;
                        }
                    }
                }
            }
            Direction::West => {
                for row in 0..self.platform.len() {
                    for col in 0..self.platform[0].len() {
                        if self.platform[row][col] == Tile::RoundedRock {
                            let distance = (0..col)
                                .rev()
                                .take_while(|&j| self.platform[row][j] == Tile::Space)
                                .count();
                            self.platform[row][col] = Tile::Space;
                            self.platform[row][col - distance] = Tile::RoundedRock;
                        }
                    }
                }
            }
            Direction::East => {
                for row in 0..self.platform.len() {
                    for col in (0..self.platform[0].len()).rev() {
                        if self.platform[row][col] == Tile::RoundedRock {
                            let distance = (col + 1..self.platform[row].len())
                                .take_while(|&j| self.platform[row][j] == Tile::Space)
                                .count();
                            self.platform[row][col] = Tile::Space;
                            self.platform[row][col + distance] = Tile::RoundedRock;
                        }
                    }
                }
            }
        }
    }

    pub fn cycle(&mut self, times: usize) {
        let mut seen = vec![self.clone()];
        for _ in 0..times {
            self.move_rocks(Direction::North);
            self.move_rocks(Direction::West);
            self.move_rocks(Direction::South);
            self.move_rocks(Direction::East);

            if let Some(index) = seen.iter().position(|e| e == self) {
                let period = seen.len() - index;
                let final_index = index + (times - index) % period;
                *self = seen[final_index].clone();
                break;
            }

            seen.push(self.clone());
        }
    }

    pub fn load(&self) -> usize {
        self.platform
            .iter()
            .rev()
            .enumerate()
            .map(|(i, row)| {
                (i + 1)
                    * row
                        .iter()
                        .filter(|&&tile| tile == Tile::RoundedRock)
                        .count()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::*;
    use rstest::*;

    #[rstest]
    #[case(
    indoc! {"
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
    "}, 
    Direction::North,
    indoc! {"
        OOOO.#.O..
        OO..#....#
        OO..O##..O
        O..#.OO...
        ........#.
        ..#....#.#
        ..O..#.O.O
        ..O.......
        #....###..
        #....#....
    "})]
    #[case(
    indoc! {"
        #OO..#....
        #....###..
        .......O..
        ..O..#O..O
        O.#..O.#.#
        .O.....O#.
        OO.#O....O
        .....##...
        O.OO#....#
        O....#....
    "}, 
    Direction::South,
    indoc! {"
        #....#....
        #....###..
        ..O.......
        ..O..#.O.O
        ..#....#.#
        ........#.
        O..#.OO...
        OO..O##..O
        OO..#....#
        OOOO.#.O..
    "})]
    #[case(
    indoc! {"
        OO#...O
        ..O.O#O
        O.O.O.#
    "}, 
    Direction::West,
    indoc! {"
        OO#O...
        OO...#O
        OOO...#
    "})]
    #[case(
    indoc! {"
        #OO.#.O..O
        OOOOO.....
        O#O#O#O#O.
    "}, 
    Direction::East,
    indoc! {"
        #.OO#...OO
        .....OOOOO
        O#O#O#O#.O
    "})]
    fn test_move_rocks(
        #[case] platform: &str,
        #[case] direction: Direction,
        #[case] expected: &str,
    ) {
        let mut platform: Platform = platform.parse().unwrap();
        platform.move_rocks(direction);
        assert_eq!(platform, expected.parse().unwrap());
    }

    #[rstest]
    #[case(
    indoc! {"
        OOOO.#.O..
        OO..#....#
        OO..O##..O
        O..#.OO...
        ........#.
        ..#....#.#
        ..O..#.O.O
        ..O.......
        #....###..
        #....#....
    "},
    1,
    indoc! {"
        .....#....
        ....#...O#
        ...OO##...
        .OO#......
        .....OOO#.
        .O#...O#.#
        ....O#....
        ......OOOO
        #...O###..
        #..OO#....
    "})]
    #[case(
    indoc! {"
        OOOO.#.O..
        OO..#....#
        OO..O##..O
        O..#.OO...
        ........#.
        ..#....#.#
        ..O..#.O.O
        ..O.......
        #....###..
        #....#....
    "},
    2,
    indoc! {"
        .....#....
        ....#...O#
        .....##...
        ..O#......
        .....OOO#.
        .O#...O#.#
        ....O#...O
        .......OOO
        #..OO###..
        #.OOO#...O
    "})]
    #[case(
    indoc! {"
        OOOO.#.O..
        OO..#....#
        OO..O##..O
        O..#.OO...
        ........#.
        ..#....#.#
        ..O..#.O.O
        ..O.......
        #....###..
        #....#....
    "},
    3,
    indoc! {"
        .....#....
        ....#...O#
        .....##...
        ..O#......
        .....OOO#.
        .O#...O#.#
        ....O#...O
        .......OOO
        #...O###.O
        #.OOO#...O
    "})]
    fn test_cycle(#[case] platform: &str, #[case] times: usize, #[case] expected: &str) {
        let mut platform: Platform = platform.parse().unwrap();
        platform.cycle(times);
        assert_eq!(platform, expected.parse().unwrap());
    }

    #[rstest]
    #[case(
        indoc! {"
        OOOO.#.O..
        OO..#....#
        OO..O##..O
        O..#.OO...
        ........#.
        ..#....#.#
        ..O..#.O.O
        ..O.......
        #....###..
        #....#....
    "}, 136)]
    fn test_calculate_load(#[case] platform: &str, #[case] expected: usize) {
        let platform: Platform = platform.parse().unwrap();
        assert_eq!(platform.load(), expected);
    }

    #[test]
    fn test_part_one() {
        let input = indoc! {"
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
        "};

        assert_eq!(136, part_one(input));
    }
}
