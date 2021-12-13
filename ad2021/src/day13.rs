use aoc_runner_derive::*;

#[derive(Clone)]
struct Grid {
    grid: Vec<Vec<bool>>,
}

#[derive(Copy, Clone)]
enum Insn {
    FoldX(usize),
    FoldY(usize),
}

impl Grid {
    fn fold_y(&mut self, y: usize) {
        assert!(y < self.grid.len());
        let folded_rows = self.grid.split_off(y + 1);
        self.grid.pop();
        if folded_rows.len() > self.grid.len() {
            panic!();
        }

        for (erow, frow) in self.grid.iter_mut().rev().zip(folded_rows.into_iter()) {
            for (e, f) in erow.iter_mut().zip(frow) {
                *e |= f;
            }
        }
    }

    fn transpose(&mut self) {
        let w = self.grid[0].len();
        self.grid = (0..w)
            .map(|x| self.grid.iter().map(|row| row[x]).collect())
            .collect();
    }

    fn fold_x(&mut self, x: usize) {
        self.transpose();
        self.fold_y(x);
        self.transpose();
    }

    fn carry_out(&mut self, insn: Insn) {
        match insn {
            Insn::FoldX(x) => self.fold_x(x),
            Insn::FoldY(y) => self.fold_y(y),
        }
    }

    fn count_ones(&self) -> usize {
        self.grid
            .iter()
            .flat_map(|row| row.iter().filter(|cell| **cell))
            .count()
    }

    fn dump(&self) {
        for row in &self.grid {
            println!(
                "{}",
                row.iter()
                    .map(|&x| if x { '#' } else { '.' })
                    .collect::<String>()
            );
        }
    }
}

#[aoc_generator(day13, part1, jorendorff)]
#[aoc_generator(day13, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<(Grid, Vec<Insn>)> {
    let sections: Vec<&str> = text.split("\n\n").collect();

    anyhow::ensure!(sections.len() == 2);

    let points = sections[0]
        .lines()
        .map(|line| {
            let coords: Vec<&str> = line.split(',').collect();
            anyhow::ensure!(coords.len() == 2, "bad point {:?}", line);
            Ok((coords[0].parse::<usize>()?, coords[1].parse::<usize>()?))
        })
        .collect::<anyhow::Result<Vec<(usize, usize)>>>()?;

    let xmax = points.iter().map(|&(x, _y)| x).max().unwrap();
    let ymax = points.iter().map(|&(_x, y)| y).max().unwrap();

    let mut grid = vec![vec![false; xmax + 1]; ymax + 1];
    for (x, y) in points {
        grid[y][x] = true;
    }

    let insns = sections[1]
        .lines()
        .map(|line| {
            anyhow::ensure!(line.starts_with("fold along "));
            let parts: Vec<&str> = line.split('=').collect();
            anyhow::ensure!(parts.len() == 2);
            let coord = parts[1].parse::<usize>()?;
            if parts[0].ends_with('x') {
                Ok(Insn::FoldX(coord))
            } else {
                Ok(Insn::FoldY(coord))
            }
        })
        .collect::<anyhow::Result<Vec<Insn>>>()?;

    Ok((Grid { grid }, insns))
}

#[aoc(day13, part1, jorendorff)]
fn part_1((grid, insns): &(Grid, Vec<Insn>)) -> usize {
    let mut grid = grid.clone();
    grid.carry_out(insns[0]);
    grid.count_ones()
}

#[aoc(day13, part2, jorendorff)]
fn part_2((grid, insns): &(Grid, Vec<Insn>)) -> &'static str {
    let mut grid = grid.clone();
    for insn in insns {
        grid.carry_out(*insn);
    }
    grid.dump();

    "ok"
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

    //#[test]
    //fn test_part_2() {
    //    assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), ());
    //}
}
