use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

// locks, keys
type Input = (Vec<Vec<usize>>, Vec<Vec<usize>>);

#[aoc_generator(day25, part1, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(sections(lines(char_of(".#")+)));
    let bitmaps = p.parse(text)?;

    let mut locks = vec![];
    let mut keys = vec![];

    for map in bitmaps {
        let w = map[0].len();
        if map[0].iter().all(|&b| b == 0) {
            keys.push((0..w).map(|c| map.iter().map(|row| row[c]).sum::<usize>() - 1).collect());
        } else {
            locks.push((0..w).map(|c| map.iter().map(|row| row[c]).sum::<usize>() - 1).collect());
        }
    }

    Ok((locks, keys))
}

#[aoc(day25, part1, jorendorff)]
fn part_1(input: &Input) -> usize {
    let (locks, keys) = input;
    locks.iter()
        .map(|lock| {
            keys.iter()
                .filter(|&key| {
                    key.iter().copied().zip(lock.iter().copied())
                        .all(|(k, l)| k + l <= 5)
                })
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 3);
    }
}
