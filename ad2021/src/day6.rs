use aoc_runner_derive::*;

const TIMER_LIMIT: usize = 9;

#[aoc_generator(day6)]
fn parse_input(text: &str) -> anyhow::Result<Vec<u64>> {
    let mut results = vec![0; TIMER_LIMIT];
    for line in text.trim().split(',') {
        let n = line.parse::<usize>()?;
        anyhow::ensure!(n < TIMER_LIMIT, "simulation can't handle such fish: {}", n);
        results[n] += 1;
    }
    Ok(results)
}

fn solve(ndays: usize, fish: &[u64]) -> u64 {
    let mut fish = fish.to_vec();
    for i in 0..ndays {
        let k = fish[i % TIMER_LIMIT];
        fish[(i + 6 + 1) % TIMER_LIMIT] += k;
    }
    fish.into_iter().sum()
}

#[aoc(day6, part1)]
fn part_1(fish: &[u64]) -> u64 {
    solve(80, fish)
}

#[aoc(day6, part2)]
fn part_2(fish: &[u64]) -> u64 {
    solve(256, fish)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
3,4,3,1,2
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 5934);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 26984457539);
    }
}
