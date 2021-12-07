use aoc_runner_derive::*;

use itertools::Itertools;

#[derive(Debug)]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Line {
    fn new((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> Self {
        Self { x1, y1, x2, y2 }
    }

    fn points(&self) -> Vec<(usize, usize)> {
        let Line { x1, y1, x2, y2 } = *self;
        let dx = (x2 as isize - x1 as isize).signum();
        let dy = (y2 as isize - y1 as isize).signum();

        let mut x = x1 as isize;
        let mut y = y1 as isize;
        let mut points = vec![(x as usize, y as usize)];
        while (x as usize, y as usize) != (x2, y2) {
            x += dx;
            y += dy;
            points.push((x as usize, y as usize));
        }
        points
    }

    fn rect_points(&self) -> Vec<(usize, usize)> {
        if self.x1 == self.x2 || self.y1 == self.y2 {
            self.points()
        } else {
            vec![]
        }
    }
}

fn parse_point(s: &str) -> anyhow::Result<(usize, usize)> {
    let nums = s.split(',').collect_vec();
    assert_eq!(nums.len(), 2);
    Ok((nums[0].parse()?, nums[1].parse()?))
}

#[aoc_generator(day5)]
fn parse_input(text: &str) -> anyhow::Result<Vec<Line>> {
    text.lines()
        .map(|line| -> anyhow::Result<Line> {
            let fields = line.split(" -> ").collect_vec();
            anyhow::ensure!(fields.len() == 2, "failed to parse line {:?}", line);
            Ok(Line::new(parse_point(fields[0])?, parse_point(fields[1])?))
        })
        .collect()
}

#[aoc(day5, part1)]
fn part_1(lines: &[Line]) -> usize {
    lines
        .iter()
        .flat_map(Line::rect_points)
        .counts()
        .into_iter()
        .filter(|(_key, count)| *count > 1)
        .count()
}

#[aoc(day5, part2)]
fn part_2(lines: &[Line]) -> usize {
    lines
        .iter()
        .flat_map(Line::points)
        .counts()
        .into_iter()
        .filter(|(_key, count)| *count > 1)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 5);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 12);
    }
}
