use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<(Vec<Status>, Vec<usize>)>;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Status {
    Unk,
    Op,
    Dmg,
}

use Status::*;

#[aoc_generator(day12, part1, jorendorff)]
#[aoc_generator(day12, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines(
        {'?' => Unk, '.' => Op, '#' => Dmg }+ " " repeat_sep(usize, ',')
    ));
    Ok(p.parse(text)?)
}

fn can_place(log: &[Status], start: usize, size: usize) -> bool {
    log[start..start+size].iter().all(|x| *x != Op) && (start + size == log.len() || log[start + size] != Dmg)
}

fn place_next(log: &[Status], mut start: usize, size: usize) -> Vec<u64> {
    let mut answers = vec![];
    while start + size <= log.len() {
        if can_place(log, start, size) {
            answers.push(1);
        } else {
            answers.push(0);
        }
        if log[start] == Dmg {
            break;
        }
        start += 1;
    }
    answers
}

fn solve(log: &[Status], sizes: &[usize]) -> u64 {
    // Let f(n, i) = number of ways groups 0..n can be placed to put group n-1 starting at offset i.
    // Then f(1, i) can be computed ad hoc
    let mut f = place_next(log, 0, sizes[0]);
    f.resize(log.len(), 0); // extend with zeros to full length

    // and given f(n, i), each element of that row contributes to f(n+1, j) 
    let mut prev_len = sizes[0];
    for &group_size in &sizes[1..] {
        let mut g = vec![0; log.len()];
        for (i, count) in f.into_iter().enumerate() {
            if count != 0 {
                for (j, jcount) in place_next(log, i + prev_len + 1, group_size).into_iter().enumerate() {
                    g[i + prev_len + 1 + j] += count * jcount;
                }
            }
        }
        prev_len = group_size;
        f = g;
    }

    // now kill answers that are impossible because there are more #'s at the end
    for i in 0..f.len() {
        if f[i] != 0 && log[i + prev_len..].iter().any(|x| *x == Dmg) {
            f[i] = 0;
        }
    }
    
    // and the answer is the sum of f(n, i) for n=len(sizes), i=0..len(log).
    f.into_iter().sum::<u64>()
}

#[aoc(day12, part1, jorendorff)]
fn part_1(input: &Input) -> u64 {
    input.iter().map(|(log, sizes)| solve(log, sizes)).sum()
}

#[aoc(day12, part2, jorendorff)]
fn part_2(input: &Input) -> u64 {
    // 897 on the global leaderboard
    input.iter().map(|(log, sizes)| {
        let log = log.to_vec();
        let mut my_log = log.clone();
        for _ in 0..4 {
            my_log.push(Unk);
            my_log.append(&mut log.clone());
        }
        let sizes = (0..5).flat_map(|_i| sizes.iter().copied()).collect::<Vec<usize>>();
        solve(&my_log, &sizes)
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

    #[track_caller]
    fn test_case(case: &str, expected: u64) {
        let (log, sizes) = parse_input(&(case.to_string() + "\n")).unwrap().pop().unwrap();
        assert_eq!(solve(&log, &sizes), expected);
    }

    #[test]
    fn test_solve() {
        test_case("???#?????? 4,4", 3);
        test_case("??..#??#??.???? 5,1", 4);
        test_case("#.?.?????.######## 1,2,8", 4);
        test_case("..?.?????.######## 1,2,8", 7);
        test_case("?.?.?????.######## 1,2,8", 11);
        test_case("?.?.???????##????# 1,2,8", 11);
    }
    
    #[test]
    fn test_part_1() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(solve(&input[0].0,&input[0].1), 1);
        assert_eq!(solve(&input[1].0,&input[1].1), 4);
        assert_eq!(solve(&input[2].0,&input[2].1), 1);
        assert_eq!(solve(&input[3].0,&input[3].1), 1);
        assert_eq!(solve(&input[4].0,&input[4].1), 4);
        assert_eq!(solve(&input[5].0,&input[5].1), 10);
        assert_eq!(part_1(&input), 21);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 525152);
    }
}
