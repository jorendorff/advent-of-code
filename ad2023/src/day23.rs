use std::collections::*;

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<Vec<usize>>;

#[aoc_generator(day23, part1, jorendorff)]
#[aoc_generator(day23, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines(char_of(".#^>v<")+));
    Ok(p.parse(text)?)
}

//const PATH: usize = 0;
const FOREST: usize = 1;
const UP: usize = 2;
const RIGHT: usize = 3;
const DOWN: usize = 4;
const LEFT: usize = 5;

fn flip(dir: usize) -> usize {
    match dir {
        UP => DOWN,
        RIGHT => LEFT,
        DOWN => UP,
        LEFT => RIGHT,
        _ => panic!(),
    }
}

fn go(p: (usize, usize), dir: usize) -> (usize, usize) {
    match dir {
        UP => (p.0.wrapping_sub(1), p.1),
        RIGHT => (p.0, p.1 + 1),
        DOWN => (p.0 + 1, p.1),
        LEFT => (p.0, p.1.wrapping_sub(1)),
        _ => panic!(),
    }
}

#[aoc(day23, part1, jorendorff)]
fn part_1(grid: &Input) -> usize {
    let nr = grid.len();
    let nc = grid[0].len();

    // 312
    #[derive(Debug, Clone, PartialOrd, Eq, PartialEq, Ord)]
    struct State {
        visited: Vec<Vec<bool>>,
        cur: (usize, usize),
    }

    let mut states: BTreeSet<State> = BTreeSet::new();
    states.insert(State {
        visited: vec![vec![false; nc]; nr],
        cur: (0, 1),
    });

    let mut t = 0;
    while !states.is_empty() {
        states = states
            .into_iter()
            .flat_map(|state| {
                [UP, RIGHT, DOWN, LEFT].into_iter().filter_map(move |dir| {
                    let (r, c) = state.cur;
                    if grid[r][c] >= UP && grid[r][c] != dir {
                        None
                    } else {
                        let dest = go(state.cur, dir);
                        let (r1, c1) = dest;
                        if r1 >= grid.len()
                            || c1 >= grid[0].len()
                            || grid[r1][c1] == FOREST
                            || state.visited[r1][c1]
                        {
                            None
                        } else {
                            let mut visited = state.visited.clone();
                            visited[r1][c1] = true;
                            Some(State { visited, cur: dest })
                        }
                    }
                })
            })
            .collect();
        t += 1;
    }
    t - 1
}

fn is_intersection(grid: &Input, point: (usize, usize)) -> bool {
    let (r, _c) = point;

    r == 0
        || r == grid.len() - 1
        || [UP, RIGHT, DOWN, LEFT]
            .into_iter()
            .filter(|&dir| {
                let (r1, c1) = go(point, dir);
                grid[r1][c1] != FOREST
            })
            .count()
            > 2
}

#[aoc(day23, part2, jorendorff)]
fn part_2(grid: &Input) -> usize {
    let nr = grid.len();
    let nc = grid[0].len();

    #[derive(Debug)]
    struct Node {
        p: (usize, usize),
        edges: Vec<(usize, usize)>, // (dest_node_idx, num_steps)
    }

    let mut visited = vec![vec![false; nc]; nr];
    visited[0][1] = true;
    let mut nodes: Vec<Node> = vec![Node {
        p: (0, 1),
        edges: vec![],
    }];
    let mut to_node: HashMap<(usize, usize), usize> = HashMap::new();
    to_node.insert((0, 1), 0);

    // 1. Build map
    struct State {
        from: usize,
        from_dir: usize,
        distance: usize,
        curr: (usize, usize),
    }

    let mut todo = vec![State {
        from: 0,
        from_dir: UP,
        distance: 0,
        curr: (0, 1),
    }];
    while let Some(state) = todo.pop() {
        //println!("at {:?}", state.curr);
        for dir in [UP, RIGHT, DOWN, LEFT] {
            if dir == state.from_dir {
                continue;
            }

            let dest: (usize, usize) = go(state.curr, dir);
            let (r1, c1) = dest;
            if r1 < nr && grid[r1][c1] != FOREST {
                if is_intersection(grid, dest) {
                    let dest_node = *to_node.entry(dest).or_insert_with(|| {
                        let new_node = nodes.len();
                        // println!("adding new node at {dest:?}");

                        // for r in &visited {
                        //     for &c in r {
                        //         print!("{}", if c { "O" } else {"."});
                        //     }
                        //     println!();
                        // }

                        nodes.push(Node {
                            p: dest,
                            edges: vec![],
                        });
                        todo.push(State {
                            from: new_node,
                            from_dir: flip(dir),
                            distance: 0,
                            curr: dest,
                        });
                        new_node
                    });

                    if !nodes[state.from]
                        .edges
                        .iter()
                        .any(|&(dest, _)| dest == dest_node)
                    {
                        let num_steps = state.distance + 1;
                        nodes[state.from].edges.push((dest_node, num_steps));
                        nodes[dest_node].edges.push((state.from, num_steps));
                    }
                } else {
                    todo.push(State {
                        from: state.from,
                        from_dir: flip(dir),
                        distance: state.distance + 1,
                        curr: dest,
                    });
                }
                visited[r1][c1] = true;
            }
        }
    }

    // for (i, node) in nodes.iter().enumerate() {
    //     println!("{i}. {node:?}");
    // }

    // 2. Walk map
    let out = nodes.iter().position(|node| node.p.0 == nr - 1).unwrap();
    let mut best = 0;
    let mut visited = vec![false; nodes.len()];
    let mut todo = vec![(0, 0, 0)];
    visited[0] = true;
    while let Some((node, edge_idx, distance)) = todo.pop() {
        assert!(visited[node]);
        if edge_idx < nodes[node].edges.len() {
            todo.push((node, edge_idx + 1, distance));
            let (dest_node, num_steps) = nodes[node].edges[edge_idx];
            if dest_node == out {
                // println!("found route of length {}: {:?}", distance + num_steps, todo.iter().map(|r| r.0).collect::<Vec<_>>());
                best = best.max(distance + num_steps);
            } else if visited[dest_node] {
                // do nothing
            } else {
                todo.push((dest_node, 0, distance + num_steps));
                //println!("entering {dest_node:?}");
                visited[dest_node] = true;
            }
        } else {
            //println!("leaving {node:?}");
            visited[node] = false;
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 94);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 154);
    }
}
