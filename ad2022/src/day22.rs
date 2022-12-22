use std::collections::{HashMap, HashSet};

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
struct Panel(usize, usize);

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

struct CubeMap {
    panels: HashSet<Panel>,
    edges: HashMap<(Panel, Dir), (Panel, Dir)>,
}

impl CubeMap {
    fn new() -> Self {
        CubeMap {
            panels: HashSet::new(),
            edges: HashMap::new(),
        }
    }

    fn add(&mut self, (p1, d1): (Panel, Dir), (p2, d2): (Panel, Dir)) {
        self.panels.insert(p1);
        self.panels.insert(p2);
        Self::add2(&mut self.edges, p1, d1, p2, d2);
    }

    fn add2(
        edges: &mut HashMap<(Panel, Dir), (Panel, Dir)>,
        p1: Panel,
        d1: Dir,
        p2: Panel,
        d2: Dir,
    ) {
        let inserted = edges.insert((p1, d1), (p2, d2));
        assert_eq!(inserted, None);
        let inserted = edges.insert((p2, d2.flip()), (p1, d1.flip()));
        assert_eq!(inserted, None);
    }

    fn finish(&mut self) {
        // Fold up the cube. This code is neat!
        assert_eq!(self.panels.len(), 6);
        while self.edges.len() < 24 {
            let mut progress = false;
            for &origin in &self.panels {
                for dir in [Right, Down, Left, Up] {
                    if !self.edges.contains_key(&(origin, dir)) {
                        // Try going to the right.
                        if let Some(&(p, d)) = self
                            .edges
                            .get(&(origin, dir.turn_right()))
                            .and_then(|&(p, d)| self.edges.get(&(p, d.turn_left())))
                        {
                            Self::add2(&mut self.edges, origin, dir, p, d.turn_right());
                            progress = true;
                            continue;
                        }

                        // Try going to the left.
                        if let Some(&(p, d)) = self
                            .edges
                            .get(&(origin, dir.turn_left()))
                            .and_then(|&(p, d)| self.edges.get(&(p, d.turn_right())))
                        {
                            Self::add2(&mut self.edges, origin, dir, p, d.turn_left());
                            progress = true;
                        }
                    }
                }
            }
            assert!(progress);
        }
    }

    fn get(&self, before: (Panel, Dir)) -> (Panel, Dir) {
        self.edges[&before]
    }
}

#[aoc(day22, part2, jorendorff)]
fn part_2(input: &Input) -> usize {
    // Rank 239 on this star's leaderboard.
    let (grid, prog) = input;
    let mut grid = grid.clone();
    let width = grid.iter().map(Vec::len).max().unwrap();
    let height = grid.len();

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

    for row in &mut grid {
        while row.len() < width {
            row.push(Blank);
        }
    }

    let is_filled = |px: usize, py: usize| grid[py * cs][px * cs] != Blank;
    let mut cube = CubeMap::new();
    for px in 0..width_panels {
        for py in 0..height_panels {
            if is_filled(px, py) {
                let p = Panel(px, py);
                if px + 1 < width_panels && is_filled(px + 1, py) {
                    cube.add((p, Right), (Panel(px + 1, py), Right));
                }
                if py + 1 < height_panels && is_filled(px, py + 1) {
                    cube.add((p, Down), (Panel(px, py + 1), Down));
                }
            }
        }
    }
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
                            let old_panel = Panel(x / cs, y / cs);
                            assert_ne!(
                                old_panel,
                                Panel(xx / cs, yy / cs),
                                "we must have changed panels"
                            );

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

                            xx += cs * dest_panel.0;
                            yy += cs * dest_panel.1;
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
