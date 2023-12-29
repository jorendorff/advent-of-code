use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

use crate::ray24::{Ray, Vec3};

type Point = (i128, i128, i128);
type Input = Vec<(Point, Point)>;

#[aoc_generator(day24, part1, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let space = ' ';
    let v3 = parser!(space* x:i128 "," space* y:i128 "," space* z:i128 space* => (x, y, z));
    let p = parser!(lines(
        v3 "@" v3
    ));
    Ok(p.parse(text)?)
}

fn intersect_in(a: Point, va: Point, b: Point, vb: Point, lo: i128, hi: i128) -> bool {
    println!("Hailstone A: {a:?} @ {va:?}");
    println!("Hailstone B: {b:?} @ {vb:?}");

    let mut c = vb.1 * va.0 - va.1 * vb.0;
    let mut d = va.0 * vb.0 * (b.1 - a.1) + va.1 * vb.0 * a.0 - vb.1 * va.0 * b.0;
    // x = -d/c

    if c == 0 {
        // vy/vx is the same for both stones.
        // The stones are thus moving in the same or opposite direction; either parallel or along the same line.
        //if dy/dx == vy/vx
        if (b.1 - a.1) * va.0 == (b.0 - a.0) * va.1 {
            panic!("unhandled case: hailstones moving along the same line");
        } else {
            println!("Hailstones' paths are parallel; they never intersect.");
            return false;
        }
    }

    // make it so x = d/c and c is positive
    if c < 0 {
        c = -c;
    } else {
        d = -d;
    }

    // if x < lo or x > hi
    if d < lo * c || d > hi * c {
        println!(
            "Hailstones' paths cross outside the test area (at x={}).",
            d as f64 / c as f64
        );
        return false;
    }

    // y = e/f
    let mut e = a.1 * c * va.0 + va.1 * (d - c * a.0);
    let mut f = c * va.0;
    if f == 0 {
        println!("uhhh not sure what this case signifies");
        return false;
    }
    if f < 0 {
        e = -e;
        f = -f;
    }

    // if y < lo or y > hi
    if e < lo * f || e > hi * f {
        println!(
            "Hailstones' paths cross outside the test area (at x={}, y={}).",
            d as f64 / c as f64,
            e as f64 / f as f64
        );
        return false;
    }

    // To check that this crossing occurs in the future for both stones,
    // check that y - y0 is either 0, or has the same sign as vy.
    // if y != y0 and (y - y0) * vy < 0
    if e != a.1 * f && (e - a.1 * f) * va.1 < 0 {
        println!("Hailstones' paths crossed in the past for hailstone A.");
        return false;
    }
    if e != b.1 * f && (e - b.1 * f) * vb.1 < 0 {
        println!("Hailstones' paths crossed in the past for hailstone B.");
        return false;
    }

    println!(
        "Hailstones' paths will cross **inside** the test area (at x={}, y={}).",
        d as f64 / c as f64,
        e as f64 / f as f64
    );
    true
}

fn solve_1(stones: &Input, lo: i128, hi: i128) -> usize {
    let mut count = 0;
    for (i, &(a, va)) in stones.iter().enumerate() {
        for &(b, vb) in stones[i + 1..].iter() {
            if intersect_in(a, va, b, vb, lo, hi) {
                count += 1;
            }
            println!();
        }
    }
    count
}

#[aoc(day24, part1, jorendorff)]
fn part_1(input: &Input) -> usize {
    solve_1(input, 200000000000000, 400000000000000)
}

#[aoc_generator(day24, part2, jorendorff)]
fn parse_input_2(text: &str) -> anyhow::Result<Vec<Ray>> {
    let space = ' ';
    let v3 = parser!(space* x:f64 "," space* y:f64 "," space* z:f64 space* => Vec3 { x, y, z });
    let p = parser!(lines(
        origin:v3 "@" vel:v3 => Ray { origin, vel }
    ));
    Ok(p.parse(text)?)
}

fn try_gradient_descent<F>(mut f: F, start: (f64, f64)) -> (f64, f64)
where
    F: FnMut(f64, f64) -> f64,
{
    let (mut t0, mut t1) = start;

    let mut rate = (t0 + t1) / 2.0;
    assert!(rate > 0.0);

    let num_steps = 40000;
    let num_octaves = rate.log(2.0) + 32.0;
    let decay = 0.5f64.powf(num_octaves / num_steps as f64);
    for _ in 0..num_steps {
        // println!("current (t0, t1): {t0}, {t1}");
        // let current_loss = f(t0, t1);
        // println!("After {i} steps, loss is {}", current_loss);

        // compute gradient
        let h = 0.1;
        let d0 = (f(t0 + h, t1) - f(t0 - h, t1)) / (2.0 * h);
        let d1 = (f(t0, t1 + h) - f(t0, t1 - h)) / (2.0 * h);

        // normalize it
        let m = d0.hypot(d1);
        let d0 = d0 / m;
        let d1 = d1 / m;

        t0 -= rate * d0;
        t0 = t0.max(0.0);
        t1 -= rate * d1;
        t1 = t1.max(0.0);

        rate *= decay;
        assert_ne!(decay, 0.0);
    }

    (t0, t1)
}

fn solve(hs0: Ray, hs1: Ray, rest: &[Ray]) -> Ray {
    // By gradient descent. Given any two hailstones, the solution must pass through both at
    // distinct times, say t0 and t1. From any pair of values (t0, t1), we can then construct
    // the unique ray that hits stone 0 at time t0, and stone 1 at time t1.
    let ray = move |t0: f64, t1: f64| -> Ray {
        assert_ne!(t0, t1);
        let p0 = hs0.origin + t0 * hs0.vel;
        let p1 = hs1.origin + t1 * hs1.vel;
        let vel = (p1 - p0) / (t1 - t0);
        let origin = p0 - t0 * vel;
        Ray { origin, vel }
    };

    // Define a function f(t0, t1) to be 0 if that ray hits all stones, and positive otherwise,
    // characterizing the badness of the choice of t0 and t1.
    let f = |t0, t1| {
        let r = ray(t0, t1);
        rest.iter()
            .map(|hailstone| r.nearest_approach_squared(*hailstone))
            .sum::<f64>()
    };

    // Now find values (t0, t1) that minimize f.
    // We try twice because I suspect there are two local minima.
    let ta = 100.0;
    let tb = 1e11;
    let ab = try_gradient_descent(f, (ta, tb));
    let fab = f(ab.0, ab.1);
    let ba = try_gradient_descent(f, (tb, ta));
    let fba = f(ba.0, ba.1);
    let (t0, t1) = if fab < fba { ab } else { ba };

    ray(t0, t1)
}

#[aoc(day24, part2, jorendorff)]
fn part_2(rays: &[Ray]) -> i64 {
    assert!(rays.len() > 2);
    let ray = solve(rays[0], rays[1], &rays[2..]);
    (ray.origin.x + ray.origin.y + ray.origin.z).round() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_intersect_in() {
        intersect_in((-5, 0, 0), (1, 0, 0), (5, 0, 0), (-1, 0, 0), -4, 4);
    }

    const EXAMPLE: &str = "\
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
";

    #[test]
    fn test_part_1() {
        assert_eq!(solve_1(&parse_input(EXAMPLE).unwrap(), 7, 27), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input_2(EXAMPLE).unwrap()), 47);
    }
}
