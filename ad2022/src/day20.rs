use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<i64>;

#[aoc_generator(day20, part1, jorendorff)]
#[aoc_generator(day20, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines(i64));
    Ok(p.parse(text)?)
}

fn mix(input: &[i64], pos_to_id: &mut [usize], id_to_pos: &mut [usize]) {
    let n = input.len();
    let mut saved_vv = (0..n).map(|p| input[pos_to_id[p]]).collect::<Vec<i64>>();
    for id in 0..n {
        for j in 0..n {
            assert_eq!(pos_to_id[id_to_pos[j]], j);
            assert_eq!(id_to_pos[pos_to_id[j]], j);
        }

        let orig = id_to_pos[id];
        let dx = input[id];
        let dest = if dx > 0 {
            (orig + dx as usize - 1) % (n - 1) + 1
        } else if dx < 0 {
            let dx = -((-dx as usize % (n - 1)) as i64);
            (orig + (n as i64 - 1 + dx) as usize) % (n - 1)
        } else {
            orig
        };
        if dest < orig {
            for p in (dest..orig).rev() {
                pos_to_id[p + 1] = pos_to_id[p];
                id_to_pos[pos_to_id[p]] += 1;
            }
        } else {
            for p in orig..dest {
                pos_to_id[p] = pos_to_id[p + 1];
                id_to_pos[pos_to_id[p]] -= 1;
            }
        }
        pos_to_id[dest] = id;
        id_to_pos[id] = dest;

        let vv = (0..n).map(|p| input[pos_to_id[p]]).collect::<Vec<i64>>();
        if dest < orig {
            assert_eq!(
                vv,
                saved_vv[0..dest]
                    .iter()
                    .cloned()
                    .chain([input[id]].into_iter())
                    .chain(saved_vv[dest..orig].iter().cloned())
                    .chain(saved_vv[orig + 1..].iter().cloned())
                    .collect::<Vec<i64>>()
            );
        } else {
            assert_eq!(
                vv,
                saved_vv[0..orig]
                    .iter()
                    .cloned()
                    .chain(saved_vv[orig + 1..dest + 1].iter().cloned())
                    .chain([input[id]].into_iter())
                    .chain(saved_vv[dest + 1..].iter().cloned())
                    .collect::<Vec<i64>>()
            );
        }
        saved_vv = vv;
    }
}

#[aoc(day20, part1, jorendorff)]
fn part_1(input: &Input) -> i64 {
    let zero_id = input.iter().copied().take_while(|&x| x != 0).count();
    let n = input.len();

    let mut pos_to_id = (0..n).collect::<Vec<usize>>();
    let mut id_to_pos = pos_to_id.clone();

    mix(input, &mut pos_to_id, &mut id_to_pos);

    let p0 = id_to_pos[zero_id];
    input[pos_to_id[(p0 + 1000) % n]]
        + input[pos_to_id[(p0 + 2000) % n]]
        + input[pos_to_id[(p0 + 3000) % n]]
}

#[aoc(day20, part2, jorendorff)]
fn part_2(input: &Input) -> i64 {
    let zero_id = input.iter().copied().take_while(|&x| x != 0).count();
    let n = input.len();

    const KEY: i64 = 811589153;
    let input = input.iter().copied().map(|x| x * KEY).collect::<Vec<i64>>();

    let mut pos_to_id = (0..n).collect::<Vec<usize>>();
    let mut id_to_pos = pos_to_id.clone();

    for _ in 0..10 {
        mix(&input, &mut pos_to_id, &mut id_to_pos);
    }

    let p0 = id_to_pos[zero_id];
    input[pos_to_id[(p0 + 1000) % n]]
        + input[pos_to_id[(p0 + 2000) % n]]
        + input[pos_to_id[(p0 + 3000) % n]]
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
1
2
-3
3
-2
0
4
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 3);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 1623178306);
    }
}
