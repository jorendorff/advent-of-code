use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<(usize, u32, u32)>;

#[aoc_generator(day18, part1, jorendorff)]
#[aoc_generator(day18, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines(
        d:char_of("RDLU") " " n:u32 " (#" c:u32_hex ")" => (d, n, c)
    ));
    Ok(p.parse(text)?)
}

fn solve(instructions: impl IntoIterator<Item=(usize, u32)>) -> i128 {
    // Imagine the digger draws a line in chalk as it goes, right in the center of the trench.
    // `area` is the (directed) area of the region bounded by this line.
    let mut area = 0i128;

    // `border` is the area of the region inside the trench, but outside the chalk line. Note:
    // Since this is half a meter wide, we should count half; instead we count the full amount when
    // going "right" and "down", and count nothing when going "left" or "up". It balances out since
    // the path is a loop.
    //
    // The reason we start with a value of 1, and not 0, is wild: we undercount the border region
    // by a quarter square at each exterior corner, and overcount by a quarter square at each
    // interior corner. A loop must have 4 more exterior corners than interior ones.
    let mut border = 1;

    let mut x = 0i128;

    for (dir, n) in instructions {
        let n = n as i128;
        match dir {
            0 => {
                x += n;
                border += n;
            }
            1 => {
                border += n;
                area += (i64::MAX as i128 - x) * n;
            }
            2 => {
                x -= n;
            }
            3 => {
                area -= (i64::MAX as i128 - x) * n;
            }
            _ => panic!(),
        }
    }

    area.abs() + border
}

#[aoc(day18, part1, jorendorff)]
fn part_1(input: &Input) -> i128 {
    // #276 on the global leaderboard, but via a completely other method, see the git history
    solve(input.iter().map(|&(d, n, _color)| (d, n)))
}

#[aoc(day18, part2, jorendorff)]
fn part_2(input: &Input) -> i128 {
    // #220 on the global leaderboard
    solve(input.iter().map(|&(_d, _n, c)| (c as usize & 15, c >> 4)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 62);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 952408144115);
    }
}
