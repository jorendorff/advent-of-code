use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<Insn>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Insn {
    Noop,
    Addx(i64),
}
use Insn::*;

#[aoc_generator(day10, part1, jorendorff)]
#[aoc_generator(day10, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines({
        "noop" => Noop,
        "addx " x:i64 => Addx(x),
    }));
    aoc_parse(text, p)
}

fn is_interesting(cycle: i64) -> bool {
    [20, 60, 100, 140, 180, 220].iter().any(|&i| i == cycle)
}

#[aoc(day10, part1, jorendorff)]
fn part_1(input: &Input) -> i64 {
    let mut total = 0;
    let mut tick = |t, ss| {
        if is_interesting(t) {
            total += ss;
        }
    };

    let mut t = 1i64;
    let mut x = 1i64;
    for i in input {
        match i {
            Noop => {
                tick(t, t * x);
                t += 1;
            }
            Addx(d) => {
                tick(t, t * x);
                t += 1;
                tick(t, t * x);
                t += 1;
                x += *d;
            }
        }
    }

    total
}

#[aoc(day10, part2, jorendorff)]
fn part_2(input: &Input) -> String {
    let mut grid = String::new();
    let mut tick = |t, x| {
        let h = (t + 39) % 40 + 1;
        grid.push(if x <= h && h <= x + 2 { '#' } else { '.' });
        if h == 40 {
            grid.push('\n');
        }
    };

    let mut t = 1i64;
    let mut x = 1i64;
    for i in input {
        match i {
            Noop => {
                tick(t, x);
                t += 1;
            }
            Addx(d) => {
                tick(t, x);
                t += 1;
                tick(t, x);
                t += 1;
                x += *d;
            }
        }
    }

    println!("{grid}");
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 13140);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(&parse_input(EXAMPLE).unwrap()),
            "\
            ##..##..##..##..##..##..##..##..##..##..\n\
            ###...###...###...###...###...###...###.\n\
            ####....####....####....####....####....\n\
            #####.....#####.....#####.....#####.....\n\
            ######......######......######......####\n\
            #######.......#######.......#######.....\n\
            "
        );
    }
}
