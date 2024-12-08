use std::collections::*;

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<Vec<char>>;
type Index = HashMap<char, Vec<(isize, isize)>>;

#[aoc_generator(day8, part1, jorendorff)]
#[aoc_generator(day8, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines({
        alnum,
        '.' => '.',
    }+));
    Ok(p.parse(text)?)
}

fn index(input: &Input) -> Index {
    let mut index = Index::new();
    for (r, row) in input.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch != '.' {
                index.entry(ch).or_default().push((r as isize, c as isize));
            }
        }
    }
    index
}

#[aoc(day8, part1, jorendorff)]
fn part_1(input: &Input) -> usize {
    let index = index(input);
    let nr = input.len() as isize;
    let nc = input[0].len() as isize;
    
    let mut antinodes = HashSet::new();
    for points in index.into_values() {
        for (i, pi) in points.iter().copied().enumerate() {
            for pj in points[..i].iter().copied() {
                let (r, c) = (2 * pj.0 - pi.0, 2 * pj.1 - pi.1);
                if 0 <= r && r < nr && 0 <= c && c < nc {
                    antinodes.insert((r, c));
                }
                let (r, c) = (2 * pi.0 - pj.0, 2 * pi.1 - pj.1);
                if 0 <= r && r < nr && 0 <= c && c < nc {
                    antinodes.insert((r, c));
                }
            }
        }
    }
    
    antinodes.len()
}

#[aoc(day8, part2, jorendorff)]
fn part_2(input: &Input) -> usize {
    let index = index(input);
    let nr = input.len() as isize;
    let nc = input[0].len() as isize;

    let mut count = 0;
    for r in 0..nr {
        for c in 0..nc {
            'myloop: for points in index.values() {
                for (i, (ri, ci)) in points.iter().copied().enumerate() {
                    for (rj, cj) in points[..i].iter().copied() {
                        if (r - ri) * (cj - ci) == (rj - ri) * (c - ci) {
                            count += 1;
                            break 'myloop;
                        }
                    }
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    const EXAMPLE_T: &str = "\
T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 14);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE_T).unwrap()), 9);
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 34);
    }
}
