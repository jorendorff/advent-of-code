use std::collections::{HashMap, VecDeque};

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = (usize, Vec<Valve>);

#[derive(Clone)]
struct Valve {
    flow_rate: u64,
    neighbors: Vec<usize>,
}

#[aoc_generator(day16, part1, jorendorff)]
#[aoc_generator(day16, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines(
        "Valve " n:string(alpha+) " has flow rate=" f:u64
            "; " _q:{"tunnels lead to valves", "tunnel leads to valve"} " "
            ns:repeat_sep(string(alpha+), ", ")
            => (n, f, ns)
    ));
    let triples = p.parse(text)?;

    let name_to_index: HashMap<String, usize> = triples
        .iter()
        .map(|(name, _, _)| name)
        .cloned()
        .enumerate()
        .map(|(i, name)| (name, i))
        .collect();

    Ok((
        name_to_index["AA"],
        triples
            .into_iter()
            .map(|(_name, flow_rate, neighbors)| Valve {
                flow_rate,
                neighbors: neighbors
                    .into_iter()
                    .map(|name| name_to_index[&name])
                    .collect(),
            })
            .collect(),
    ))
}

fn search(
    valves: &[Valve],
    time_limit: usize,
    time: usize,
    loc: usize,
    open: u128,
    score: u64,
) -> u64 {
    if time >= time_limit {
        return score;
    }
    let mut best = score;
    let mut seen = 1u128 << loc;
    let mut todo = VecDeque::from([(time, loc)]);
    while let Some((t, p)) = todo.pop_front() {
        if t >= time_limit {
            break;
        }
        let flow = valves[p].flow_rate;
        if flow > 0 && open & (1 << p) == 0 {
            let stab = search(
                valves,
                time_limit,
                t + 1,
                p,
                open | (1 << p),
                score + (time_limit as u64 - (t as u64 + 1)) * flow,
            );
            best = best.max(stab);
        }
        for &n in &valves[p].neighbors {
            if seen & (1 << n) == 0 {
                todo.push_back((t + 1, n));
                seen |= 1 << n;
            }
        }
    }
    best
}

#[aoc(day16, part1, jorendorff)]
fn part_1(input: &Input) -> u64 {
    let (start, valves) = input;
    search(valves, 30, 0, *start, 0, 0)
}

#[aoc(day16, part2, jorendorff)]
fn part_2(input: &Input) -> u64 {
    let (start, valves) = input;
    let start = *start;
    let valves: &[Valve] = valves;

    let vi: Vec<usize> = valves
        .iter()
        .enumerate()
        .filter(|pair| pair.1.flow_rate != 0)
        .map(|pair| pair.0)
        .collect();

    let n = vi.len();
    let mut best = 0;
    for i in 0..(1 << (n as u32)) {
        // make a copy with only some enabled
        let mut map1 = valves.to_vec();
        for j in 0..n {
            if i & (1 << j) == 0 {
                map1[vi[j]].flow_rate = 0;
            }
        }
        let my_best = search(&map1, 26, 0, start, 0, 0);

        // the other ones enabled
        let mut map2 = valves.to_vec();
        for j in 0..n {
            if i & (1 << j) != 0 {
                map2[vi[j]].flow_rate = 0;
            }
        }
        let elephant_best = search(&map2, 26, 0, start, 0, 0);

        let stab = my_best + elephant_best;
        best = best.max(stab);
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 1651);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 1707);
    }
}
