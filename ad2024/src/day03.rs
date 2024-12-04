use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<(u32, u32)>;

#[aoc_generator(day03, part1, jorendorff)]
fn parse_input_1(text: &str) -> anyhow::Result<Input> {
    let p = parser!({
        "mul("
            x:{string(digit), string(digit digit), string(digit digit digit)} ','
            y:{string(digit), string(digit digit), string(digit digit digit)} ')' =>
            Some((x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap())),
        any_char => None
    }*);
    Ok(p.parse(text)?.into_iter().flatten().collect())
}

#[derive(Clone, Copy)]
enum Insn {
    Mul(u32, u32),
    Do,
    Dont,
}

#[aoc_generator(day03, part2, jorendorff)]
fn parse_input_2(text: &str) -> anyhow::Result<Vec<Insn>> {
    let p = parser!({
        "mul("
            x:{string(digit), string(digit digit), string(digit digit digit)} ','
            y:{string(digit), string(digit digit), string(digit digit digit)} ')' =>
            Some(Insn::Mul(x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap())),
        "do()" => Some(Insn::Do),
        "don't()"=> Some(Insn::Dont),
        any_char => None
    }*);
    Ok(p.parse(text)?.into_iter().flatten().collect())
}

#[aoc(day03, part1, jorendorff)]
fn part_1(input: &Input) -> u32 {
    input.iter().map(|&(a, b)| a * b).sum()
}

#[aoc(day03, part2, jorendorff)]
fn part_2(input: &[Insn]) -> u32 {
    let mut enabled = true;
    let mut total = 0;
    for &insn in input {
        match insn {
            Insn::Mul(a, b) => if enabled { total += a * b; }
            Insn::Do => enabled = true,
            Insn::Dont => enabled = false,
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input_1(EXAMPLE_1).unwrap()), 161);
    }

    const EXAMPLE_2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input_2(EXAMPLE_2).unwrap()), 48);
    }
}
