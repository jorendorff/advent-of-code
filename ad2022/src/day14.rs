use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

#[derive(Clone, Copy)]
struct Point(usize, usize);

type Input = Vec<Vec<Point>>;

#[aoc_generator(day14, part1, jorendorff)]
#[aoc_generator(day14, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines(
        repeat_sep(x:usize ',' y:usize => Point(x, y), " -> ")
    ));
    Ok(p.parse(text)?)
}

const AIR: u8 = b'.';
const ROCK: u8 = b'#';
const SAND: u8 = b'o';

#[allow(clippy::needless_range_loop)]
fn draw_line(grid: &mut [Vec<u8>], p: Point, q: Point) {
    if p.0 == q.0 {
        // vertical line
        let c = p.0;
        for r in p.1.min(q.1)..=p.1.max(q.1) {
            grid[r][c] = ROCK;
        }
    } else {
        // horizontal line
        assert_eq!(p.1, q.1);
        let r = p.1;
        for c in p.0.min(q.0)..=p.0.max(q.0) {
            grid[r][c] = ROCK;
        }
    }
}

fn draw_grid(paths: &Input) -> Vec<Vec<u8>> {
    let mut grid = vec![vec![AIR; 1001]; 1000];

    for path in paths {
        for pair in path.windows(2) {
            draw_line(&mut grid, pair[0], pair[1]);
        }
    }
    grid
}

fn try_dropping_sand(grid: &mut Vec<Vec<u8>>) -> bool {
    let mut r = 0;
    let mut c = 500;
    loop {
        if r == grid.len() - 1 {
            return false;
        } else if grid[r + 1][c] == AIR {
            r += 1;
        } else if grid[r + 1][c - 1] == AIR {
            r += 1;
            c -= 1;
        } else if grid[r + 1][c + 1] == AIR {
            r += 1;
            c += 1;
        } else {
            grid[r][c] = SAND;
            return true;
        }
    }
}

fn dump_grid(grid: &[Vec<u8>], y_max: usize) {
    for row in &grid[0..=y_max] {
        println!("{}", std::str::from_utf8(&row[200..800]).unwrap());
    }
    std::thread::sleep(std::time::Duration::from_millis(100));
}

#[aoc(day14, part1, jorendorff)]
fn part_1(input: &Input) -> usize {
    let mut grid = draw_grid(input);
    for i in 0..1_000_000 {
        if !try_dropping_sand(&mut grid) {
            return i;
        }
    }
    panic!("sand kept coming to rest");
}

#[aoc(day14, part2, jorendorff)]
fn part_2(input: &Input) -> usize {
    let mut grid = draw_grid(input);
    let floor_y = input.iter().flatten().map(|p| p.1).max().unwrap() + 2;
    draw_line(&mut grid, Point(0, floor_y), Point(1000, floor_y));

    for i in 1..1_000_000 {
        assert!(try_dropping_sand(&mut grid));
        if grid[0][500] == SAND {
            return i;
        }
        if i % 16 == 0 {
            dump_grid(&grid, floor_y);
        }
    }
    panic!("sand seemed to go forever");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 24);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 93);
    }
}
