use aoc_runner_derive::*;

type Input = Vec<Vec<u8>>;

#[aoc_generator(day15, part1, jorendorff)]
#[aoc_generator(day15, part2, jorendorff)]
fn parse_input(text: &str) -> Input {
    text.lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect::<Vec<u8>>())
        .collect()
}

#[aoc(day15, part1, jorendorff)]
fn part_1(input: &Input) -> u32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    type Heap = BinaryHeap<(Reverse<u32>, usize, usize)>;

    let h = input.len();
    let w = input[0].len();

    let mut costs = vec![vec![None; w]; h];
    costs[0][0] = Some(0);
    let mut queue = Heap::new();

    fn try_enter(
        input: &[Vec<u8>],
        costs: &mut Vec<Vec<Option<u32>>>,
        queue: &mut Heap,
        r: usize,
        c: usize,
        current: u32,
    ) {
        if costs[r][c].is_none() {
            let total = current + input[r][c] as u32;
            costs[r][c] = Some(total);
            queue.push((Reverse(total), r, c));
        }
    }

    queue.push((Reverse(0), 0, 0));
    while costs[h - 1][w - 1].is_none() {
        if let Some((Reverse(current), r, c)) = queue.pop() {
            if r > 0 {
                try_enter(input, &mut costs, &mut queue, r - 1, c, current);
            }
            if r < h - 1 {
                try_enter(input, &mut costs, &mut queue, r + 1, c, current);
            }
            if c > 0 {
                try_enter(input, &mut costs, &mut queue, r, c - 1, current);
            }
            if c < w - 1 {
                try_enter(input, &mut costs, &mut queue, r, c + 1, current);
            }
        } else {
            break;
        }
    }
    costs[h - 1][w - 1].unwrap()
}

#[aoc(day15, part2, jorendorff)]
fn part_2(input: &Input) -> u32 {
    let h = input.len();
    let w = input[0].len();
    let expanded_cave: Vec<Vec<u8>> = (0..5)
        .flat_map(|rr| {
            (0..h).map(move |r| {
                (0..5)
                    .flat_map(move |cc| {
                        (0..w).map(move |c| {
                            let risk = input[r][c] + rr + cc;
                            if risk > 9 {
                                risk - 9
                            } else {
                                risk
                            }
                        })
                    })
                    .collect()
            })
        })
        .collect();
    for r in expanded_cave.iter() {
        println!(
            "{}",
            r.iter().map(|&b| (b'0' + b) as char).collect::<String>()
        );
    }
    part_1(&expanded_cave)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE)), 40);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE)), 315);
    }
}
