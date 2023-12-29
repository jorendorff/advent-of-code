use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::{HashMap, HashSet, VecDeque};

type Input = HashMap<String, Vec<String>>;

#[aoc_generator(day25, part1, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines(
        string(alpha+) ": " repeat_sep(string(alpha+), ' ')
    ));

    let mut m: Input = HashMap::new();
    for (a, values) in p.parse(text)? {
        for b in values {
            m.entry(a.clone()).or_default().push(b.clone());
            m.entry(b).or_default().push(a.clone());
        }
    }
    Ok(m)
}

/// Return an arbitrary path from among the shortest paths from a to b.
///
/// Any shortest path may be chosen but the distribution is not uniformly random.
///
/// # Panics
///
/// If there is no path from a to b.
fn shortest_path<'map>(
    map: &'map Input,
    a: &'map str,
    b: &'map str,
) -> Vec<(&'map str, &'map str)> {
    let mut todo: VecDeque<&str> = vec![a].into();
    let mut back: HashMap<&str, Option<&str>> = HashMap::new();
    back.insert(a, None);

    'outer: while let Some(current) = todo.pop_front() {
        let mut forward_edges: Vec<&str> = map
            .get(current)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
            .iter()
            .filter(|next| !back.contains_key(next.as_str()))
            .map(|s| s.as_str())
            .collect();
        forward_edges.shuffle(&mut rand::thread_rng());
        for next in forward_edges {
            back.insert(next, Some(current));
            if next == b {
                break 'outer;
            }
            todo.push_back(next);
        }
    }

    // walk the 'back' mapping to get the path
    let mut path = vec![];
    let mut current = b;
    while let Some(prev) = back.get(current).copied().flatten() {
        path.push((prev, current));
        current = prev;
    }
    path.reverse();
    assert_eq!(path[0].0, a);
    path
}

fn normalize_edge<'a>((a, b): (&'a str, &'a str)) -> (&'a str, &'a str) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

fn connected_size(map: &Input, start: &str) -> usize {
    let mut seen: HashSet<&str> = HashSet::new();
    seen.insert(start);

    let mut todo = vec![start];
    while let Some(current) = todo.pop() {
        if let Some(neighbors) = map.get(current) {
            for next in neighbors {
                if seen.insert(next) {
                    todo.push(next);
                }
            }
        }
    }
    seen.len()
}

#[aoc(day25, part1, jorendorff)]
fn part_1(map: &Input) -> usize {
    let mut counts: HashMap<(&str, &str), usize> = HashMap::new();
    let p = (map.len() as f64).powf(-0.7);

    println!("inverse p is {}", 1.0 / p);

    for a in map.keys() {
        for b in map.keys() {
            if a != b && rand::thread_rng().gen_bool(p) {
                let path = shortest_path(map, a, b);
                for segment in path {
                    *counts.entry(normalize_edge(segment)).or_insert(0) += 1;
                }
            }
        }
    }

    let mut v: Vec<((&str, &str), usize)> = counts.into_iter().collect();
    v.sort_by_key(|pair| pair.1);
    v.reverse();

    println!("{v:#?}");

    let mut map = map.clone();
    for ((a, b), _count) in v.into_iter().take(3) {
        let remove_value = |vec: &mut Vec<String>, item: &str| {
            for (i, elem) in vec.iter().enumerate() {
                if elem == item {
                    vec.swap_remove(i);
                    return;
                }
            }
        };
        remove_value(map.get_mut(a).unwrap(), b);
        remove_value(map.get_mut(b).unwrap(), a);
    }

    let any_key = map.keys().next().unwrap();
    let one_group = connected_size(&map, any_key);
    one_group * (map.len() - one_group)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 54);
    }
}
