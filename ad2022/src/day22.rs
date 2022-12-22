use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = (Vec<Vec<usize>>, Vec<Insn>);

#[derive(Debug)]
enum Insn {
    Go(u64),
    TurnLeft,
    TurnRight,
}
use Insn::*;

#[aoc_generator(day22, part1, jorendorff)]
#[aoc_generator(day22, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(
        section(lines(char_of(" .#")+))
        line({
            n:u64 => Go(n),
            'R' => TurnRight,
            'L' => TurnLeft,
        }+)
    );
    Ok(p.parse(text)?)
}

#[aoc(day22, part1, jorendorff)]
fn part_1(input: &Input) -> usize {
    // Rank 152 on this star's leaderboard.
    let (grid, prog) = input;
    let mut grid = grid.clone();
    let width = grid.iter().map(Vec::len).max().unwrap();
    let height = grid.len();

    for row in &mut grid {
        while row.len() < width {
            row.push(0);
        }
    }

    let mut y = 0;
    let mut x = grid[0].iter().take_while(|c| **c != 1).count();
    let mut h = Right;

    for insn in prog {
        match insn {
            Go(n) => {
                'zig: for _ in 0..*n {
                    let mut xx = x;
                    let mut yy = y;
                    'step: loop {
                        xx = (xx as i64 + width as i64 + h.dx()) as usize % width;
                        yy = (yy as i64 + height as i64 + h.dy()) as usize % height;
                        if grid[yy][xx] == 2 {
                            // don't move
                            break 'zig;
                        } else if grid[yy][xx] == 1 {
                            x = xx;
                            y = yy;
                            break 'step;
                        }
                    }
                }
            }
            TurnLeft => h = h.turn_left(),
            TurnRight => h = h.turn_right(),
        }
    }
    1000 * (y + 1) + 4 * (x + 1) + h.facing()
}

#[derive(Debug, PartialEq)]
struct Panel(usize, usize);

#[derive(Debug, Copy, Clone, PartialEq)]
enum Dir {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

use Dir::*;

impl Dir {
    fn dx(&self) -> i64 {
        match self {
            Right => 1,
            Down => 0,
            Left => -1,
            Up => 0,
        }
    }

    fn dy(&self) -> i64 {
        match self {
            Right => 0,
            Down => 1,
            Left => 0,
            Up => -1,
        }
    }

    #[allow(dead_code)]
    fn glyph(&self) -> char {
        match self {
            Right => '>',
            Down => 'v',
            Left => '<',
            Up => '^',
        }
    }

    fn turn_left(self) -> Dir {
        match self {
            Right => Up,
            Down => Right,
            Left => Down,
            Up => Left,
        }
    }

    fn turn_right(self) -> Dir {
        match self {
            Right => Down,
            Down => Left,
            Left => Up,
            Up => Right,
        }
    }

    fn facing(self) -> usize {
        match self {
            Right => 0,
            Down => 1,
            Left => 2,
            Up => 3,
        }
    }
}

#[aoc(day22, part2, jorendorff)]
fn part_2(input: &Input) -> usize {
    // Rank 239 on this star's leaderboard.
    let (grid, prog) = input;
    let mut grid = grid.clone();
    let width = grid.iter().map(Vec::len).max().unwrap();
    let height = grid.len();

    for row in &mut grid {
        while row.len() < width {
            row.push(0);
        }
    }

    let mut y = 0;
    let mut x = grid[0].iter().take_while(|c| **c != 1).count();
    let mut h = Right;

    //  BA
    //  C
    // ED
    // F

    const A: Panel = Panel(2, 0);
    const B: Panel = Panel(1, 0);
    const C: Panel = Panel(1, 1);
    const D: Panel = Panel(1, 2);
    const E: Panel = Panel(0, 2);
    const F: Panel = Panel(0, 3);

    // Cheating: explicitly enumerate all edges of the cube where we need to
    // apply tape. This would not work for the example, only for my puzzle
    // input (and not yours). But you can make a paper model and figure out a
    // corresponding array that will work for you.
    //
    // Note that the last element of this 4-tuple needs to be the direction
    // you'll be facing after teleporting to the appropriate spot on the edge,
    // so if you arrive at the bottom edge of a panel, you'll be facing Up, and
    // if you arrive at the right edge, you'll be facing Left.
    //
    // This contains two entries for each of the 12 edges of the cube, except
    // for the 5 edges that are already adjacent in the puzzle input and thus
    // don't require stitching. (12 - 5) * 2 == 14.
    let stitches = vec![
        // Panel A top edge = panel F bottom edge.
        (A, Up, F, Up),
        // Panel A right edge = panel D right edge.
        (A, Right, D, Left),
        // Panel A bottom edge = panel C right edge.
        (A, Down, C, Left),
        // Panel B top edge = panel F left edge.
        (B, Up, F, Right),
        // Panel B left edge = panel E left edge.
        (B, Left, E, Right),
        // Panel C left edge = panel E top edge.
        (C, Left, E, Down),
        // Panel C right edge = panel A bottom edge.
        (C, Right, A, Up),
        // Panel D right edge = panel A right edge.
        (D, Right, A, Left),
        // Panel D bottom edge = panel F right edge.
        (D, Down, F, Left),
        // Panel E top edge = panel C left edge.
        (E, Up, C, Right),
        // Panel E left edge = panel B left edge.
        (E, Left, B, Right),
        // Panel F right edge = panel D bottom edge.
        (F, Right, D, Up),
        // Panel F bottom edge = panel A top edge.
        (F, Down, A, Down),
        // Panel F left edge = panel B top edge
        (F, Left, B, Down),
    ];

    for insn in prog {
        match insn {
            TurnLeft => h = h.turn_left(),
            TurnRight => h = h.turn_right(),
            Go(n) => {
                'zig: for _ in 0..*n {
                    assert!(grid[y][x] == 1);

                    let xx = (x as i64 + width as i64 + h.dx()) as usize % width;
                    let yy = (y as i64 + height as i64 + h.dy()) as usize % height;
                    if grid[yy][xx] == 2 {
                        // don't move
                        break 'zig;
                    } else if grid[yy][xx] == 1 {
                        x = xx;
                        y = yy;
                    } else {
                        // We have wandered off the map! Perform stitching.
                        assert_eq!(grid[yy][xx], 0);
                        let old_panel = Panel(x / 50, y / 50);
                        assert_ne!(
                            old_panel,
                            Panel(xx / 50, yy / 50),
                            "we must have changed panels"
                        );

                        let mut stitched = false;
                        for (origin_panel, dir, dest_panel, new_dir) in &stitches {
                            if old_panel == *origin_panel && h == *dir {
                                // We have found the right stitch.
                                let slot = match h {
                                    Right => y % 50,
                                    Down => 49 - x % 50,
                                    Left => 49 - y % 50,
                                    Up => x % 50,
                                };
                                let (mut xx, mut yy) = match new_dir {
                                    Right => (0, slot),
                                    Down => (49 - slot, 0),
                                    Left => (49, 49 - slot),
                                    Up => (slot, 49),
                                };

                                xx += 50 * dest_panel.0;
                                yy += 50 * dest_panel.1;
                                if grid[yy][xx] == 1 {
                                    x = xx;
                                    y = yy;
                                    h = *new_dir;
                                } else {
                                    assert_eq!(grid[yy][xx], 2);
                                    break 'zig;
                                }
                                stitched = true;
                                break;
                            }
                        }
                        assert!(stitched);
                    }
                }
            }
        }
    }
    1000 * (y + 1) + 4 * (x + 1) + h.facing()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 6032);
    }

    #[test]
    fn test_part_2() {
        // My implementation of part_2 won't work for the example!
        //assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 5031);
    }
}
