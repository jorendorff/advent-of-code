use aoc_runner_derive::*;

#[aoc_generator(day21, part1, jorendorff)]
#[aoc_generator(day21, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Vec<i32>> {
    text.lines()
        .map(|line| -> anyhow::Result<i32> {
            Ok(line
                .split_whitespace()
                .rev()
                .next()
                .unwrap()
                .parse::<i32>()?)
        })
        .collect()
}

#[aoc(day21, part1, jorendorff)]
fn part_1(starts: &Vec<i32>) -> i32 {
    let mut pos = starts.clone();
    let mut score = [0, 0];
    let mut turn = 0;
    let mut nrolls = 0;
    while score[0] < 1000 && score[1] < 1000 {
        for _ in 0..3 {
            pos[turn] = (pos[turn] + (nrolls % 100 + 1) - 1) % 10 + 1;
            nrolls += 1;
        }
        score[turn] += pos[turn];
        turn = (turn + 1) % pos.len();
    }
    score.into_iter().min().unwrap() * nrolls
}

// cache[p0][p1][s0][s1][0] is the number of universes where player 0 wins
// when it is player 0's turn
// starting at position p0+1 and score s0,
// with their opponent at position p1+1 and score s1.
// cache[p0][p1][s0][s1][1] is the number of universes where the other player wins.
type Cache = [[[[[i64; 2]; 21]; 21]; 10]; 10];

fn answers(cache: &mut Cache, p0: usize, p1: usize, s0: usize, s1: usize) -> [i64; 2] {
    if s1 >= 21 {
        [0, 1]
    } else if cache[p0][p1][s0][s1][0] != -1 {
        cache[p0][p1][s0][s1]
    } else {
        let mut counts = [0; 2];
        for r1 in 1..=3 {
            for r2 in 1..=3 {
                for r3 in 1..=3 {
                    let p0 = (p0 + r1 + r2 + r3) % 10;
                    let s0 = s0 + (p0 + 1);
                    let [w1, w0] = answers(cache, p1, p0, s1, s0);
                    counts[0] += w0;
                    counts[1] += w1;
                }
            }
        }
        cache[p0][p1][s0][s1] = counts;
        counts
    }
}

#[aoc(day21, part2, jorendorff)]
fn part_2(starts: &Vec<i32>) -> i64 {
    let mut cache = [[[[[-1; 2]; 21]; 21]; 10]; 10];
    answers(
        &mut cache,
        starts[0] as usize - 1,
        starts[1] as usize - 1,
        0,
        0,
    )
    .iter()
    .copied()
    .max()
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Player 1 starting position: 4
Player 2 starting position: 8
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 739785);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 444356092776315);
    }
}
