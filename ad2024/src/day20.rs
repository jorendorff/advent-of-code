use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<Vec<usize>>;

#[aoc_generator(day20, part1, jorendorff)]
#[aoc_generator(day20, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines(char_of(".#SE")+));
    Ok(p.parse(text)?)
}

fn neighbors(nr: usize, nc: usize, r: usize, c: usize) -> impl Iterator<Item = (usize, usize)> {
    [
        (r, c + 1),
        (r.wrapping_sub(1), c),
        (r, c.wrapping_sub(1)),
        (r + 1, c),
    ]
    .into_iter()
    .filter(move |&(r, c)| r < nr && c < nc)
}

fn cheats(nr: usize, nc: usize, r: usize, c: usize, duration: usize) -> impl Iterator<Item = (usize, usize)> {
    (r.saturating_sub(duration)..=(r + duration).min(nr - 1))
        .flat_map(move |rr| {
            let dc = duration - r.abs_diff(rr);
            (c.saturating_sub(dc)..=(c + dc).min(nc - 1))
                .map(move |cc| (rr, cc))
        })
}

fn cheats_that_save_at_least(input: &Input, floor: usize, duration: usize) -> usize {
    let nr = input.len();
    let nc = input[0].len();

    let mut map = input.clone();
    let mut start = None;
    let mut end = None;
    for (r, row) in map.iter_mut().enumerate() {
        for (c, ch) in row.iter_mut().enumerate() {
            match *ch {
                1 => {
                    *ch = usize::MAX;
                }
                2 => {
                    *ch = 0;
                    assert!(start.is_none());
                    start = Some((r, c));
                }
                3 => {
                    *ch = 0;
                    assert!(end.is_none());
                    end = Some((r, c));
                }
                _ => {}
            }
        }
    }
    let start = start.unwrap();
    let end = end.unwrap();

    let mut point = end;
    let mut path = vec![end];
    while point != start {
        let mut n = 0;
        for (rr, cc) in neighbors(nr, nc, point.0, point.1) {
            if map[rr][cc] == 0 && (rr, cc) != end {
                point = (rr, cc);
                n = 1;
                map[rr][cc] = path.len();
                path.push((rr, cc));
            }
        }
        assert_eq!(n, 1);
    }
    println!("reached start!");

    let track_len = path.len() - 1;
    let threshold = track_len - floor;

    path.reverse();

    let mut count = 0;
    for (i, point) in path.iter().copied().enumerate() {
        let (r, c) = point;
        for (rr, cc) in cheats(nr, nc, r, c, duration) {
            if map[rr][cc] < usize::MAX {
                let t = i + r.abs_diff(rr) + c.abs_diff(cc) + map[rr][cc];
                if t <= threshold {
                    println!("found cheat from {point:?} to ({rr}, {cc}) saving {}", track_len - t);
                    count += 1;
                }
            }
        }
    }
    count
}

#[aoc(day20, part1, jorendorff)]
fn part_1(input: &Input) -> usize {
    cheats_that_save_at_least(input, 100, 2)
}

#[aoc(day20, part2, jorendorff)]
fn part_2(input: &Input) -> usize {
    cheats_that_save_at_least(input, 100, 20)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

    #[test]
    fn test_part_1() {
        assert_eq!(
            cheats_that_save_at_least(&parse_input(EXAMPLE).unwrap(), 20, 2),
            5
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            cheats_that_save_at_least(&parse_input(EXAMPLE).unwrap(), 76, 20),
            3
        );
        assert_eq!(
            cheats_that_save_at_least(&parse_input(EXAMPLE).unwrap(), 72, 20),
            29
        );
    }
}
