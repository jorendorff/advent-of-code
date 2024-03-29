use aoc_runner_derive::*;

#[aoc_generator(day7, part1, jorendorff)]
#[aoc_generator(day7, part1, jorendorff_binary_search)]
#[aoc_generator(day7, part2, jorendorff_binary_search)]
#[aoc_generator(day7, part2, jorendorff_parabolic)]
fn parse_input(text: &str) -> Result<Vec<i64>, std::num::ParseIntError> {
    text.split(',')
        .map(|num| num.trim().parse::<i64>())
        .collect()
}

#[aoc(day7, part1, jorendorff)]
fn part_1(nums: &[i64]) -> i64 {
    let n = nums.len();
    let mut nums = nums.to_vec();
    let x = *nums.select_nth_unstable(n / 2).1;
    nums.iter().map(|&xi| (x - xi).abs()).sum()
}

// Given a U-shaped function `f`, find any coordinate x in the range `lo..hi`
// for which f(x) is minimal.
//
// A U-shaped function has a minimum, but no other local minima. It may have a
// flat region at the minimum.
fn find_minimum<F: Fn(i64) -> i64>(mut lo: i64, mut hi: i64, f: F) -> i64 {
    // Hand-coded binary search, once again.
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        let fx = f(mid);
        let fxp1 = f(mid + 1);
        if fxp1 < fx {
            lo = mid + 1;
        } else if fxp1 == fx {
            return mid;
        } else if f(mid - 1) < fx {
            hi = mid;
        } else {
            return mid;
        }
    }
    lo
}

#[aoc(day7, part1, jorendorff_binary_search)]
fn part_1_binary_search(nums: &[i64]) -> i64 {
    let fuel_cost = |x: i64| -> i64 { nums.iter().map(|&xi| (x - xi).abs()).sum() };
    let lo = nums.iter().copied().min().unwrap();
    let hi = nums.iter().copied().max().unwrap() + 1;
    let x = find_minimum(lo, hi, fuel_cost);
    fuel_cost(x)
}

#[aoc(day7, part2, jorendorff_binary_search)]
fn part_2_binary_search(nums: &[i64]) -> i64 {
    // The fuel cost for a crab at x0, in terms of x, is a function of the
    // distance traveled, `d = (x - x0).abs()`. It is, specifically, the d'th
    // triangle number, `d * (d + 1) / 2`.
    let fuel_cost_at = |x: i64| -> i64 {
        nums.iter()
            .copied()
            .map(|x0| {
                let d = (x - x0).abs();
                d * (d + 1) / 2
            })
            .sum()
    };

    let lo = nums.iter().copied().min().unwrap();
    let hi = nums.iter().copied().max().unwrap() + 1;
    fuel_cost_at(find_minimum(lo, hi, fuel_cost_at))
}

#[aoc(day7, part2, jorendorff_parabolic)]
fn part_2(nums: &[i64]) -> i64 {
    let mut nums = nums.to_vec();
    nums.sort_unstable();

    // The function to minimize is a sum of parabolas--almost. The fuel cost
    // for a crab at xᵢ, in terms of its destination x, is a function of the
    // distance traveled, `d = (x - xᵢ).abs()`. It is, specifically, the d'th
    // triangle number, `d * (d + 1) / 2`. However, the `.abs()` is a bit of a
    // problem; we have
    //
    //     fᵢ(x) = (x - xᵢ) * (x - xᵢ - 1) / 2   if x < xᵢ
    //             (x - xᵢ) * (x - xᵢ + 1) / 2   otherwise
    //
    // and the goal is to find the minimum of the function `f` that is the sum
    // of all N of these weirdos.
    //
    // But between any two crabs, the sum of all these functions is indeed just
    // a parabola. Each individual region is very easy to solve -- the vertex
    // of a parabola is at x coordinate `-b/2a`.

    // First recast the parabolas into standard form.
    //
    //     fᵢ(x) = 1/2 * x² + (-xᵢ - 1/2) * x + (xᵢ * (xᵢ + 1) / 2)   if x ≤ xᵢ
    //             1/2 * x² + (-xᵢ + 1/2) * x + (xᵢ * (xᵢ - 1) / 2)   otherwise
    //
    // Ugh, division by 2. Let's double all coefficients--it won't affect the x
    // coordinate of the minimum point--and then we must remember to divide the
    // answer by 2 at each `return`.
    //
    //     fᵢ(x) = x² + (-2xᵢ - 1) * x + (xᵢ * (xᵢ + 1))   if x ≤ xᵢ
    //             x² + (-2xᵢ + 1) * x + (xᵢ * (xᵢ - 1))   otherwise
    //
    // We can scan the list from left to right, examining each parabolic slice.
    // For the leftmost slice, x ≤ xᵢ will be true for all i, so we have these
    // coefficients which describe that slice:
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    for &xi in &nums {
        a += 1;
        b += -2 * xi - 1;
        c += xi * (xi + 1);
    }

    /// Find the value of x that minimizes the value of ax² + bx.
    ///
    /// That's a parabola with vertex at -b/2a. But the vertex may have a
    /// fractional part. Which of the 2 nearest integers has the minimum y
    /// value? Fortunately, it's whichever point is closer to the vertex --
    /// that is, we can just round to the nearest integer.
    fn xmin(a: i64, b: i64) -> i64 {
        debug_assert!(a > 0, "parabola must open upward");
        debug_assert!(-b >= 0, "negacrabs?! sorry, this will round incorrectly");
        (-b + a) / (2 * a)
    }

    // The vertex is at -b/2a.
    let vx = xmin(a, b);
    if vx <= nums[0] {
        return (a * vx * vx + b * vx + c) / 2;
    }

    let mut left = nums[0];
    for &right in &nums[1..] {
        // OK, move on to the next segment. In this segment, where we once had x ≤ xᵢ,
        // now that is no longer the case, so we must adjust our summary (a, b, c) of f
        // to account for fᵢ tipping into the right-hand region.
        b += 2;
        c -= 2 * left;
        let vx = xmin(a, b).max(left);
        if vx <= right {
            return (a * vx * vx + b * vx + c) / 2;
        }

        left = right;
    }

    // Last segment.
    b += 2;
    c -= 2 * left;
    let vx = xmin(a, b).max(left);
    (a * vx * vx + b * vx + c) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
16,1,2,0,4,2,7,1,2,14
";

    #[test]
    fn test_find_minimum() {
        // exhaustively construct all reasonably small test cases
        for x_min_left in 0..16 {
            for x_min_right in x_min_left..=(x_min_left + 2) {
                for right in x_min_right..=(x_min_right + 16) {
                    let f = move |x| {
                        if x < x_min_left {
                            x_min_left - x
                        } else if x <= x_min_right {
                            0
                        } else {
                            x - x_min_right
                        }
                    };
                    let x_min = find_minimum(0, right, f);
                    assert!(
                        x_min_left <= x_min && x_min <= x_min_right,
                        "failed test case (x_min_left={}, x_min_right={}, right={}); expected {}..={}, got x_min={}",
                        x_min_left,
                        x_min_right,
                        right,
                        x_min_left,
                        x_min_right,
                        x_min,
                    );
                }
            }
        }
    }

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
