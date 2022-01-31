use aoc_runner_derive::*;

#[aoc_generator(day25, part1, jorendorff)]
#[aoc_generator(day25, part2, jorendorff)]
fn parse_input(text: &str) -> Vec<Vec<u8>> {
    text.lines().map(|line| line.as_bytes().to_vec()).collect()
}

#[aoc(day25, part1, jorendorff)]
fn part_1(grid: &Vec<Vec<u8>>) -> u64 {
    let w = grid[0].len();
    let h = grid.len();
    let mut grid = grid.to_owned();
    let mut count = 0;

    println!("Initial state: ");
    for row in &grid {
        println!("{}", std::str::from_utf8(row).unwrap());
    }
    println!();

    loop {
        count += 1;
        let mut moved = false;
        for row in &mut grid {
            let new_row: Vec<u8> = (0..w)
                .map(|c| {
                    let cm1 = (c + w - 1) % w;
                    let cp1 = (c + 1) % w;
                    if row[c] == b'.' && row[cm1] == b'>' {
                        moved = true;
                        b'>'
                    } else if row[c] == b'>' && row[cp1] == b'.' {
                        b'.'
                    } else {
                        row[c]
                    }
                })
                .collect();
            *row = new_row;
        }
        for c in 0..w {
            let movers: Vec<usize> = (0..h)
                .filter(|&r| grid[r][c] == b'v' && grid[(r + 1) % h][c] == b'.')
                .collect();
            for r in movers {
                moved = true;
                grid[r][c] = b'.';
                grid[(r + 1) % h][c] = b'v';
            }
        }

        println!("After {} steps: ", count);
        for row in &grid {
            println!("{}", std::str::from_utf8(row).unwrap());
        }
        println!();

        if !moved {
            break;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE)), 58);
    }
}
