use aoc_runner_derive::*;

enum Command {
    Forward(u64),
    Down(u64),
    Up(u64),
}

use Command::*;

#[aoc_generator(day2)]
fn parse_input(text: &str) -> anyhow::Result<Vec<Command>> {
    text.lines()
        .map(|line| {
            let fields = line.split_whitespace().collect::<Vec<&str>>();
            anyhow::ensure!(fields.len() == 2, "each line should have two fields on it");
            let arg = fields[1].parse::<u64>()?;
            Ok(match fields[0] {
                "forward" => Forward(arg),
                "down" => Down(arg),
                "up" => Up(arg),
                other => anyhow::bail!("unrecognized command {:?}", other),
            })
        })
        .collect()
}

#[aoc(day2, part1)]
fn part_1(commands: &[Command]) -> u64 {
    let (x, d) = commands.iter().fold((0, 0), |(x, d), cmd| match cmd {
        Forward(n) => (x + n, d),
        Down(n) => (x, d + n),
        Up(n) => (x, d - n),
    });
    x * d
}

#[aoc(day2, part2)]
fn part_2(commands: &[Command]) -> u64 {
    let (x, d, _a) = commands.iter().fold((0, 0, 0), |(x, d, a), cmd| match cmd {
        Forward(n) => (x + n, d + a * n, a),
        Down(n) => (x, d, a + n),
        Up(n) => (x, d, a - n),
    });
    x * d
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
forward 5
down 5
forward 8
up 3
down 8
forward 2
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 150);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 900);
    }
}
