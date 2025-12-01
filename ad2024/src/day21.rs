use pathfinding::directed::dijkstra;

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<String>;

#[aoc_generator(day21, part1, jorendorff)]
#[aoc_generator(day21, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines(string(any_char+)));
    Ok(p.parse(text)?)
}

enum Cmd {
    Up,
    Down,
    Left,
    Right,
    Activate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    goal: (i8, i8),
    robot1: (i8, i8),
    robot2: (i8, i8),
    robot3: (i8, i8),
    success: bool,
}

impl State {
    fn new(robot1: (i8, i8), goal: (i8, i8)) -> State {
        State {
            goal,
            robot1,
            robot2: (0, 2),
            robot3: (0, 2),
            success: false,
        }
    }

    fn punch(&mut self, cmd: Cmd) {
        match cmd {
            Cmd::Up => self.robot3_move(-1, 0),
            Cmd::Down => self.robot3_move(1, 0),
            Cmd::Left => self.robot3_move(0, -1),
            Cmd::Right => self.robot3_move(0, 1),
            Cmd::Activate => self.robot3_punch(),
        }
    }

    fn robot3_move(&mut self, dr: i8, dc: i8) {
        let (r, c) = self.robot3;
        if 0 <= r + dr && r + dr <= 1 && 0 <= c + dc && c + dc <= 2 && (r + dr, c + dc) != (0, 0) {
            self.robot3 = (r + dr, c + dc);
        }
    }

    fn robot3_punch(&mut self) {
        println!("robot3_punch");
        match self.robot3 {
            (0, 1) => self.robot2_move(-1, 0),
            (0, 2) => self.robot2_punch(),
            (1, 0) => self.robot2_move(0, -1),
            (1, 1) => self.robot2_move(1, 0),
            (1, 2) => self.robot2_move(0, 1),
            _ => unreachable!(),
        }
    }

    fn robot2_move(&mut self, dr: i8, dc: i8) {
        let (r, c) = self.robot2;
        if 0 <= r + dr && r + dr <= 1 && 0 <= c + dc && c + dc <= 2 && (r + dr, c + dc) != (0, 0) {
            self.robot2 = (r + dr, c + dc);
        }
    }

    fn robot2_punch(&mut self) {
        println!("robot2_punch");
        match self.robot2 {
            (0, 1) => self.robot1_move(-1, 0),
            (0, 2) => self.robot1_punch(),
            (1, 0) => self.robot1_move(0, -1),
            (1, 1) => self.robot1_move(1, 0),
            (1, 2) => self.robot1_move(0, 1),
            _ => unreachable!(),
        }
    }

    fn robot1_move(&mut self, dr: i8, dc: i8) {
        let (r, c) = self.robot1;
        println!("trying to move robot 1 from {r},{c} by {dr},{dc}");
        if 0 <= r + dr && r + dr <= 3 && 0 <= c + dc && c + dc <= 2 && (r + dr, c + dc) != (3, 0) {
            println!("success!");
            self.robot1 = (r + dr, c + dc);
        }
    }

    fn robot1_punch(&mut self) {
        if self.robot1 == self.goal {
            self.success = true;
        }
    }
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

fn coords(ch: char) -> (i8, i8) {
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

fn min_len(code: &str) -> u64 {
    let mut pos = coords('A');
    let mut len = 0;
    for ch in code.chars() {
        let goal = coords(ch);
        let state = State::new(pos, goal);
        let (_path, cost) = dijkstra::dijkstra(
            &state,
            move |s| {
                let s = *s;
                println!("exploring from {s:?}");
                [Cmd::Left, Cmd::Right, Cmd::Up, Cmd::Down, Cmd::Activate]
                    .into_iter()
                    .map(move |cmd| {
                        let mut ss = s;
                        ss.punch(cmd);
                        (ss, 1)
                    })
            },
            |s| s.success,
        )
        .unwrap();
        len += cost;
        pos = goal;
    }
    len
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
