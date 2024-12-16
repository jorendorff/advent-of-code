use std::cmp::Reverse;
use std::collections::*;

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<Vec<char>>;

#[aoc_generator(day16, part1, jorendorff)]
#[aoc_generator(day16, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines(any_char+));
    Ok(p.parse(text)?)
}

const EAST: u8 = 0;

const DIRS: [(isize, isize); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];

fn step(grid: &[Vec<char>], r: usize, c: usize, dir: u8) -> Option<(usize, usize)> {
    let (dr, dc) = DIRS[dir as usize];
    let r1 = (r as isize + dr) as usize;
    let c1 = (c as isize + dc) as usize;
    if r1 >= grid.len() || c1 >= grid[r1].len() || grid[r1][c1] == '#' {
        None
    } else {
        Some((r1, c1))
    }
}

#[aoc(day16, part1, jorendorff)]
fn part_1(input: &Input) -> u64 {
    let mut start = (usize::MAX, usize::MAX);
    let mut end = (usize::MAX, usize::MAX);
    for (r, row) in input.iter().enumerate() {
        for (c, ch) in row.iter().copied().enumerate() {
            match ch {
                'S' => start = (r, c),
                'E' => end = (r, c),
                _ => {}
            }
        }
    }

    let mut seen = HashSet::new();
    let mut todo = BinaryHeap::new();
    fn add(
        seen: &mut HashSet<(usize, usize, u8)>,
        todo: &mut BinaryHeap<(Reverse<u64>, usize, usize, u8)>,
        r: usize,
        c: usize,
        dir: u8,
        score: u64,
    ) {
        if !seen.contains(&(r, c, dir)) {
            seen.insert((r, c, dir));
            todo.push((Reverse(score), r, c, dir));
        }
    }

    add(&mut seen, &mut todo, start.0, start.1, EAST, 0);
    while let Some((Reverse(score), r, c, dir)) = todo.pop() {
        if (r, c) == end {
            return score;
        }
        if let Some((r1, c1)) = step(input, r, c, dir) {
            add(&mut seen, &mut todo, r1, c1, dir, score + 1);
        }
        add(&mut seen, &mut todo, r, c, (dir + 1) % 4, score + 1000);
        add(&mut seen, &mut todo, r, c, (dir + 3) % 4, score + 1000);
    }

    panic!("no route to end");
}

#[aoc(day16, part2, jorendorff)]
fn part_2(input: &Input) -> usize {
    let mut start = (usize::MAX, usize::MAX);
    let mut end = (usize::MAX, usize::MAX);
    for (r, row) in input.iter().enumerate() {
        for (c, ch) in row.iter().copied().enumerate() {
            match ch {
                'S' => start = (r, c),
                'E' => end = (r, c),
                _ => {}
            }
        }
    }

    type Breadcrumbs = HashMap<(usize, usize, u8), (u64, Vec<(usize, usize, u8)>)>;

    let mut seen = Breadcrumbs::new();
    let mut todo = BinaryHeap::new();
    fn add(
        seen: &mut Breadcrumbs,
        todo: &mut BinaryHeap<(Reverse<u64>, usize, usize, u8)>,
        r: usize,
        c: usize,
        dir: u8,
        score: u64,
        prev: (usize, usize, u8),
    ) {
        match seen.entry((r, c, dir)) {
            hash_map::Entry::Vacant(e) => {
                e.insert((score, vec![prev]));
                todo.push((Reverse(score), r, c, dir));
            }
            hash_map::Entry::Occupied(mut e) if score == e.get().0 => {
                e.get_mut().1.push(prev);
            }
            hash_map::Entry::Occupied(mut e) if score < e.get().0 => {
                *e.get_mut() = (score, vec![prev]);
            }

            _ => {}
        }
    }

    let mut best_score: Option<u64> = None;

    add(&mut seen, &mut todo, start.0, start.1, EAST, 0, (usize::MAX, usize::MAX, EAST));
    while let Some((Reverse(score), r, c, dir)) = todo.pop() {
        if let Some(best_score) = best_score {
            if score > best_score {
                break;
            }
        }
        if (r, c) == end {
            best_score = Some(score);
        }
        if let Some((r1, c1)) = step(input, r, c, dir) {
            add(&mut seen, &mut todo, r1, c1, dir, score + 1, (r, c, dir));
        }
        add(&mut seen, &mut todo, r, c, (dir + 1) % 4, score + 1000, (r, c, dir));
        add(&mut seen, &mut todo, r, c, (dir + 3) % 4, score + 1000, (r, c, dir));
    }

    // Now work backwards to find all cells on shortest paths.
    let mut todo: VecDeque<(usize, usize, u8)> = VecDeque::new();
    for dir in 0..4 {
        todo.push_back((end.0, end.1, dir));
    }
    let mut good_tiles = HashSet::new();
    while let Some((r, c, dir)) = todo.pop_front() {
        good_tiles.insert((r, c));
        if let Some(pair) = seen.get(&(r, c, dir)) {
            for &triple in &pair.1 {
                todo.push_back(triple);
            }
        }
    }

    good_tiles.remove(&(usize::MAX, usize::MAX));
    good_tiles.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

    const EXAMPLE2: &str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";
    
    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 7036);
        assert_eq!(part_1(&parse_input(EXAMPLE2).unwrap()), 11048);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 45);
        assert_eq!(part_2(&parse_input(EXAMPLE2).unwrap()), 64);
    }
}
