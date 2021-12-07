use aoc_runner_derive::*;

#[aoc_generator(day7)]
fn parse_input(text: &str) -> Result<Vec<i64>, std::num::ParseIntError> {
    text.split(',')
        .map(|num| num.trim().parse::<i64>())
        .collect()
}

#[aoc(day7, part1)]
fn part_1(nums: &[i64]) -> i64 {
    let mut nums = nums.to_vec();
    nums.sort_unstable();
    let x = nums[nums.len() / 2];
    nums.iter().copied().map(|x0| (x - x0).abs()).sum()
}

#[aoc(day7, part2)]
fn part_2(nums: &[i64]) -> i64 {
    // The function to minimize is a sum of parabolas--almost. The fuel cost
    // for a crab at x0, in terms of x, is a function of `d = (x - x0).abs()`.
    // it is, specifically, the d'th triangle number, `d * (d + 1) / 2`. I
    // would love to get rid of the abs() part and solve algebraically.
    // Unfortunately it's not trivial. There is a neat solution with a single
    // binary search over `nums` (sorted and dedup'd), which involves finding
    // the vertex of each parabolic slice of the function; only one slice will
    // actually contain its vertex, and that's the solution.
    let fuel_cost_at = |x: i64| -> i64 {
        nums.iter()
            .copied()
            .map(|x0| {
                let d = (x - x0).abs();
                d * (d + 1) / 2
            })
            .sum()
    };

    let mut lo = nums.iter().copied().min().unwrap();
    let mut hi = nums.iter().copied().max().unwrap() + 1;

    // Hand-coded binary search, once again.
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        let fx = fuel_cost_at(mid);
        let fxp1 = fuel_cost_at(mid + 1);
        if fxp1 < fx {
            lo = mid + 1;
        } else if fxp1 == fx {
            break;
        } else {
            let fxm1 = fuel_cost_at(mid - 1);
            if fxm1 < fx {
                hi = mid;
            } else {
                break;
            }
        }
    }

    fuel_cost_at(lo)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
16,1,2,0,4,2,7,1,2,14
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 37);

        assert_eq!(part_1(&[4]), 0);
        assert_eq!(part_1(&[3, 4, 4, 55]), 52);
        assert_eq!(part_1(&[0, 4, 6, 10]), 12);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 168);
    }
}
