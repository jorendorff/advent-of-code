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

fn gradient<F>(mut loss: F, my_ray: Ray) -> Ray
where
    F: FnMut(Ray) -> f64,
{
    let h = 0.1;

    let mut left_ray = my_ray;
    left_ray.origin.x = my_ray.origin.x - h;
    let mut right_ray = my_ray;
    right_ray.origin.x = my_ray.origin.x + h;
    let dx = (loss(right_ray) - loss(left_ray)) / (2.0 * h);

    let mut left_ray = my_ray;
    left_ray.origin.y = my_ray.origin.y - h;
    let mut right_ray = my_ray;
    right_ray.origin.y = my_ray.origin.y + h;
    let dy = (loss(right_ray) - loss(left_ray)) / (2.0 * h);

    let mut left_ray = my_ray;
    left_ray.origin.z = my_ray.origin.z - h;
    let mut right_ray = my_ray;
    right_ray.origin.z = my_ray.origin.z + h;
    let dz = (loss(right_ray) - loss(left_ray)) / (2.0 * h);

    let mut left_ray = my_ray;
    left_ray.vel.x = my_ray.vel.x - h;
    let mut right_ray = my_ray;
    right_ray.vel.x = my_ray.vel.x + h;
    let dvx = (loss(right_ray) - loss(left_ray)) / (2.0 * h);

    let mut left_ray = my_ray;
    left_ray.vel.y = my_ray.vel.y - h;
    let mut right_ray = my_ray;
    right_ray.vel.y = my_ray.vel.y + h;
    let dvy = (loss(right_ray) - loss(left_ray)) / (2.0 * h);

    let mut left_ray = my_ray;
    left_ray.vel.z = my_ray.vel.z - h;
    let mut right_ray = my_ray;
    right_ray.vel.z = my_ray.vel.z + h;
    let dvz = (loss(right_ray) - loss(left_ray)) / (2.0 * h);

    Ray {
        origin: Vec3 {
            x: dx,
            y: dy,
            z: dz,
        },
        vel: Vec3 {
            x: dvx,
            y: dvy,
            z: dvz,
        },
    }
}

fn try_gradient_descent(rays: &[Ray], start: Ray) -> (Ray, f64) {
    let mut my_ray = start;

    let loss = |my_ray| {
        rays.iter()
            .map(|r| r.nearest_approach_squared(my_ray))
            .sum::<f64>()
    };

    let mut p_rate = 1.0e-4f64; // learning rates
    let mut v_rate = 1.0e-4f64; // learning rates
    let num_steps = 4000;
    let num_octaves = 16;
    let decay = 0.5f64.powf(num_octaves as f64 / num_steps as f64);
    for i in 0..num_steps {
        println!("current ray: {my_ray:?}");
        let current_loss = loss(my_ray);
        println!("After {i} steps, loss is {}", loss(my_ray));

        let g = gradient(loss, my_ray);

        dbg!(v_rate);
        dbg!(g.vel);
        dbg!(v_rate / current_loss * g.vel);
        //println!("Gradient is: {g:?}, v_rate * g.vel = {:?}", v_rate * g.vel);
        
        my_ray.origin -= p_rate / current_loss.sqrt() * g.origin;
        my_ray.vel -= v_rate / current_loss.sqrt() * g.vel;

        p_rate *= decay;
        v_rate *= decay;
    }

    (my_ray, loss(my_ray))
}

#[aoc(day24, part2, jorendorff)]
fn part_2(rays: &[Ray]) -> i64 {
    let total = rays.iter().fold(Ray::default(), |acc, ray| Ray {
        origin: acc.origin + ray.origin,
        vel: acc.vel + ray.vel,
    });

    let n = rays.len() as f64;
    let start = Ray {
        origin: total.origin / n,
        vel: total.vel / n,
    };

    let (best_ray, best_loss) = [
        (-1, -1, -1),
        (-1, -1, 1),
        (-1, 1, -1),
        (-1, 1, 1),
        (1, -1, -1),
        (1, -1, 1),
        (1, 1, -1),
        (1, 1, 1),
    ]
    .into_iter()
    .map(|(x, y, z)| {
        try_gradient_descent(
            rays,
            Ray {
                origin: start.origin,
                vel: Vec3 {
                    x: x as f64 * start.vel.x,
                    y: y as f64 * start.vel.y,
                    z: z as f64 * start.vel.z,
                },
            },
        )
    })
    .min_by(|(_ray1, loss1), (_ray2, loss2)| loss1.partial_cmp(loss2).unwrap())
    .unwrap();

    println!("{best_ray:?} loss: {best_loss}");

    (best_ray.origin.x + best_ray.origin.y + best_ray.origin.z).round() as i64
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
