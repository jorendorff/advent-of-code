use std::collections::{HashMap, HashSet};
use std::fmt::{Display, self, Formatter};

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Host {
    n0: u8,
    n1: u8,
}

impl Host {
    fn starts_with_t(&self) -> bool {
        self.n0 == b't'
    }
}

impl Display for Host {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.n0 as char, self.n1 as char)
    }
}

type Input = Vec<(Host, Host)>;

#[aoc_generator(day23, part1, jorendorff)]
#[aoc_generator(day23, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let host = parser!(n0:alpha n1:alpha => Host { n0: n0 as u32 as u8, n1: n1 as u32 as u8 });
    let p = parser!(lines(host '-' host));
    Ok(p.parse(text)?)
}

#[aoc(day23, part1, jorendorff)]
fn part_1(input: &Input) -> usize {
    let mut neighbors = HashMap::<Host, HashSet<Host>>::new();
    for (a, b) in input {
        neighbors.entry(*a).or_default().insert(*b);
        neighbors.entry(*b).or_default().insert(*a);
    }

    let mut found = HashSet::<[Host; 3]>::new();
    for (a, b) in input {
        if a.starts_with_t() || b.starts_with_t() {
            for c in &neighbors[a] {
                if neighbors[b].contains(c) {
                    let mut triple = [*a, *b, *c];
                    triple.sort_unstable(); // canonical form, since HashSet<String> isn't hashable
                    found.insert(triple);
                }
            }
        }
    }

    found.len()
}

fn is_subsequence<T: PartialEq<U>, U>(needles: &[T], haystack: &[U]) -> bool {
    let mut hay_iter = haystack.iter();
    for needle in needles {
        loop {
            let Some(item) = hay_iter.next() else {
                return false;
            };
            if needle == item {
                break;
            }
        }
    }
    true
}

#[aoc(day23, part2, jorendorff)]
fn part_2(input: &Input) -> String {
    let mut neighbors = HashMap::<Host, Vec<Host>>::new();
    for (a, b) in input {
        neighbors.entry(*a).or_default().push(*b);
        neighbors.entry(*b).or_default().push(*a);
    }

    for list in neighbors.values_mut() {
        list.sort();
        list.dedup();
    }

    let mut best_set = Vec::<Host>::new();
    // Loop invariant: best_set is one of the largest fully connected sets of hosts that contains
    // any of the hosts that have been h0 so far.
    for (h0, hs) in &neighbors {
        if hs.len() > best_set.len() && !best_set.contains(h0) {
            improve(&neighbors, &mut best_set, hs, 0, &mut vec![*h0]);
        }
    }

    best_set.sort_unstable();
    let best_set = best_set.into_iter().map(|h| h.to_string()).collect::<Vec<String>>();
    best_set.join(",")
}

/// If there is a set containing all of `*current_set`
///
/// This mutates current_set locally but on return it is unchanged.
fn improve(
    neighbors: &HashMap<Host, Vec<Host>>,
    best_set: &mut Vec<Host>,
    hs: &[Host],
    start: usize,
    current_set: &mut Vec<Host>,
) {
    for (i, hi) in hs.iter().enumerate().skip(start) {
        let his = &neighbors[hi];
        if his.len() > best_set.len() && {
            let mut sorted = current_set.clone();
            sorted.sort_unstable();
            is_subsequence(&sorted, his)
        } {
            current_set.push(*hi);
            improve(neighbors, best_set, hs, i + 1, current_set);
            if current_set.len() > best_set.len() {
                *best_set = current_set.clone();
            }
            current_set.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 7);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), "co,de,ka,ta");
    }
}
