use aoc_runner_derive::*;

type Program = Vec<bool>;

#[derive(Clone)]
struct Image {
    grid: Vec<Vec<bool>>,
    elsewhere: bool,
}

#[aoc_generator(day20, part1, jorendorff)]
#[aoc_generator(day20, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<(Vec<bool>, Image)> {
    let (program, image) = text.split_once("\n\n").unwrap();
    anyhow::ensure!(program.len() == 512);
    let program: Vec<bool> = program
        .chars()
        .map(|c| match c {
            '#' => Ok(true),
            '.' => Ok(false),
            _ => anyhow::bail!("unrecognized character {:?}", c),
        })
        .collect::<anyhow::Result<Vec<bool>>>()?;
    let grid: Vec<Vec<bool>> = image
        .lines()
        .map(|line| -> anyhow::Result<Vec<bool>> {
            line.chars()
                .map(|c| match c {
                    '#' => Ok(true),
                    '.' => Ok(false),
                    _ => anyhow::bail!("unrecognized character {:?}", c),
                })
                .collect()
        })
        .collect::<anyhow::Result<Vec<Vec<bool>>>>()?;
    Ok((
        program,
        Image {
            grid,
            elsewhere: false,
        },
    ))
}

fn count_ones(image: &Image) -> usize {
    assert_eq!(image.elsewhere, false);
    image
        .grid
        .iter()
        .flat_map(|row| row.iter().copied())
        .filter(|&x| x)
        .count()
}

fn step(program: &[bool], image: Image) -> Image {
    let old_elsewhere = image.elsewhere;
    let elsewhere = program[if old_elsewhere { 0b111_111_111 } else { 0 }];
    let w = image.grid[0].len() as isize;
    let h = image.grid.len() as isize;
    let get = move |r: isize, c: isize| -> usize {
        if (0..h).contains(&r) && (0..w).contains(&c) {
            image.grid[r as usize][c as usize] as usize
        } else {
            image.elsewhere as usize
        }
    };

    let grid = (-2..h + 2)
        .map(move |r| -> Vec<bool> {
            (-2..w + 2)
                .map(|c| {
                    let index = (get(r - 1, c - 1) << 8)
                        + (get(r - 1, c) << 7)
                        + (get(r - 1, c + 1) << 6)
                        + (get(r, c - 1) << 5)
                        + (get(r, c) << 4)
                        + (get(r, c + 1) << 3)
                        + (get(r + 1, c - 1) << 2)
                        + (get(r + 1, c) << 1)
                        + get(r + 1, c + 1);
                    program[index]
                })
                .collect()
        })
        .collect();

    Image { grid, elsewhere }
}

#[aoc(day20, part1, jorendorff)]
fn part_1(input: &(Program, Image)) -> usize {
    let program: &[bool] = &input.0;
    let mut image = input.1.clone();
    for _ in 0..2 {
        image = step(program, image);
    }
    count_ones(&image)
}

#[aoc(day20, part2, jorendorff)]
fn part_2(input: &(Program, Image)) -> usize {
    let program: &[bool] = &input.0;
    let mut image = input.1.clone();
    for _ in 0..50 {
        image = step(program, image);
    }
    count_ones(&image)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 35);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 3351);
    }
}
