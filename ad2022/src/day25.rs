use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<Vec<usize>>;

#[aoc_generator(day25, part1, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines(char_of("=-012")+));
    Ok(p.parse(text)?)
}

fn encode(v: i64) -> String {
    let mut s = if v > 2 {
        encode((v + 2) / 5)
    } else {
        String::new()
    };
    s.push(['=', '-', '0', '1', '2'][((v + 2) as usize) % 5]);
    s
}

#[aoc(day25, part1, jorendorff)]
fn part_1(input: &Input) -> String {
    // Rank 298 on this star's leaderboard.
    let total = input.iter()
        .map(|digits| {
            let mut v = 0i64;
            for d in digits {
                v *= 5;
                v += *d as i64 - 2;
            }
            v
        })
        .sum::<i64>();
    encode(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
";

    #[test]
    fn test_part_1() {
        assert_eq!(&encode(2022), "1=11-2");
        assert_eq!(&encode(12345), "1-0---0");
        assert_eq!(&encode(314159265), "1121-1110-1=0");
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), "2=-1=0".to_string());
    }
}
