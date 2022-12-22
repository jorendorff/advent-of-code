#![allow(clippy::needless_range_loop, clippy::collapsible_if)]

use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<i64>;

#[aoc_generator(day17, part1, jorendorff)]
#[aoc_generator(day17, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(line({
        '<' => -1,
        '>' => 1,
    }+));
    Ok(p.parse(text)?)
}

fn rocks() -> Vec<Vec<Vec<char>>> {
    vec![
        vec![vec!['#', '#', '#', '#']],
        vec![
            vec!['.', '#', '.'],
            vec!['#', '#', '#'],
            vec!['.', '#', '.'],
        ],
        vec![
            vec!['.', '.', '#'],
            vec!['.', '.', '#'],
            vec!['#', '#', '#'],
        ],
        vec![vec!['#'], vec!['#'], vec!['#'], vec!['#']],
        vec![vec!['#', '#'], vec!['#', '#']],
    ]
}

struct Grid {
    grid: VecDeque<Vec<char>>,
    highest_rock: usize,
    dropped_rows: usize,
}

const WELL_WIDTH: usize = 7;

impl Grid {
    fn new() -> Self {
        Grid {
            grid: VecDeque::new(),
            highest_rock: 0,
            dropped_rows: 0,
        }
    }

    fn state(&self) -> String {
        let mut s = String::new();
        for row in self.grid.iter().skip(self.highest_rock) {
            for c in row {
                s.push(*c);
            }
        }
        s
    }

    fn accommodate_rock(&mut self, rock: &[Vec<char>]) {
        let h = rock.len();
        while self.highest_rock > h + 3 {
            self.grid.pop_front();
            self.highest_rock -= 1;
        }
        while self.highest_rock < h + 3 {
            self.grid.push_front(vec!['.'; WELL_WIDTH]);
            self.highest_rock += 1;
        }
    }

    fn tower_height(&self) -> usize {
        self.grid.len() - self.highest_rock + self.dropped_rows
    }

    fn rock_can_move_by(
        &mut self,
        rock: &[Vec<char>],
        x: usize,
        y: usize,
        dx: isize,
        dy: isize,
    ) -> bool {
        let w = rock[0].len();
        let h = rock.len();
        let x = ((x as isize) + dx) as usize;
        let y = ((y as isize) + dy) as usize;
        if y + h > self.grid.len() {
            return false;
        }

        for r in 0..h {
            for c in 0..w {
                if rock[r][c] == '#' {
                    if self.grid[y + r][x + c] != '.' {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn drop_rock(&mut self, rock: &[Vec<char>], jet_pattern: &[i64], jet_iter: &mut usize) {
        let w = rock[0].len();
        let h = rock.len();
        self.accommodate_rock(rock);

        let mut y = 0;
        let mut x = 2;

        loop {
            let jet = jet_pattern[*jet_iter];
            *jet_iter = (*jet_iter + 1) % jet_pattern.len();
            match jet {
                -1 => {
                    if x > 0 && self.rock_can_move_by(rock, x, y, -1, 0) {
                        x -= 1
                    }
                }
                1 => {
                    if x + w < WELL_WIDTH && self.rock_can_move_by(rock, x, y, 1, 0) {
                        x += 1
                    }
                }
                _ => unreachable!("invalid jet value or end of jets"),
            }
            if self.rock_can_move_by(rock, x, y, 0, 1) {
                y += 1;
            } else {
                break;
            }
        }

        //paste rock into grid
        for r in 0..h {
            for c in 0..w {
                if rock[r][c] == '#' {
                    self.grid[y + r][x + c] = '#';
                }
            }
        }
        self.highest_rock = self.highest_rock.min(y);

        // cleanup - not a true flood search, I don't think it's needed
        let mut floodbits = 0b1111111;
        for (i, row) in self.grid.iter().enumerate() {
            for c in 0..WELL_WIDTH {
                if row[c] == '#' {
                    floodbits &= !(1 << c);
                }
            }
            for c in 0..WELL_WIDTH {
                if floodbits & (1 << c) != 0 {
                    for c1 in (0..c).rev() {
                        if row[c1] != '#' {
                            floodbits |= 1 << c1;
                        } else {
                            break;
                        }
                    }
                    for c1 in c + 1..WELL_WIDTH {
                        if row[c1] != '#' {
                            floodbits |= 1 << c1;
                        } else {
                            break;
                        }
                    }
                }
            }
            if floodbits == 0 {
                self.dropped_rows += self.grid.len() - i;
                self.grid.truncate(i);
                return;
            }
        }
    }

    #[allow(dead_code)]
    fn dump(&self) {
        for (r, row) in self.grid.iter().enumerate() {
            for c in row.iter() {
                print!("{c}");
            }
            if r == self.highest_rock {
                print!(" <-- highest rock");
            }
            println!();
        }
        println!("-------");
        println!();
    }
}

#[aoc(day17, part1, jorendorff)]
fn part_1(jets: &Input) -> usize {
    // rank 193
    let rocks = rocks();
    let mut jet_iter = 0;
    let mut rock_iter = rocks.iter().cycle();

    let mut grid = Grid::new();
    for _ in 0..2022 {
        let next_rock = rock_iter.next().unwrap();
        grid.drop_rock(next_rock, jets, &mut jet_iter);
    }

    grid.tower_height()
}

#[aoc(day17, part2, jorendorff)]
fn part_2(jets: &Input) -> usize {
    // rank 459
    let mut jet_iter = 0;
    let rocks = rocks();

    #[derive(Hash, PartialEq, Eq)]
    struct State {
        jet: usize,
        grid: String,
    }

    let mut cache: HashMap<State, (usize, usize)> = HashMap::new();

    let mut grid = Grid::new();
    let goal = 1000000000000usize;
    let mut n = 0;
    let mut bonus = 0;
    loop {
        for rock in &rocks {
            grid.drop_rock(rock, jets, &mut jet_iter);
            n += 1;
            if n == goal {
                return grid.tower_height() + bonus;
            }
        }
        let state = State {
            jet: jet_iter,
            grid: grid.state(),
        };
        match cache.entry(state) {
            Entry::Vacant(e) => {
                e.insert((n, grid.tower_height()));
            }
            Entry::Occupied(e) => {
                let (n0, h0) = *e.get();
                let dn = n - n0;
                let dh = grid.tower_height() - h0;
                let num_leaps = (goal - n) / dn;
                n += num_leaps * dn;
                bonus += num_leaps * dh;
                assert!(n <= goal);
                if n == goal {
                    return grid.tower_height() + bonus;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
>>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 3068);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 1514285714288);
    }
}
