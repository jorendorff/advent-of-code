use aoc_runner_derive::*;

/// Parse the puzzle input, which is just a series of numbers, one per line.
#[aoc_generator(day1, part1, jorendorff)]
#[aoc_generator(day1, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Vec<u64>> {
    let data = text
        .lines()
        .map(|s| s.parse::<u64>())
        .collect::<Result<Vec<u64>, _>>()?;
    Ok(data)
}

#[aoc(day1, part1, jorendorff)]
fn part_1(depths: &[u64]) -> usize {
    depths.windows(2).filter(|pair| pair[0] < pair[1]).count()
}

#[aoc(day1, part2, jorendorff)]
fn part_2(depths: &[u64]) -> usize {
    part_1(
        &depths
            .windows(3)
            .map(|window| window.iter().sum::<u64>())
            .collect::<Vec<u64>>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
199
200
208
210
200
207
240
269
260
263
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 7);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 5);
    }
}
