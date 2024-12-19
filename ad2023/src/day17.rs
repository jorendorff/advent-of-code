use pathfinding::directed::dijkstra;
use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<Vec<usize>>;

#[aoc_generator(day17, part1, jorendorff)]
#[aoc_generator(day17, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines(line(digit+)));
    Ok(p.parse(text)?)
}

const RIGHT: usize = 0;
const UP: usize = 1;
const LEFT: usize = 2;
const DOWN: usize = 3;

fn reverse(dir: usize) -> usize {
    match dir {
        RIGHT => LEFT,
        UP => DOWN,
        LEFT => RIGHT,
        DOWN => UP,
        _ => panic!(),
    }
}

fn bump_row(row: usize, dir: usize) -> usize {
    row.wrapping_add([0, usize::MAX, 0, 1][dir])
}

fn bump_col(col: usize, dir: usize) -> usize {
    col.wrapping_add([1, 0, usize::MAX, 0][dir])
}

#[aoc(day17, part1, jorendorff)]
fn part_1(input: &Input) -> usize {
    // #101 on the global leaderboard
    let h = input.len();
    let w = input[0].len();

    dijkstra::dijkstra(
        &(0, 0, 0, 0),  // row, col, dir, count
        |&(r, c, dir, count)| {
            [LEFT, RIGHT, UP, DOWN].into_iter().flat_map(move |dir1| {
                if dir1 != reverse(dir) && (dir1 != dir || count < 3) {
                    let r1 = bump_row(r, dir1);
                    let c1 = bump_col(c, dir1);

                    let count1 = if dir1 == dir { count + 1 } else { 1 };
                    if r1 < h && c1 < w {
                        let cost = input[r1][c1];
                        return Some(((r1, c1, dir1, count1), cost));
                    }
                }
                None
            })
        },
        |&(r, c, _dir, _count)| r == h - 1 && c == w - 1,
    )
        .unwrap()
        .1
}

#[aoc(day17, part2, jorendorff)]
fn part_2(input: &Input) -> usize {
    // #68 on the global leaderboard
    let h = input.len();
    let w = input[0].len();

    dijkstra::dijkstra(
        &(0, 0, 0, 0), // row, col, dir, count
        |&(r, c, dir, count)| {
            [LEFT, RIGHT, UP, DOWN].into_iter()
                .filter_map(move |dir1| {
                    if (dir1 == dir && count < 10) || (dir1 != dir && dir1 != reverse(dir) && count >= 4) {
                        let r1 = bump_row(r, dir1);
                        let c1 = bump_col(c, dir1);

                        let count1 = if dir1 == dir { count + 1 } else { 1 };
                        if r1 < h && c1 < w {
                            return Some(((r1, c1, dir1, count1), input[r1][c1]));
                        }
                    }
                    None
                })
        },
        |&(r, c, _dir, count)| r == h - 1 && c == w - 1 && count >= 4,
    )
        .unwrap()
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 102);
    }

    const EXAMPLE_2: &str = "\
111111111111
999999999991
999999999991
999999999991
999999999991
";

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 94);
        assert_eq!(part_2(&parse_input(EXAMPLE_2).unwrap()), 71);
    }
}
