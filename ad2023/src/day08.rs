use std::collections::HashMap;

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = (Vec<Dir>, HashMap<String, (String, String)>);

enum Dir {
    Left,
    Right,
}

use Dir::*;

#[aoc_generator(day8, part1, jorendorff)]
#[aoc_generator(day8, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(
        section(line({'R' => Right, 'L' => Left}+))
        section(hash_map(lines(
            node:string(alnum+) " = (" l:string(alnum+) ", " r:string(alnum+) ")"
                => (node, (l, r))
        )))
    );
    Ok(p.parse(text)?)
}

#[aoc(day8, part1, jorendorff)]
fn part_1(input: &Input) -> usize {
    let (path, nodes) = input;
    let mut pos = "AAA";
    let mut num_moves = 0;
    while pos != "ZZZ" {
        let opts = nodes.get(pos).unwrap();
        pos = match path[num_moves % path.len()] {
            Left => &opts.0,
            Right => &opts.1,
        };
        num_moves += 1;
    }
    num_moves
}

#[derive(Debug)]
struct Cycle {
    hits: Vec<usize>,
    cycle_start: usize,
    len: usize,
}

impl Cycle {
    fn starting_at(path: &[Dir], nodes: &HashMap<String, (String, String)>, start: &str) -> Self {
        let mut pos = start;
        let mut hits = vec![];
        let mut backmap: HashMap<(&str, usize), usize> = HashMap::new();
        let mut time = 0;
        let cycle_start = loop {
            let path_index = time % path.len();
            if let Some(time_last_here) = backmap.get(&(pos, path_index)) {
                break *time_last_here;
            }
            backmap.insert((pos, path_index), time);
            let opts = nodes.get(pos).unwrap();
            pos = match path[path_index] {
                Left => &opts.0,
                Right => &opts.1,
            };
            time += 1;
            if pos.ends_with('Z') {
                hits.push(time);
            }
        };

        Cycle {
            hits,
            cycle_start,
            len: time - cycle_start,
        }
    }
}

#[aoc(day8, part2, jorendorff)]
fn part_2(input: &Input) -> usize {
    let (path, nodes) = input;

    let cycles = nodes
        .keys()
        .filter(|key| key.ends_with('A'))
        .map(|key| Cycle::starting_at(path, nodes, key))
        .collect::<Vec<Cycle>>();

    // do something that is not fully general (could get a wrong answer)
    let mut n = 1;
    for cycle in &cycles {
        if cycle.hits != [cycle.len] {
            eprintln!("WARNING: assumption violated, algorithm is unsound");
        }
        n = num::integer::lcm(n, cycle.len);
    }

    // now check it
    for cycle in &cycles {
        assert!(cycle
            .hits
            .contains(&(cycle.cycle_start + (n - cycle.cycle_start) % cycle.len)));
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

    const EXAMPLE2: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";
    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE1).unwrap()), 2);
        assert_eq!(part_1(&parse_input(EXAMPLE2).unwrap()), 6);
    }

    const EXAMPLE3: &str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE3).unwrap()), 6);
    }
}
