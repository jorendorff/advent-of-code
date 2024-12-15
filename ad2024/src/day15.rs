// Part 2 rank 316

use std::collections::*;

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = (Vec<Vec<char>>, Vec<(i64, i64)>);

#[aoc_generator(day15, part1, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(
        section(lines(any_char+))
            section(lines:lines({
                '^' => (-1, 0),
                '<' => (0, -1),
                '>' => (0, 1),
                'v' => (1, 0),
            }+) => lines.into_iter().flatten().collect())
    );
    Ok(p.parse(text)?)
}

#[aoc_generator(day15, part2, jorendorff)]
fn parse_input_2(text: &str) -> anyhow::Result<Input> {
    let p = parser!(
        section(lines(
            strs:{
                '#' => "##",
                'O' => "[]",
                '.' => "..",
                '@' => "@.",
            }+ => strs.into_iter().flat_map(|s| s.chars()).collect::<Vec<char>>()
        ))
            section(lines:lines({
                '^' => (-1, 0),
                '<' => (0, -1),
                '>' => (0, 1),
                'v' => (1, 0),
            }+) => lines.into_iter().flatten().collect())
    );
    Ok(p.parse(text)?)
}

#[aoc(day15, part1, jorendorff)]
fn part_1(input: &Input) -> u64 {
    let (mut grid, moves) = input.clone();

    let mut r_bot = 0;
    let mut c_bot = 0;
    'search:
    for (r, row) in grid.iter_mut().enumerate() {
        for (c, cell) in row.iter_mut().enumerate() {
            if *cell == '@' {
                r_bot = r;
                c_bot = c;
                *cell = '.';
                break 'search;
            }
        }
    }
    
    for (dr, dc) in moves {
        let r_next = (r_bot as i64 + dr) as usize;
        let c_next = (c_bot as i64 + dc) as usize;
        let mut r_blank = r_next;
        let mut c_blank = c_next;
        while grid[r_blank][c_blank] == 'O' {
            r_blank = (r_blank as i64 + dr) as usize;
            c_blank = (c_blank as i64 + dc) as usize;
        }
        if grid[r_blank][c_blank] == '.' {
            grid[r_blank][c_blank] = grid[r_next][c_next];
            grid[r_next][c_next] = '.';
            r_bot = r_next;
            c_bot = c_next;
        }

        grid[r_bot][c_bot] = '@';
        println!("Move {:?}:", (dr, dc));
        for row in &grid {
            println!("{}", row.iter().copied().collect::<String>());
        }
        println!();
        grid[r_bot][c_bot] = '.';
    }
    
    grid.into_iter()
        .enumerate()
        .map(|(r, row)| -> u64{
            row.into_iter()
                .enumerate()
                .map(|(c, ch)| {
                    if ch == 'O' {
                        100 * r as u64 + c as u64
                    } else {
                        0
                    }
                })
                .sum()
        })
        .sum()
}

#[aoc(day15, part2, jorendorff)]
fn part_2(input: &Input) -> u64 {
    let (mut grid, moves) = input.clone();

    let mut r_bot = 0;
    let mut c_bot = 0;
    'search:
    for (r, row) in grid.iter_mut().enumerate() {
        for (c, cell) in row.iter_mut().enumerate() {
            if *cell == '@' {
                r_bot = r;
                c_bot = c;
                break 'search;
            }
        }
    }
    
    for (dr, dc) in moves {
        let r_next = (r_bot as i64 + dr) as usize;
        let c_next = (c_bot as i64 + dc) as usize;

        fn enqueue(
            grid: &[Vec<char>],
            junk: &mut HashSet<(char, usize, usize)>,
            todo: &mut VecDeque<(usize, usize)>,
            r: usize,
            c: usize,
        ) {
            let triple = (grid[r][c], r, c);
            if !junk.contains(&triple) {
                junk.insert(triple);
                todo.push_back((r, c));
            }
        }

        let mut junk = HashSet::new();
        let mut todo = VecDeque::new();
        enqueue(&grid, &mut junk, &mut todo, r_bot, c_bot);

        let mut blocked = false;
        while let Some((r, c)) = todo.pop_front() {
            match grid[r][c] {
                '[' => enqueue(&grid, &mut junk, &mut todo, r, c + 1),
                ']' => enqueue(&grid, &mut junk, &mut todo, r, c - 1),
                _ => {}
            }
            let r1 = (r as i64 + dr) as usize;
            let c1 = (c as i64 + dc) as usize;
            match grid[r1][c1] {
                '#' => {
                    blocked = true;
                    break;
                }
                '.' => {}
                _ => enqueue(&grid, &mut junk, &mut todo, r1, c1),
            }
        }

        if !blocked {
            for (_, r, c) in junk.iter().copied() {
                grid[r][c] = '.';
            }
            for (ch, r, c) in junk.iter().copied() {
                grid[(r as i64 + dr) as usize][(c as i64 + dc) as usize] = ch;
            }
            r_bot = (r_bot as i64 + dr) as usize;
            c_bot = (c_bot as i64 + dc) as usize;
        }


        println!("Move {:?}:", (dr, dc));
        for row in &grid {
            println!("{}", row.iter().copied().collect::<String>());
        }
        println!();
    }
    
    grid.into_iter()
        .enumerate()
        .map(|(r, row)| -> u64{
            row.into_iter()
                .enumerate()
                .map(|(c, ch)| {
                    if ch == '[' {
                        100 * r as u64 + c as u64
                    } else {
                        0
                    }
                })
                .sum()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_SMALL: &str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";
    
    const EXAMPLE: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";


    
    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE_SMALL).unwrap()), 2028);
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 10092);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input_2(EXAMPLE).unwrap()), 9021);
    }
}
