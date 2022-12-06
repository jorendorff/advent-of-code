use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<char>;

#[aoc_generator(day6, part1, jorendorff)]
#[aoc_generator(day6, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(line(alpha+));
    aoc_parse(text, p)
}

fn solve(chars: &[char], n: usize) -> usize {
    let mut counts = [0usize; 128];
    let mut nonzero = 0;
    for (i, c) in chars.iter().copied().enumerate() {
        let c = c as usize;
        if c < 128 {
            if counts[c] == 0 {
                nonzero += 1;
                if nonzero == n {
                    return i + 1;
                }
            }
            counts[c] += 1;
        }

        if i + 1 >= n {
            let c = chars[i + 1 - n] as usize;
            if c < 128 {
                counts[c] -= 1;
                if counts[c] == 0 {
                    nonzero -= 1;
                }
            }
        }
    }
    panic!("no match found");
}

#[aoc(day6, part1, jorendorff)]
fn part_1(signal: &[char]) -> usize {
    solve(signal, 4)
}

#[aoc(day6, part2, jorendorff)]
fn part_2(signal: &[char]) -> usize {
    solve(signal, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p1(s: &str) -> usize {
        part_1(&parse_input(s).unwrap())
    }

    #[test]
    fn test_part_1() {
        assert_eq!(p1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(p1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(p1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(p1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    fn p2(s: &str) -> usize {
        part_2(&parse_input(s).unwrap())
    }

    #[test]
    fn test_part_2() {
        assert_eq!(p2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(p2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(p2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(p2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(p2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
