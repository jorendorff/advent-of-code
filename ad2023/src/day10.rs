use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<Vec<u8>>;

const NORTH: u8 = 1;
const EAST: u8 = 2;
const SOUTH: u8 = 4;
const WEST: u8 = 8;
const UNKNOWN: u8 = 255;

#[aoc_generator(day10, part1, jorendorff)]
#[aoc_generator(day10, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines({
        '.' => 0,
        '|' => NORTH | SOUTH,
        '-' => EAST | WEST,
        'L' => NORTH | EAST,
        'J' => NORTH | WEST,
        '7' => SOUTH | WEST,
        'F' => SOUTH | EAST,
        'S' => UNKNOWN
    }+));
    Ok(p.parse(text)?)
}

fn fix_input(input: &mut Input) -> (usize, usize) {
    let h = input.len();
    let w = input[0].len();
    for r in 0..h {
        for c in 0..w {
            let mut s = input[r][c];
            if s == UNKNOWN {
                s = 0;
                if c > 0 && input[r][c - 1] & EAST != 0 {
                    s |= WEST;
                }
                if c < w - 1 && input[r][c + 1] & WEST != 0 {
                    s |= EAST;
                }
                if r > 0 && input[r - 1][c] & SOUTH != 0 {
                    s |= NORTH;
                }
                if r < h - 1 && input[r + 1][c] & NORTH != 0 {
                    s |= SOUTH;
                }
                input[r][c] = s;
                return (r, c);
            }
        }
    }
    panic!("S not found");
}

fn go(dir: u8, r: &mut usize, c: &mut usize) {
    match dir {
        NORTH => *r -= 1,
        SOUTH => *r += 1,
        EAST => *c += 1,
        WEST => *c -= 1,
        _ => panic!("invalid dir"),
    }
}

#[aoc(day10, part1, jorendorff)]
fn part_1(input: &Input) -> usize {
    // 469 on the global leaderboard
    let mut input = input.clone();
    let (r0, c0) = fix_input(&mut input);

    let (mut r, mut c) = (r0, c0);
    let mut came_from = 0;
    let mut num_steps = 0;
    loop {
        let mut goes_to = 0;
        for dir in [NORTH, SOUTH, EAST, WEST] {
            if dir != came_from && input[r][c] & dir != 0 {
                goes_to = dir;
            }
        }
        assert_ne!(goes_to, 0);
        go(goes_to, &mut r, &mut c);
        num_steps += 1;
        if (r, c) == (r0, c0) {
            break;
        }
        came_from = match goes_to {
            NORTH => SOUTH,
            SOUTH => NORTH,
            WEST => EAST,
            EAST => WEST,
            _ => panic!("oh no"),
        };
    }
    num_steps / 2
}

#[aoc(day10, part2, jorendorff)]
fn part_2(input: &Input) -> usize {
    // rank 74 on the global leaderboard
    let mut input = input.clone();
    let h = input.len();
    let w = input[0].len();
    let mut path_dir = vec![vec![(0, 0); w]; h];
    let (r0, c0) = fix_input(&mut input);

    let (mut r, mut c) = (r0, c0);
    let mut came_from = 0;
    loop {
        let mut goes_to = 0;
        for dir in [NORTH, SOUTH, EAST, WEST] {
            if dir != came_from && input[r][c] & dir != 0 {
                goes_to = dir;
            }
        }

        path_dir[r][c] = (came_from, goes_to);

        assert_ne!(goes_to, 0);
        go(goes_to, &mut r, &mut c);
        came_from = match goes_to {
            NORTH => SOUTH,
            SOUTH => NORTH,
            WEST => EAST,
            EAST => WEST,
            _ => panic!("oh no"),
        };
        if (r, c) == (r0, c0) {
            break;
        }
    }

    let p0 = path_dir[r0][c0];
    path_dir[r0][c0].0 = came_from;

    let mut num_enclosed = 0;
    for r in 0..h {
        let mut row_wind_count = 0;
        for c in 0..w {
            if path_dir[r][c] == (0, 0) {
                if row_wind_count != 0 {
                    println!("counting at {r}, {c}; count is {row_wind_count}");
                    num_enclosed += 1;
                }
            } else {
                let (came_from, goes_to) = path_dir[r][c];
                if came_from == SOUTH {
                    println!("up at {r}, {c}");
                    row_wind_count += 1;
                } else if came_from == NORTH {
                    row_wind_count -= 1;
                }

                if goes_to == SOUTH {
                    println!("down at {r}, {c}");
                    row_wind_count -= 1;
                } else if goes_to == NORTH {
                    row_wind_count += 1;
                }
            }
        }
    }
    num_enclosed
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "\
-L|F7
7S-7|
L|7||
-L-J|
L|-JF
";

    const EXAMPLE2: &str = "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";
    
    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE1).unwrap()), 4);
        assert_eq!(part_1(&parse_input(EXAMPLE2).unwrap()), 8);
    }

    const EXAMPLE3: &str = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";

    const EXAMPLE4: &str = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";

    const EXAMPLE5: &str = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE3).unwrap()), 4);
        assert_eq!(part_2(&parse_input(EXAMPLE4).unwrap()), 8);
        assert_eq!(part_2(&parse_input(EXAMPLE5).unwrap()), 10);
    }
}
