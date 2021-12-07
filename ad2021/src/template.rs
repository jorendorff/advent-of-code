use aoc_runner_derive::*;

#[aoc_generator(dayX)]
fn parse_input(text: &str) -> anyhow::Result<Vec<()>> {
    text.lines()
        .map(|line| {
            todo!();
        })
        .collect()
}

#[aoc(dayX, part1)]
fn part_1() -> u64 {
    todo!();
}

#[aoc(dayX, part2)]
fn part_2() -> u64 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), ());
    }

    //#[test]
    //fn test_part_2() {
    //    assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), ());
    //}
}
