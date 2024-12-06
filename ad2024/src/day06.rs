// Part 2 rank 880.

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<Vec<char>>;

#[aoc_generator(day6, part1, jorendorff)]
#[aoc_generator(day6, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines(any_char*));
    Ok(p.parse(text)?)
}

fn take_pos(maze: &mut Input) -> (usize, usize) {
    for (r, row) in maze.iter_mut().enumerate() {
        for (c, cell) in row.iter_mut().enumerate() {
            if *cell == '^' {
                *cell = '.';
                return (r, c);
            }
        }
    }
    panic!("could not find '^' in input");
}

fn in_maze(maze: &Input, pos: (usize, usize)) -> bool {
    pos.0 < maze.len() && pos.1 < maze[0].len()
}

#[aoc(day6, part1, jorendorff)]
fn part_1(input: &Input) -> usize {
    let mut maze = input.clone();
    let mut pos = take_pos(&mut maze);
    maze[pos.0][pos.1] = 'X';
    let mut count = 1;

    let mut dr = -1_isize;
    let mut dc = 0_isize;

    loop {
        let next = (
            pos.0.wrapping_add(dr as usize),
            pos.1.wrapping_add(dc as usize),
        );
        if !in_maze(&maze, next) {
            break;
        }
        match maze[next.0][next.1] {
            '#' => {
                (dr, dc) = (dc, -dr);
                continue;
            }
            '.' => {
                maze[next.0][next.1] = 'X';
                count += 1;
            }
            'X' => {}
            _ => panic!(),
        }
        pos = next;
    }

    count
}

fn can_block_at(
    maze: &Input,
    start: (usize, usize),
    start_dr: isize,
    start_dc: isize,
    block: (usize, usize),
) -> bool {
    let mut maze = maze.clone();
    maze[block.0][block.1] = '#';
    let (mut dr, mut dc) = (start_dc, -start_dr);

    use std::collections::HashSet;

    let mut seen = HashSet::new();
    
    let mut pos = start;
    loop {
        let next = (
            pos.0.wrapping_add(dr as usize),
            pos.1.wrapping_add(dc as usize),
        );
        if !in_maze(&maze, next) {
            return false;
        }
        if seen.contains(&(next, dr, dc)) {
            return true;
        }
        seen.insert((next, dr, dc));
        match maze[next.0][next.1] {
            '#' => {
                (dr, dc) = (dc, -dr);
                continue;
            }
            '.' => {}
            'X' => {}
            _ => panic!(),
        }
        pos = next;
    }
}

#[aoc(day6, part2, jorendorff)]
fn part_2(input: &Input) -> usize {
    let mut maze = input.clone();
    let mut pos = take_pos(&mut maze);
    maze[pos.0][pos.1] = 'X';
    let mut count = 0;

    let mut dr = -1_isize;
    let mut dc = 0_isize;

    loop {
        let next = (
            pos.0.wrapping_add(dr as usize),
            pos.1.wrapping_add(dc as usize),
        );
        if !in_maze(&maze, next) {
            break;
        }
        match maze[next.0][next.1] {
            '#' => {
                (dr, dc) = (dc, -dr);
                continue;
            }
            '.' => {
                if can_block_at(&maze, pos, dr, dc, next) {
                    count += 1;
                }
                maze[next.0][next.1] = 'X';
            }
            'X' => {}
            _ => panic!(),
        }
        pos = next;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 41);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 6);
    }
}
