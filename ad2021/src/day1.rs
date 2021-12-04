use aoc_runner_derive::*;

/// Parse the puzzle input, which is just a series of numbers, one per line.
#[aoc_generator(day1)]
fn parse_input(text: &str) -> anyhow::Result<Vec<u64>> {
    let data = text
        .lines()
        .map(|s| s.parse::<u64>())
        .collect::<Result<Vec<u64>, _>>()?;
    Ok(data)
}

#[aoc(day1, part1)]
fn count_increases(depths: &[u64]) -> usize {
    depths.windows(2).filter(|pair| pair[0] < pair[1]).count()
}

#[aoc(day1, part2)]
fn count_triplet_increases(depths: &[u64]) -> usize {
    count_increases(
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
        assert_eq!(count_increases(&parse_input(EXAMPLE).unwrap()), 7);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(count_triplet_increases(&parse_input(EXAMPLE).unwrap()), 5);
    }
}
