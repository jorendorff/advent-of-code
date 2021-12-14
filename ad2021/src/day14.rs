use std::collections::HashMap;

use aoc_runner_derive::*;

type Rules = [[u8; 26]; 26];

#[aoc_generator(day14, part1, jorendorff)]
#[aoc_generator(day14, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<(Vec<u8>, Rules)> {
    let sections: Vec<&str> = text.split("\n\n").collect();
    anyhow::ensure!(sections.len() == 2);

    let template: Vec<u8> = sections[0]
        .trim()
        .bytes()
        .map(|b| {
            anyhow::ensure!((b'A'..=b'Z').contains(&b));
            Ok(b - b'A')
        })
        .collect::<anyhow::Result<Vec<u8>>>()?;

    let mut rules = [[0; 26]; 26];

    for line in sections[1].lines() {
        let fields: Vec<&str> = line.split(" -> ").collect();
        anyhow::ensure!(fields.len() == 2, "bad line: {:?}", line);
        anyhow::ensure!(fields[0].len() == 2);
        let b0 = fields[0].as_bytes()[0];
        let b1 = fields[0].as_bytes()[1];
        anyhow::ensure!((b'A'..=b'Z').contains(&b0));
        anyhow::ensure!((b'A'..=b'Z').contains(&b1));
        anyhow::ensure!(fields[1].len() == 1);
        let c = fields[1].as_bytes()[0];
        anyhow::ensure!((b'A'..=b'Z').contains(&c));
        rules[(b0 - b'A') as usize][(b1 - b'A') as usize] = c - b'A';
    }

    Ok((template, rules))
}

type Counts = [u64; 26];

type Cache = HashMap<(u8, u8, usize), Counts>;

fn add(a: &mut Counts, b: &Counts) {
    for i in 0..26 {
        a[i] += b[i];
    }
}

// Count how many of each element exists after expanding the 2-element chain
// `ab` for the given number of `steps`, **not counting** the `b` at the end.
//
// We don't count the last element so that adjacent slices add cleanly.
fn count(rules: &Rules, cache: &mut Cache, a: u8, b: u8, steps: usize) -> Counts {
    if steps == 0 {
        let mut counts = [0; 26];
        counts[a as usize] = 1;
        counts
    } else {
        let key = (a, b, steps);
        match cache.get(&key) {
            Some(result) => *result,
            None => {
                let m = rules[a as usize][b as usize];
                let mut counts = count(rules, cache, a, m, steps - 1);
                add(&mut counts, &count(rules, cache, m, b, steps - 1));
                cache.insert(key, counts);
                counts
            }
        }
    }
}

fn count_all(template: &[u8], rules: &Rules, steps: usize) -> Counts {
    let mut counts = [0; 26];
    let mut cache = HashMap::new();
    for i in 0..template.len() - 1 {
        add(
            &mut counts,
            &count(rules, &mut cache, template[i], template[i + 1], steps),
        );
    }

    // Don't forget to count the last character! See comment on `count`.
    counts[template[template.len() - 1] as usize] += 1;
    counts
}

fn solve(template: &[u8], rules: &Rules, steps: usize) -> u64 {
    let counts = count_all(template, rules, steps);
    let max = counts.iter().copied().max().unwrap();
    let min = counts.iter().copied().filter(|n| *n > 0).min().unwrap();
    max - min
}

#[aoc(day14, part1, jorendorff)]
fn part_1((template, rules): &(Vec<u8>, Rules)) -> u64 {
    solve(template, rules, 10)
}

#[aoc(day14, part2, jorendorff)]
fn part_2((template, rules): &(Vec<u8>, Rules)) -> u64 {
    solve(template, rules, 40)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";

    fn counts(expected: &str) -> Counts {
        let mut c = [0; 26];
        for ch in expected.bytes() {
            assert!((b'A'..=b'Z').contains(&ch));
            c[(ch - b'A') as usize] += 1;
        }
        c
    }

    fn assert_expansion(template: &[u8], rules: &Rules, steps: usize, expected: &str) {
        let actual = count_all(template, rules, steps);
        let expected = counts(expected);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part_1() {
        let (template, rules) = parse_input(EXAMPLE).unwrap();
        assert_expansion(&template, &rules, 1, "NCNBCHB");
        assert_expansion(&template, &rules, 2, "NBCCNBBBCBHCB");
        assert_expansion(&template, &rules, 3, "NBBBCNCCNBBNBNBBCHBHHBCHB");
        assert_expansion(
            &template,
            &rules,
            4,
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB",
        );
        //poly = expand(poly, &rules);
        //assert_eq!(poly.len(), 97);
        //for _ in 5..10 {
        //    poly = expand(poly, &rules);
        //}
        //assert_eq!(poly.len(), 3073);

        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 1588);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 2188189693529);
    }
}
