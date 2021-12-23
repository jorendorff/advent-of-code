use std::{
    cmp::Reverse,
    collections::{hash_map::Entry, BinaryHeap, HashMap},
    fmt::{self, Debug},
};

use aoc_runner_derive::*;
use regex::Regex;

#[derive(Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(u8)]
enum Cell {
    Empty,
    A,
    B,
    C,
    D,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
struct State<const SIZE: usize> {
    rooms: [[Cell; SIZE]; 4],
    hall: [Cell; 7],
}

impl Cell {
    fn is_empty(&self) -> bool {
        *self == Cell::Empty
    }

    fn code(&self) -> usize {
        *self as u8 as usize
    }
}

impl Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Cell::Empty => '.',
                Cell::A => 'A',
                Cell::B => 'B',
                Cell::C => 'C',
                Cell::D => 'D',
            }
        )
    }
}

impl<const SIZE: usize> Debug for State<SIZE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "#############")?;
        writeln!(
            f,
            "#{:?}{:?}.{:?}.{:?}.{:?}.{:?}{:?}#",
            self.hall[0],
            self.hall[1],
            self.hall[2],
            self.hall[3],
            self.hall[4],
            self.hall[5],
            self.hall[6],
        )?;
        let n = SIZE - 1;
        writeln!(
            f,
            "###{:?} {:?} {:?} {:?}###",
            self.rooms[0][n], self.rooms[1][n], self.rooms[2][n], self.rooms[3][n],
        )?;
        for i in (0..n).rev() {
            writeln!(
                f,
                "  #{:?} {:?} {:?} {:?}#",
                self.rooms[0][i], self.rooms[1][i], self.rooms[2][i], self.rooms[3][i],
            )?;
        }
        writeln!(f, "  #########")
    }
}

impl<const SIZE: usize> State<SIZE> {
    fn done(&self) -> bool {
        self.rooms
            == [
                [Cell::A; SIZE],
                [Cell::B; SIZE],
                [Cell::C; SIZE],
                [Cell::D; SIZE],
            ]
    }

    fn ready(&self, room: usize) -> bool {
        let who = match room {
            0 => Cell::A,
            1 => Cell::B,
            2 => Cell::C,
            3 => Cell::D,
            _ => unreachable!(),
        };
        self.rooms[room]
            .iter()
            .all(|cell| cell.is_empty() || *cell == who)
    }

    fn move_out(&self, room_index: usize, y: usize, out: &mut Vec<(u64, State<SIZE>)>) {
        let cost_per_step = 10u64.pow(self.rooms[room_index][y].code() as u32 - 1);
        let first_hall_index_to_right = room_index + 2;
        for hall_index in 0..7 {
            let cells_between = if hall_index < first_hall_index_to_right {
                hall_index..first_hall_index_to_right
            } else {
                first_hall_index_to_right..hall_index + 1
            };
            let can_move_out = cells_between.clone().all(|h| self.hall[h].is_empty());
            if can_move_out {
                let mut result = *self;
                std::mem::swap(
                    &mut result.rooms[room_index][y],
                    &mut result.hall[hall_index],
                );
                let nsteps_horiz = ([0i32, 1, 3, 5, 7, 9, 10][hall_index]
                    - [2i32, 4, 6, 8][room_index])
                    .abs() as u64;
                let nsteps_vert = (SIZE - y) as u64;
                let nsteps = nsteps_horiz + nsteps_vert;
                out.push((cost_per_step * nsteps, result));
            }
        }
    }

    fn move_in(&self, hall_index: usize, room_index: usize) -> (u64, Self) {
        let cost_per_step = 10u64.pow(self.hall[hall_index].code() as u32 - 1);
        // find downmost empty y
        let mut y = 0;
        while y < SIZE && !self.rooms[room_index][y].is_empty() {
            y += 1;
        }

        let mut result = *self;
        std::mem::swap(
            &mut result.rooms[room_index][y],
            &mut result.hall[hall_index],
        );
        let nsteps_horiz =
            ([0i32, 1, 3, 5, 7, 9, 10][hall_index] - [2i32, 4, 6, 8][room_index]).abs() as u64;
        let nsteps_vert = (SIZE - y) as u64;
        let nsteps = nsteps_horiz + nsteps_vert;
        (cost_per_step * nsteps, result)
    }

    // #############
    // #01.2.3.4.56#
    // ###1#2#3#4###
    //   # # # # #
    //   #########

    fn successors(&self) -> Vec<(u64, State<SIZE>)> {
        // it can't be useful for an amphipod in its final position to move, so
        // ignore that possibility.
        // 1. moving out
        let mut out = vec![];
        for i in 0..4 {
            let mut y = SIZE - 1;
            while y > 0 && self.rooms[i][y].is_empty() {
                y -= 1;
            }
            if !self.rooms[i][y].is_empty() {
                self.move_out(i, y, &mut out);
            }
        }

        // 2. moving right and in
        for x in 0..5 {
            if !self.hall[x].is_empty() {
                let destination = self.hall[x].code() - 1;
                if x < destination + 2
                    && self.ready(destination)
                    && (x + 1..destination + 2).all(|i| self.hall[i].is_empty())
                {
                    out.push(self.move_in(x, destination));
                }
            }
        }

        // 3. moving left and in
        for x in 2..7 {
            if !self.hall[x].is_empty() {
                let destination = self.hall[x].code() - 1;
                if x >= destination + 2
                    && self.ready(destination)
                    && (destination + 2..x).all(|i| self.hall[i].is_empty())
                {
                    out.push(self.move_in(x, destination));
                }
            }
        }
        out
    }
}

#[aoc_generator(day23, part1, jorendorff)]
#[aoc_generator(day23, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<State<2>> {
    let re = Regex::new(
        r"#############
#\.\.\.\.\.\.\.\.\.\.\.#
###([A-D])#([A-D])#([A-D])#([A-D])###
  #([A-D])#([A-D])#([A-D])#([A-D])#
  #########
?",
    )
    .unwrap();
    let cap = re
        .captures(text)
        .ok_or_else(|| anyhow::anyhow!("no match for {:?}", text))?;
    fn f(s: &str) -> Cell {
        match s.as_bytes()[0] {
            b'A' => Cell::A,
            b'B' => Cell::B,
            b'C' => Cell::C,
            b'D' => Cell::D,
            _ => unreachable!("re shouldn't have matched"),
        }
    }
    Ok(State {
        hall: [Cell::Empty; 7],
        rooms: [
            [f(&cap[5]), f(&cap[1])],
            [f(&cap[6]), f(&cap[2])],
            [f(&cap[7]), f(&cap[3])],
            [f(&cap[8]), f(&cap[4])],
        ],
    })
}

fn solve<const SIZE: usize>(start: &State<SIZE>) -> u64 {
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0u64), *start));
    let mut cache: HashMap<State<SIZE>, (u64, Option<State<SIZE>>)> = HashMap::new();
    cache.insert(*start, (0, None));
    while let Some((Reverse(spent), state)) = queue.pop() {
        if state.done() {
            //println!("FOUND IT");
            //let mut current = state;
            //let mut log = vec![];
            //while let Some((spent, prev)) = cache.get(&current) {
            //    log.push((spent, current));
            //    current = match *prev {
            //        Some(prev) => prev,
            //        None => break,
            //    };
            //}
            //log.reverse();
            //for (spent, state) in log {
            //    println!("{}\n{:?}\n", spent, state);
            //}
            return spent;
        }
        for (cost, result) in state.successors() {
            let new_spent = spent + cost;
            match cache.entry(result) {
                Entry::Occupied(mut e) => {
                    if e.get().0 > new_spent {
                        *e.get_mut() = (new_spent, Some(state));
                        queue.push((Reverse(new_spent), result));
                    }
                }
                Entry::Vacant(e) => {
                    e.insert((new_spent, Some(state)));
                    queue.push((Reverse(spent + cost), result));
                }
            }
        }
    }
    panic!("no solutions");
}

#[aoc(day23, part1, jorendorff)]
fn part_1(start: &State<2>) -> u64 {
    solve(start)
}

#[aoc(day23, part2, jorendorff)]
fn part_2(start: &State<2>) -> u64 {
    let r = &start.rooms;
    let start = State {
        hall: start.hall,
        rooms: [
            [r[0][0], Cell::D, Cell::D, r[0][1]],
            [r[1][0], Cell::B, Cell::C, r[1][1]],
            [r[2][0], Cell::A, Cell::B, r[2][1]],
            [r[3][0], Cell::C, Cell::A, r[3][1]],
        ],
    };
    solve(&start)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
";

    #[test]
    fn test_part_1() {
        assert_eq!(
            parse_input(EXAMPLE).unwrap(),
            State {
                hall: [Cell::Empty; 7],
                rooms: [
                    [Cell::A, Cell::B],
                    [Cell::D, Cell::C],
                    [Cell::C, Cell::B],
                    [Cell::A, Cell::D],
                ],
            }
        );
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 12521);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 44169);
    }
}
