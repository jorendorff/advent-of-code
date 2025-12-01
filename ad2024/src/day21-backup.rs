use pathfinding::directed::astar;

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<String>;

#[aoc_generator(day21, part1, jorendorff)]
#[aoc_generator(day21, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines(string(any_char+)));
    Ok(p.parse(text)?)
}

// up down left right A

// to move to and push the following button:
// up down left right A
// takes this many mashes:
// 2 3 4 2 1

// pushing any button again is just 1

// to get the second robot to move to and push:
//   up down left right A
// takes this many mashes:
//   8 9 10 6 1

// to get the third robot to move to and push:
//   0 1 2 3 4 5 6 7 8 9
// takes:
//

fn coords(ch: char) -> (i32, i32) {
    match ch {
        '0' => (3, 1),
        '1' => (2, 0),
        '2' => (2, 1),
        '3' => (2, 2),
        '4' => (1, 0),
        '5' => (1, 1),
        '6' => (1, 2),
        '7' => (0, 0),
        '8' => (0, 1),
        '9' => (0, 2),
        'A' => (3, 2),
        _ => panic!("unrecognized char in code: {ch:?}"),
    }
}

fn robot_1(code: &str) -> u64 {
    let mut count = 0;
    let mut pos = coords('A');
    for ch in code.chars() {
        let next = coords(ch);
        count += robot_2_move_me_and_push(next.0 - pos.0, next.1 - pos.1, pos.0, pos.1);
        pos = next;
    }
    count
}

fn robot_2_move_me_and_push(mut dr: i32, mut dc: i32, r: i32, c: i32) -> u64 {
    let mut count = 0;
    let mut pos = (0, 2);
    if dc > 0 {
        // I need to press `>`
        let next = (1, 2);
        count += robot_3_move_me_and_push(next.0 - pos.0, next.1 - pos.1);
        pos = next;
        dc -= 1;
        while dc > 0 {
            count += 1;
            dc -= 1;
        }
    } else if dc < 0 && !(r == 3 && c + dc == 0) {
        // I need to press `<`
        let next = (1, 0);
        count += robot_3_move_me_and_push(next.0 - pos.0, next.1 - pos.1);
        pos = next;
        dc += 1;
        while dc < 0 {
            count += 1;
            dc += 1;
        }
    }

    if dr < 0 {
        // I need to press `^`
        let next = (0, 1);
        count += robot_3_move_me_and_push(next.0 - pos.0, next.1 - pos.1);
        pos = next;
        dr += 1;
        while dr < 0 {
            count += 1;
            dr += 1;
        }
    } else if dr > 0 {
        // I need to press `v`
        let next = (1, 1);
        count += robot_3_move_me_and_push(next.0 - pos.0, next.1 - pos.1);
        pos = next;
        dr -= 1;
        while dr > 0 {
            count += 1;
            dr -= 1;
        }
    }
    if dc < 0 {
        // I need to press `<`
        let next = (1, 0);
        count += robot_3_move_me_and_push(next.0 - pos.0, next.1 - pos.1);
        pos = next;
        dc += 1;
        while dc < 0 {
            count += 1;
            dc += 1;
        }
    }

    // Now I need to press `A`
    let next = (0, 2);
    count += robot_3_move_me_and_push(next.0 - pos.0, next.1 - pos.1);
    count
}

fn robot_3_move_me_and_push(mut dr: i32, mut dc: i32) -> u64 {
    let odr = dr;
    let odc = dc;

    let mut count = 0;
    let mut pos = (0i32, 2i32);
    if dc > 0 {
        let next = (1, 2);
        count += pos.0.abs_diff(next.0) as u64 + pos.1.abs_diff(next.1) as u64 + 1; // v A
        pos = next;
        dc -= 1;
        while dc > 0 {
            count += 1; // A
            dc -= 1;
        }
    }
    if dr < 0 {
        let next = (0, 1);
        count += pos.0.abs_diff(next.0) as u64 + pos.1.abs_diff(next.1) as u64 + 1; // v A
        pos = next;
        dr += 1;
        while dr < 0 {
            count += 1; // A
            dr += 1;
        }
    } else if dr > 0 {
        let next = (1, 1);
        count += pos.0.abs_diff(next.0) as u64 + pos.1.abs_diff(next.1) as u64 + 1; // v A
        pos = next;
        dr -= 1;
        while dr > 0 {
            count += 1; // A
            dr -= 1;
        }
    }
    if dc < 0 {
        let next = (1, 0);
        count += pos.0.abs_diff(next.0) as u64 + pos.1.abs_diff(next.1) as u64 + 1; // v A
        pos = next;
        dc += 1;
        while dc < 0 {
            count += 1; // A
            dc += 1;
        }
    }

    // Now I need to press `A`
    let next = (0, 2);
    count += pos.0.abs_diff(next.0) as u64 + pos.1.abs_diff(next.1) as u64 + 1; // > > ^ A

    println!("robot_3_mmap({odr}, {odc}) ==> {count}");
    count
}

fn move_on_numeric_keypad(pos: u8, dir: u8) -> Option<u8> {
    let out = [
        [2, 99, 99, 10], // 0
        [4, 99, 99, 2],  // 1
        [5, 0, 1, 3],    // 2
        [6, 10, 2, 99],  // 3
        [7, 1, 99, 5],   // 4
        [8, 2, 4, 6],    // 5
        [9, 3, 5, 99],   // 6
        [99, 4, 99, 8],  // 7
        [99, 5, 7, 9],   // 8
        [99, 6, 8, 99],  // 9
        [3, 99, 0, 99],  // A
    ][pos as usize][dir as usize];
    match out {
        99 => None,
        n => Some(n),
    }
}

fn min_len(code: &str) -> u64 {
    println!();
    robot_1(code)
}

#[aoc(day21, part1, jorendorff)]
fn part_1(input: &Input) -> i32 {
    input
        .iter()
        .map(|code| min_len(code) as i32 * code[..code.len() - 1].parse::<i32>().unwrap())
        .sum()
}

#[aoc(day21, part2, jorendorff)]
fn part_2(input: &Input) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
029A
980A
179A
456A
379A
";

    #[test]
    fn test_part_1() {
        assert_eq!(
            robot_2_move_me_and_push(-2, -2, 2, 2),
            "<vA<AA>>^AAvA<^A>AAvA^A".len() as u64
        );
        // my schedule:   robot 2 goes ^^<<A, robot 3 goes <AAv<AA>>^A,
        // best schedule: robot 2 goes <<^^A, robot 3 goes v<<AA>^AA>A, I type <vA<AA>>^AAvA<^A>AAvA^A

        assert_eq!(
            robot_2_move_me_and_push(-1, 0, 3, 2),
            "<v<A>>^AvA^A".len() as u64
        );
        assert_eq!(
            robot_2_move_me_and_push(0, 2, 0, 0),
            "<vA>^AA<A".len() as u64
        );
        assert_eq!(
            robot_2_move_me_and_push(3, 0, 0, 2),
            ">A<v<A>A>^AAAvA<^A>A".len() as u64
        );

        assert_eq!(min_len("379A"), 64);
        assert_eq!(min_len("029A"), 68);
        assert_eq!(min_len("980A"), 60);
        assert_eq!(min_len("179A"), 68);
        assert_eq!(min_len("456A"), 64);
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 126384);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 0);
    }
}
