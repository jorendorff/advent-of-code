use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<char>;

#[aoc_generator(day6, part1, jorendorff)]
#[aoc_generator(day6, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(line(alpha+));
    Ok(p.parse(text)?)
}

// Find the first slice of `n` consecutive characters in `chars` that are all
// distinct. Returns the end of the slice range.
//
// This can panic if `chars` contains non-ascii characters.
fn solve(chars: &[char], n: usize) -> usize {
    // counts[c] == number of times the character c occurs in the window
    let mut counts = [0u8; 128];
    // number of distinct characters in the window
    let mut distinct = 0;

    for (i, c) in chars.iter().copied().enumerate() {
        if i >= n {
            // window has n characters in it; remove trailing-edge character x
            // to make room for the new character c
            let x = chars[i - n] as usize;
            counts[x] -= 1;
            if counts[x] == 0 {
                distinct -= 1;
            }
        }

        // add new character to the window
        let c = c as usize;
        if counts[c] == 0 {
            distinct += 1;
            if distinct == n {
                return i + 1;
            }
        }
        counts[c] += 1;
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
