// Part 1 rank 891, part 2 rank 496.

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<(i64, i64, i64, i64)>;

#[aoc_generator(day14, part1, jorendorff)]
#[aoc_generator(day14, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines("p=" i64 "," i64 " v=" i64 "," i64));
    Ok(p.parse(text)?)
}

fn quadrant_of(nx: i64, ny: i64, x: i64, y: i64) -> usize {
    if x < nx / 2 {
        if y < ny / 2 {
            0
        } else if y > ny / 2 {
            1
        } else {
            4
        }
    } else if x > nx / 2 {
        if y < ny / 2 {
            2
        } else if y > ny / 2 {
            3
        } else {
            4
        }
    } else {
        4
    }
}

fn part1(input: &Input, nx: i64, ny: i64, dt: i64) -> u64 {
    let mut counts = [0; 5];

    for (px, py, vx, vy) in input.iter().copied() {
        let x = (px + vx * dt).rem_euclid(nx);
        let y = (py + vy * dt).rem_euclid(ny);
        counts[quadrant_of(nx, ny, x, y)] += 1;
    }

    counts[0] * counts[1] * counts[2] * counts[3]
}


#[aoc(day14, part1, jorendorff)]
fn part_1(input: &Input) -> u64 {
    part1(input, 101, 103, 100)
}

#[aoc(day14, part2, jorendorff)]
fn part_2(input: &Input) -> usize {
    let nx = 101;
    let ny = 103;

    let mut input = input.clone();
    for t in 0.. {
        let mut grid = vec![vec!['.'; nx]; ny];
        for &(px, py, _, _) in input.iter() {
            grid[py as usize][px as usize] = '#';
        }

        if grid.iter().any(|row| row.iter().collect::<String>().contains("###############################")) {
            for row in grid {
                println!("    {}", row.into_iter().collect::<String>());
            }
            return t;
        }

        for (px, py, vx, vy) in input.iter_mut() {
            *px = (*px + *vx).rem_euclid(nx as i64);
            *py = (*py + *vy).rem_euclid(ny as i64);
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";
        
    #[test]
    fn test_part_1() {
        assert_eq!(part1(&parse_input(EXAMPLE).unwrap(), 11, 7, 100), 12);
    }

    #[test]
    fn test_part_2() {
    }
}
