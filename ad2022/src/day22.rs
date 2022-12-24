use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = (Vec<Vec<Square>>, Vec<Insn>);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Square {
    Blank,
    Open,
    Wall,
}
use Square::*;

#[derive(Debug)]
enum Insn {
    TurnLeft,
    TurnRight,
    Go(u64),
}
use Insn::*;

#[aoc_generator(day22, part1, jorendorff)]
#[aoc_generator(day22, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(
        section(lines({
            ' ' => Blank,
            '.' => Open,
            '#' => Wall,
        }+))
        line({
            'L' => TurnLeft,
            'R' => TurnRight,
            n:u64 => Go(n),
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
            row.push(Blank);
        }
    }

    let mut y = 0;
    let mut x = grid[0].iter().take_while(|c| **c != Open).count();
    let mut h = Right;

    for insn in prog {
        match insn {
            TurnLeft => h = h.turn_left(),
            TurnRight => h = h.turn_right(),
            Go(n) => {
                'zig: for _ in 0..*n {
                    let mut xx = x;
                    let mut yy = y;
                    'step: loop {
                        xx = (xx as i64 + width as i64 + h.dx()) as usize % width;
                        yy = (yy as i64 + height as i64 + h.dy()) as usize % height;
                        match grid[yy][xx] {
                            Wall => {
                                // don't move
                                break 'zig;
                            }
                            Open => {
                                x = xx;
                                y = yy;
                                break 'step;
                            }
                            Blank => {
                                // just keep going until we wrap around the edge
                            }
                        }
                    }
                }
            }
        }
    }
    1000 * (y + 1) + 4 * (x + 1) + h.facing()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    fn flip(self) -> Dir {
        self.turn_right().turn_right()
    }

    fn facing(self) -> usize {
        self as usize
    }
}

type Panel = usize;

struct CubeMap {
    edges: [[Option<(Panel, Dir)>; 4]; 6],
    count: usize,
}

impl CubeMap {
    fn new() -> Self {
        CubeMap {
            edges: [[None; 4]; 6],
            count: 0,
        }
    }

    fn apply_tape(&mut self, (p1, d1): (Panel, Dir), (p2, d2): (Panel, Dir)) {
        assert_eq!(self.edges[p1][d1 as usize], None);
        self.edges[p1][d1 as usize] = Some((p2, d2));
        assert_eq!(self.edges[p2][d2.flip() as usize], None);
        self.edges[p2][d2.flip() as usize] = Some((p1, d1.flip()));
        self.count += 1;
    }

    fn finish(&mut self) {
        // Fold up the cube. This code is neat!
        while self.count < 12 {
            let count_before = self.count;
            for origin in 0..6 {
                for dir in [Right, Down, Left, Up] {
                    if self.edges[origin][dir as usize].is_none() {
                        // Try folding in from the right.
                        if let Some((p, d)) = self.edges[origin][dir.turn_right() as usize]
                            .and_then(|(p, d)| self.edges[p][d.turn_left() as usize])
                        {
                            self.apply_tape((origin, dir), (p, d.turn_right()));
                            continue;
                        }

                        // Try folding in from the left.
                        if let Some((p, d)) = self.edges[origin][dir.turn_left() as usize]
                            .and_then(|(p, d)| self.edges[p][d.turn_right() as usize])
                        {
                            self.apply_tape((origin, dir), (p, d.turn_left()));
                        }
                    }
                }
            }
            assert!(self.count > count_before, "progress");
        }
    }

    fn get(&self, (depart_panel, depart_dir): (Panel, Dir)) -> (Panel, Dir) {
        self.edges[depart_panel][depart_dir as usize].unwrap()
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
        row.resize(width, Blank);
    }

    // Cube size.
    let cs = {
        let area = grid
            .iter()
            .flat_map(|row| row.iter().filter(|x| **x != Blank))
            .count();
        assert_eq!(area % 6, 0);
        let s = ((area / 6) as f64).sqrt() as usize;
        assert_eq!(6 * s * s, area);
        s
    };
    assert_eq!(height % cs, 0);
    let height_panels = height / cs;
    assert_eq!(width % cs, 0);
    let width_panels = width / cs;

    // Compute data used in teleportation.
    // `panels` lists (x, y) coordinates of all faces of the cube
    // coordinates are in panels (1/cs scale), not grid-squares
    let mut panels = vec![];
    // mini_map[y][x] = i whenever panels[i] == (x, y)
    let mut mini_map = vec![vec![None; width_panels]; height_panels];
    // `cube` tracks relationships between panels along edges
    let mut cube = CubeMap::new();
    for px in 0..width_panels {
        for py in 0..height_panels {
            if grid[py * cs][px * cs] != Blank {
                let curr = panels.len();
                if py > 0 {
                    if let Some(prev) = mini_map[py - 1][px] {
                        cube.apply_tape((prev, Down), (curr, Down));
                    }
                }
                if px > 0 {
                    if let Some(prev) = mini_map[py][px - 1] {
                        cube.apply_tape((prev, Right), (curr, Right));
                    }
                }
                mini_map[py][px] = Some(curr);
                panels.push((px, py));
            }
        }
    }
    assert_eq!(panels.len(), 6);
    cube.finish();

    let mut y = 0;
    let mut x = grid[0].iter().take_while(|c| **c != Open).count();
    let mut h = Right;
    for insn in prog {
        match insn {
            TurnLeft => h = h.turn_left(),
            TurnRight => h = h.turn_right(),
            Go(n) => {
                'zig: for _ in 0..*n {
                    assert_eq!(grid[y][x], Open);

                    // The casts to usize here wrap to a huge value if x+dx or
                    // y+dy goes negative. We need to know we left the panel.
                    let xx = (x as i64 + h.dx()) as usize;
                    let yy = (y as i64 + h.dy()) as usize;
                    let sq = if xx >= width || yy >= height {
                        Blank
                    } else {
                        grid[yy][xx]
                    };

                    match sq {
                        Wall => {
                            // don't move
                            break 'zig;
                        }
                        Open => {
                            x = xx;
                            y = yy;
                        }
                        Blank => {
                            // We have wandered off the map! Perform stitching.
                            assert_ne!(
                                (x / cs, y / cs),
                                (xx / cs, yy / cs),
                                "we must have changed panels"
                            );
                            let old_panel = mini_map[y / cs][x / cs].unwrap();
                            let (dest_panel, new_dir) = cube.get((old_panel, h));

                            let slot = match h {
                                Right => y % cs,
                                Down => (cs - 1) - x % cs,
                                Left => (cs - 1) - y % cs,
                                Up => x % cs,
                            };
                            let max = cs - 1;
                            let (mut xx, mut yy) = match new_dir {
                                Right => (0, slot),
                                Down => (max - slot, 0),
                                Left => (max, max - slot),
                                Up => (slot, max),
                            };

                            xx += cs * panels[dest_panel].0;
                            yy += cs * panels[dest_panel].1;
                            match grid[yy][xx] {
                                Open => {
                                    x = xx;
                                    y = yy;
                                    h = new_dir;
                                }
                                Wall => break 'zig,
                                Blank => panic!("wrong turn in teleportation"),
                            }
                        }
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
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 5031);
    }
}
