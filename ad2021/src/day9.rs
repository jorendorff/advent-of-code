use aoc_runner_derive::*;

#[aoc_generator(day9, part1, jorendorff)]
#[aoc_generator(day9, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Vec<Vec<u8>>> {
    text.lines()
        .map(|line| Ok(line.trim().bytes().map(|b| b - b'0').collect()))
        .collect()
}

fn neighbors(arr: &[Vec<u8>], r: usize, c: usize) -> Vec<(usize, usize)> {
    let mut pts = vec![];
    if r > 0 {
        pts.push((r - 1, c));
    }
    if r < arr.len() - 1 {
        pts.push((r + 1, c));
    }
    if c > 0 {
        pts.push((r, c - 1));
    }
    if c < arr[0].len() - 1 {
        pts.push((r, c + 1));
    }
    pts
}

fn is_low_point(arr: &[Vec<u8>], r: usize, c: usize) -> bool {
    let h = arr[r][c];
    let rmax = arr.len() - 1;
    let cmax = arr[0].len() - 1;
    (r == 0 || arr[r - 1][c] > h)
        && (r == rmax || arr[r + 1][c] > h)
        && (c == 0 || arr[r][c - 1] > h)
        && (c == cmax || arr[r][c + 1] > h)
}

#[aoc(day9, part1, jorendorff)]
fn part_1(arr: &[Vec<u8>]) -> u64 {
    let mut total = 0;
    for r in 0..arr.len() {
        for c in 0..arr[0].len() {
            if is_low_point(arr, r, c) {
                total += 1 + arr[r][c] as u64;
            }
        }
    }
    total
}

#[aoc(day9, part2, jorendorff)]
#[allow(clippy::needless_range_loop)]
fn part_2(arr: &[Vec<u8>]) -> u64 {
    let nr = arr.len();
    let nc = arr[0].len();

    let mut smoke = vec![vec![1; nc]; nr];

    for i in (1..=8).rev() {
        for r in 0..nr {
            for c in 0..nc {
                let h = arr[r][c];
                if h == i {
                    let (dr, dc) = neighbors(arr, r, c)
                        .into_iter()
                        .min_by_key(|&(nr, nc)| arr[nr][nc])
                        .unwrap();
                    if arr[dr][dc] < h {
                        // smoke settles
                        smoke[dr][dc] += smoke[r][c];
                        smoke[r][c] = 0;
                    }
                }
            }
        }
    }

    let mut basin_sizes = vec![];
    for r in 0..nr {
        for c in 0..nc {
            if smoke[r][c] != 0 && arr[r][c] != 9 {
                assert!(is_low_point(arr, r, c), "bad point at {}, {}", r, c);
                basin_sizes.push(smoke[r][c]);
            }
        }
    }
    let nbasins = basin_sizes.len();
    let top3 = basin_sizes.select_nth_unstable(nbasins - 4).2;
    assert_eq!(top3.len(), 3);
    top3.iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
2199943210
3987894921
9856789892
8767896789
9899965678
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 15);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 1134);
    }
}
