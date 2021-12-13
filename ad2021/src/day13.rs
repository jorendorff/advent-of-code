use aoc_runner_derive::*;
use itertools::Itertools;

#[derive(Copy, Clone)]
enum Insn {
    FoldX(i32),
    FoldY(i32),
}

#[aoc_generator(day13, part1, jorendorff)]
#[aoc_generator(day13, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<(Vec<(i32, i32)>, Vec<Insn>)> {
    let sections: Vec<&str> = text.split("\n\n").collect();

    anyhow::ensure!(sections.len() == 2);

    let points = sections[0]
        .lines()
        .map(|line| {
            let coords: Vec<&str> = line.split(',').collect();
            anyhow::ensure!(coords.len() == 2, "bad point {:?}", line);
            Ok((coords[0].parse::<i32>()?, coords[1].parse::<i32>()?))
        })
        .collect::<anyhow::Result<Vec<(i32, i32)>>>()?;

    let insns = sections[1]
        .lines()
        .map(|line| {
            anyhow::ensure!(line.starts_with("fold along "));
            let parts: Vec<&str> = line.split('=').collect();
            anyhow::ensure!(parts.len() == 2);
            let coord = parts[1].parse::<i32>()?;
            if parts[0].ends_with('x') {
                Ok(Insn::FoldX(coord))
            } else {
                Ok(Insn::FoldY(coord))
            }
        })
        .collect::<anyhow::Result<Vec<Insn>>>()?;

    Ok((points, insns))
}

fn fold<'a>(points: &'a [(i32, i32)], insns: &'a [Insn]) -> impl Iterator<Item = (i32, i32)> + 'a {
    points.iter().cloned().map(|p| {
        insns.iter().fold(p, |(x, y), &insn| match insn {
            Insn::FoldX(fx) => (if x > fx { 2 * fx - x } else { x }, y),
            Insn::FoldY(fy) => (x, if y > fy { 2 * fy - y } else { y }),
        })
    })
}

#[aoc(day13, part1, jorendorff)]
fn part_1((points, insns): &(Vec<(i32, i32)>, Vec<Insn>)) -> usize {
    fold(points, &insns[..1]).unique().count()
}

#[aoc(day13, part2, jorendorff)]
fn part_2((points, insns): &(Vec<(i32, i32)>, Vec<Insn>)) -> String {
    let points: Vec<(i32, i32)> = fold(points, insns).collect();
    let xmax = points.iter().map(|&(x, _y)| x).max().unwrap();
    let ymax = points.iter().map(|&(_x, y)| y).max().unwrap();

    let mut grid = vec![vec!['.'; xmax as usize + 1]; ymax as usize + 1];
    for (x, y) in points {
        grid[y as usize][x as usize] = '#';
    }
    let mut out = "\n".to_string();
    for row in grid {
        let line: String = row.into_iter().collect();
        out += &line;
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 17);
    }

    #[test]
    fn test_part_2() {
        let expected = "
#####
#...#
#...#
#...#
#####
";
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), expected);
    }
}
