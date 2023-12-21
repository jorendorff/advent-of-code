use adlib::*;
use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;
use std::collections::BinaryHeap;

type Input = Grid<usize>;

#[aoc_generator(day21, part1, jorendorff)]
#[aoc_generator(day21, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines(
        char_of(".#S")+
    ));
    Ok(Grid {
        data: p.parse(text)?,
    })
}

fn solve_1(grid: &Input, count: usize) -> usize {
    let mut grid = grid.clone();
    for _ in 0..count {
        for r in 0..grid.num_rows() {
            for c in 0..grid.num_cols() {
                let p = Point { row: r, col: c };
                if grid[p] == 0 {
                    for d in [Right, Left, Up, Down] {
                        let p2 = p + d;
                        if grid.has(p2) && grid[p2] == 2 {
                            grid[p] = 3;
                        }
                    }
                }
            }
        }

        for r in 0..grid.num_rows() {
            for c in 0..grid.num_cols() {
                let cell = &mut grid[Point { row: r, col: c }];
                let v = *cell;
                *cell = match v {
                    0 => 0,
                    1 => 1,
                    2 => 0,
                    3 => 2,
                    _ => panic!(),
                };
            }
        }
    }
    grid.data.into_iter().flatten().filter(|c| *c == 2).count()
}

#[aoc(day21, part1, jorendorff)]
fn part_1(input: &Input) -> usize {
    solve_1(input, 64)
}

fn clear(grid: &mut Input) -> Point {
    let mut start: Option<Point> = None;
    for r in 0..grid.num_rows() {
        for c in 0..grid.num_cols() {
            let p = Point { row: r, col: c };
            if grid[p] == 2 {
                grid[p] = 0;
                start = Some(p);
            }
        }
    }
    start.unwrap()
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Task {
    distance: u64,
    point: Point,
}

impl std::cmp::Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl std::cmp::PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn schedule(grid: &Input, mut start_points: Vec<(Point, u64)>) -> (Grid<u64>, Vec<u64>) {
    let mut seen = Grid {
        data: vec![vec![u64::MAX; grid.num_cols()]; grid.num_rows()],
    };

    let mut todo = BinaryHeap::new();
    let mut counts = vec![0, 0];
    start_points.sort_by_key(|pair| pair.1);
    for (start_point, distance) in start_points {
        seen[start_point] = distance;
        todo.push(Task {
            distance,
            point: start_point,
        });

        while distance >= counts.len() as u64 {
            counts.push(counts[counts.len() - 2]);
        }
        counts[distance as usize] += 1;
    }

    while let Some(Task { distance, point: p }) = todo.pop() {
        for d in [Right, Left, Up, Down] {
            let p2 = p + d;
            if grid.has(p2) && grid[p2] == 0 && seen[p2] == u64::MAX {
                seen[p2] = distance + 1;
                while distance + 1 >= counts.len() as u64 {
                    counts.push(counts[counts.len() - 2]);
                }
                for d in ((distance as usize + 1)..counts.len()).step_by(2) {
                    counts[d] += 1;
                }
                todo.push(Task {
                    point: p2,
                    distance: distance + 1,
                });
            }
        }
    }

    // println!("counts:{counts:?}");
    // for row in &seen.data {
    //     for &col in row {
    //         if col == u64::MAX {
    //             print!(" ##");
    //         } else {
    //             print!("{col:3}");
    //         }
    //     }
    //     println!();
    // }

    (seen, counts)
}

fn count_quadrant(counts: Vec<u64>, grid_size: u64, mut steps: u64) -> u64 {
    //println!("searching quadrant, {steps} steps to go");
    let mut total = 0;
    let mut copies = 1;
    loop {
        //println!("{steps} steps to go");
        let steps_here = {
            let mut most_possible = counts.len() as u64 - 1;
            if most_possible & 1 != steps & 1 {
                most_possible -= 1;
            }
            steps.min(most_possible)
        };
        //println!("counted {copies} copies of a map of {} reachable points", counts[steps_here as usize]);
        total += copies * counts[steps_here as usize];
        if grid_size > steps {
            break;
        }
        steps -= grid_size;
        copies += 1;
    }
    total
}

fn steps_here(counts: &[u64], steps: u64) -> usize {
    let mut most_possible = counts.len() as u64 - 1;
    if most_possible & 1 != steps & 1 {
        most_possible -= 1;
    }
    steps.min(most_possible).try_into().unwrap()
}

fn far_edge<T>(grid: &Grid<T>, dir: Dir) -> Box<dyn Iterator<Item = Point>> {
    let nr = grid.num_rows();
    let nc = grid.num_cols();
    match dir {
        Up => Box::new((0..grid.num_cols()).map(|c| Point { row: 0, col: c })),
        Down => Box::new((0..grid.num_cols()).map(move |c| Point {
            row: nr - 1,
            col: c,
        })),
        Left => Box::new((0..grid.num_rows()).map(|r| Point { row: r, col: 0 })),
        Right => Box::new((0..grid.num_rows()).map(move |r| Point {
            row: r,
            col: nc - 1,
        })),
    }
}

fn count_cardinal_strip(grid: &Input, distance_grid: &Grid<u64>, dir: Dir, mut steps: u64) -> u64 {
    // count the initial part of the strip where we might arrive at a non-corner point first
    // then go fast, with two cases, one for each corner in that direction
    // TODO
    let grid_size = match dir {
        Left | Right => distance_grid.num_cols() as u64,
        _ => distance_grid.num_rows() as u64,
    };

    let mut total = 0;

    let mut distance_grid = distance_grid.clone();

    let mut counts: Vec<u64>;
    let mut adjust = 0;
    // let mut map_count = 0;
    loop {
        let prev_distance_grid = distance_grid;

        let start_points = far_edge(grid, dir)
            .zip(far_edge(grid, dir.reverse()))
            .map(|(far_point, near_point)| -> (Point, u64) {
                (near_point, prev_distance_grid[far_point] + 1 - adjust)
            })
            .collect();
        // map_count += 1;
        (distance_grid, counts) = schedule(grid, start_points);
        if distance_grid == prev_distance_grid {
            break;
        }
        let n = counts[steps_here(&counts, steps)];
        //println!("counting {n} places in a map {map_count} {dir:?} of starting map");

        total += n;
        let min_steps_to_next_map =
            far_edge(grid, dir).map(|p| distance_grid[p]).min().unwrap() + 1;
        if min_steps_to_next_map > steps {
            return total;
        }
        steps -= min_steps_to_next_map;
        adjust = min_steps_to_next_map;
    }

    loop {
        let n = counts[steps_here(&counts, steps)];
        // map_count += 1;
        //println!("in a map {map_count} {dir:?} of starting map, with {steps} steps to go, counting {n} places");
        total += n;
        if grid_size > steps {
            break;
        }
        steps -= grid_size;
    }
    
    total
}

// The first schedule gives the distance from S to each corner; let D be the distance to the NW corner.
// The distance, then, to the nearest point on a map that is Y maps north and X maps west of the starting map
// is D + X*W + 1 + Y*H + 1.

fn solve_2(grid: &Input, steps: u64) -> u64 {
    println!();
    println!("solve_2: {steps} steps");

    let mut grid = grid.clone();
    let nr = grid.num_rows();
    let nc = grid.num_cols();
    let start = clear(&mut grid);
    let (tgrid, counts) = schedule(&grid, vec![(start, 0)]);

    let nw_point = Point { row: 0, col: 0 };
    let ne_point = Point {
        row: 0,
        col: nc - 1,
    };
    let sw_point = Point {
        row: nr - 1,
        col: 0,
    };
    let se_point = Point {
        row: nr - 1,
        col: nc - 1,
    };

    // Count reachable points within the starting map.
    let mut count = counts[steps_here(&counts, steps)];
    //println!("found {count} reachable points within the starting map");

    // Count all points in maps that are both north-or-south and east-or-west of the starting map.
    assert_eq!(nr, nc); // count_quardant requires square grid
    let grid_size: u64 = nr.try_into().unwrap();
    if tgrid[nw_point] + 2 <= steps {
        let (_, se_counts) = schedule(&grid, vec![(se_point, 0)]);
        //println!("distance to NW point: {}", tgrid[nw_point]);
        let n = count_quadrant(se_counts, grid_size, steps - tgrid[nw_point] - 2);
        //println!("found {n} reachable points in NW quadrant");
        count += n;
    }
    if tgrid[ne_point] + 2 <= steps {
        let (_, sw_counts) = schedule(&grid, vec![(sw_point, 0)]);
        count += count_quadrant(sw_counts, grid_size, steps - tgrid[ne_point] - 2);
    }
    if tgrid[sw_point] + 2 <= steps {
        let (_, ne_counts) = schedule(&grid, vec![(ne_point, 0)]);
        count += count_quadrant(ne_counts, grid_size, steps - tgrid[sw_point] - 2);
    }
    if tgrid[se_point] + 2 <= steps {
        let (_, nw_counts) = schedule(&grid, vec![(nw_point, 0)]);
        count += count_quadrant(nw_counts, grid_size, steps - tgrid[se_point] - 2);
    }

    // Count points in the strip of maps that line in each cardinal direction from the starting map.
    for dir in [Up, Down, Left, Right] {
        let found = count_cardinal_strip(&grid, &tgrid, dir, steps);
        //println!("found {found} places in direction {dir:?}");
        count += found;
    }

    count
}

#[aoc(day21, part2, jorendorff)]
fn part_2(input: &Input) -> u64 {
    solve_2(input, 26501365)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SILLY_EXAMPLE: &str = "\
...
.S.
...
";

    const SILLY_EXAMPLE_2: &str = "\
....
.S..
....
....
";

    const SILLY_EXAMPLE_3: &str = "\
....
.S..
..#.
....
";

    const SMALL_EXAMPLE: &str = "\
.....
.###.
..S..
.....
.....
";

    
    const EXAMPLE: &str = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

    #[test]
    fn test_part_1() {
        assert_eq!(solve_1(&parse_input(EXAMPLE).unwrap(), 6), 16);
    }

    #[test]
    fn test_part_2_good() {
        let grid = parse_input(SILLY_EXAMPLE).unwrap();
        assert_eq!(solve_2(&grid, 1), 4);
        assert_eq!(solve_2(&grid, 2), 9);
        assert_eq!(solve_2(&grid, 3), 16);
        assert_eq!(solve_2(&grid, 4), 25);
        assert_eq!(solve_2(&grid, 5), 36);
        assert_eq!(solve_2(&grid, 6), 49);
        assert_eq!(solve_2(&grid, 7), 64);
        assert_eq!(solve_2(&grid, 100), 10201);
        
        let grid = parse_input(SILLY_EXAMPLE_2).unwrap();
        assert_eq!(solve_2(&grid, 1), 4);
        assert_eq!(solve_2(&grid, 2), 9);
        assert_eq!(solve_2(&grid, 3), 16);
        assert_eq!(solve_2(&grid, 4), 25);
        assert_eq!(solve_2(&grid, 5), 36);
        assert_eq!(solve_2(&grid, 6), 49);
        assert_eq!(solve_2(&grid, 7), 64);
        assert_eq!(solve_2(&grid, 100), 10201);
        
        let grid = parse_input(SILLY_EXAMPLE_3).unwrap();
        assert_eq!(solve_2(&grid, 1), 4);
        assert_eq!(solve_2(&grid, 2), 8);
        assert_eq!(solve_2(&grid, 3), 16);
        assert_eq!(solve_2(&grid, 5), 36);
        assert_eq!(solve_2(&grid, 7), 64);

        let grid = parse_input(SMALL_EXAMPLE).unwrap();
        assert_eq!(solve_2(&grid, 1), 3);
        assert_eq!(solve_2(&grid, 2), 6);
        assert_eq!(solve_2(&grid, 3), 12);
        assert_eq!(solve_2(&grid, 4), 18);
        assert_eq!(solve_2(&grid, 5), 26);
        assert_eq!(solve_2(&grid, 6), 18+19);
    }

    #[test]
    fn test_part_2() {        
        let grid = parse_input(EXAMPLE).unwrap();
        assert_eq!(solve_2(&grid, 6), 16);
        assert_eq!(solve_2(&grid, 10), 50);
        assert_eq!(solve_2(&grid, 50), 1594);
        assert_eq!(solve_2(&grid, 100), 6536);
        assert_eq!(solve_2(&grid, 500), 167004);
        assert_eq!(solve_2(&grid, 1000), 668697);
        assert_eq!(solve_2(&grid, 5000), 16733044);
    }
}
