use anyhow::Result;
use std::{env, fs, io::Read, iter::zip};

fn main() -> Result<()> {
    let input_file = env::args().nth(1).unwrap_or("./day-06.in".to_owned());
    let mut input = String::new();
    fs::File::open(&input_file)?.read_to_string(&mut input)?;
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));

    Ok(())
}

/* Let t = the time limit of a rance
 * and p = the distance record
 *
 *  Since the 'acceleration' of charging
 *  the boat is a = 1 mm/ms^2,
 *  the final velocity v is going
 *  to be equal to the charging time
 *  t_c.
 *
 *  Let f: N -> N, f(t_c) = (t - t_c) * v
 *                 f(v)   = (t - v  ) * v
 *                        = tv - v^2
 *                        = -v^2 + tv
 *  be the function that gives the distance
 *  traveled for a charging time of t_c.
 *
 *  The winning condition is
 *      f(v)          > p
 *      -v^2 + tv     > p
 *      -v^2 + tv - p > 0
 *  which implies that
 *      v \in [0.5 * (t - sqrt(t - 4p)), 0.5 * (t + sqrt(t - 4p))] \intersect N
 *  or, a more useful form
 *      ⌈0.5 * (t - sqrt(t - 4p))⌉ < v < ⌊0.5 * (t + sqrt(t - 4p))⌋
 */

fn part_one(input: &str) -> u64 {
    let (times, distances) = input.split_once("\n").unwrap();
    zip(
        times.split_whitespace().skip(1),
        distances.split_whitespace().skip(1),
    )
    .map(|(t, d)| (t.parse::<f64>().unwrap(), d.parse::<f64>().unwrap()))
    .map(|(t, d)| {
        let lower_bound = (0.5 * (t - (t * t - 4.0 * d).sqrt())).floor() as u64;
        let upper_bound = (0.5 * (t + (t * t - 4.0 * d).sqrt())).ceil() as u64;

        upper_bound - lower_bound - 1
    })
    .product()
}

fn part_two(input: &str) -> usize {
    let (time, distance) = input.split_once("\n").unwrap();

    let time = time
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let distance = distance
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    /* Quadratic formula gives wrong result for such big numbers, can't be bothered to think of a
     * way to solve that. */

    let lower_bound = (1..time).find(|v| v * (time - v) > distance).unwrap();
    let upper_bound = (lower_bound..time)
        .find(|v| v * (time - v) < distance)
        .unwrap();

    upper_bound - lower_bound
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "Time:      7  15   30\nDistance:  9  40  200";

        assert_eq!(288, part_one(&input));
    }

    #[test]
    fn test_part_two() {
        let input = "Time:      7  15   30\nDistance:  9  40  200";

        assert_eq!(71503, part_two(&input));
    }
}
