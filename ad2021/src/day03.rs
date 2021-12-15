use aoc_runner_derive::*;

#[aoc_generator(day3, part1, jorendorff)]
#[aoc_generator(day3, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Vec<String>> {
    text.lines().map(|line| Ok(line.to_string())).collect()
}

#[aoc(day3, part1, jorendorff)]
fn part_1(lines: &[String]) -> u64 {
    let nbits = lines[0].len();
    let mut counts = vec![0_usize; nbits];
    for line in lines {
        for (total, c) in counts.iter_mut().zip(line.chars()) {
            if c == '1' {
                *total += 1;
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    for count in counts {
        gamma <<= 1;
        epsilon <<= 1;
        if count > lines.len() - count {
            gamma |= 1;
        } else {
            epsilon |= 1;
        }
    }

    gamma * epsilon
}

#[aoc(day3, part2, jorendorff)]
fn part_2(lines: &[String]) -> u64 {
    let nbits = lines[0].len();
    let mut numbers = vec![];
    let mut counts = vec![0_usize; nbits];
    for line in lines {
        let mut n = 0;
        for (total, c) in counts.iter_mut().zip(line.chars()) {
            n <<= 1;
            if c == '1' {
                *total += 1;
                n |= 1;
            }
        }
        numbers.push(n);
    }

    let mut o2 = numbers.clone();
    let mut bit = 1 << (nbits - 1);
    while o2.len() > 1 {
        let ones = o2.iter().cloned().filter(|n| *n & bit == bit).count();
        let most_common_value = ones >= (o2.len() - ones);
        o2.retain(|i| (*i & bit == bit) == most_common_value);
        bit >>= 1;
    }

    let mut co2 = numbers;
    bit = 1 << (nbits - 1);
    while co2.len() > 1 {
        let ones = co2.iter().cloned().filter(|n| *n & bit == bit).count();
        let least_common_value = ones < co2.len() - ones;
        co2.retain(|i| (*i & bit == bit) == least_common_value);
        bit >>= 1;
    }

    o2.pop().unwrap() * co2.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 198);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 230);
    }
}
