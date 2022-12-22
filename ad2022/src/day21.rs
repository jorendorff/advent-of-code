use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Neg, Sub};

use num_bigint::BigInt;

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<(String, Rule)>;

type Ratio = num_rational::Ratio<BigInt>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq)]
enum Rule {
    Num(i64),
    Job(String, Op, String),
    Expr(Polynomial),
}

#[derive(Debug, Clone, PartialEq)]
struct Term {
    coeff: Ratio,
    degree: u64,
}

#[derive(Clone, Debug, Default, PartialEq)]
struct Polynomial {
    // maps degree to coefficient
    coeffs: HashMap<u64, Ratio>,
}

impl Polynomial {
    fn x() -> Self {
        Polynomial {
            coeffs: [(1, Ratio::new(1.into(), 1.into()))].into_iter().collect(),
        }
    }

    fn constant(i: i64) -> Self {
        Polynomial {
            coeffs: [(0, Ratio::new(i.into(), 1.into()))].into_iter().collect(),
        }
    }

    fn zero() -> Self {
        Self::default()
    }

    fn simplify(mut self) -> Self {
        self.coeffs.retain(|_k, v| *v != Ratio::default());
        self
    }
}

#[aoc_generator(day21, part1, jorendorff)]
#[aoc_generator(day21, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let name = parser!(string(alpha+));
    let op = parser!({
        '+' => Op::Add,
        '-' => Op::Sub,
        '*' => Op::Mul,
        '/' => Op::Div,
    });
    let p = parser!(lines(name ": " {
        n:i64 => Rule::Num(n),
        a:name ' ' o:op ' ' b:name => Rule::Job(a, o, b),
    }));
    Ok(p.parse(text)?)
}

fn what(rules: &mut HashMap<String, Rule>, target: &str) -> i64 {
    match &rules[target] {
        Rule::Num(n) => *n,
        Rule::Job(left, op, right) => {
            let left = left.to_string();
            let right = right.to_string();
            let op = *op;
            let ln = what(rules, &left);
            let rn = what(rules, &right);
            let out = match op {
                Op::Add => ln + rn,
                Op::Sub => ln - rn,
                Op::Mul => ln * rn,
                Op::Div => ln / rn,
            };
            rules.insert(target.to_string(), Rule::Num(out)); // cache
            out
        }
        _ => panic!("oh no"),
    }
}

#[aoc(day21, part1, jorendorff)]
fn part_1(input: &Input) -> i64 {
    let mut h = input.iter().cloned().collect::<HashMap<String, Rule>>();
    what(&mut h, "root")
}

impl Add for Polynomial {
    type Output = Polynomial;
    fn add(mut self, other: Polynomial) -> Polynomial {
        for (deg, coeff) in other.coeffs {
            *self.coeffs.entry(deg).or_default() += coeff;
        }
        self.simplify()
    }
}

impl Neg for Polynomial {
    type Output = Polynomial;
    fn neg(mut self) -> Polynomial {
        for (_deg, coeff) in &mut self.coeffs {
            *coeff *= Ratio::from(BigInt::from(-1));
        }
        self
    }
}

impl Sub for Polynomial {
    type Output = Polynomial;
    fn sub(mut self, other: Polynomial) -> Polynomial {
        for (deg, coeff) in other.coeffs {
            *self.coeffs.entry(deg).or_default() -= coeff;
        }
        self.simplify()
    }
}

impl Mul for Polynomial {
    type Output = Polynomial;
    fn mul(self, other: Polynomial) -> Polynomial {
        let mut p = Polynomial::zero();
        for (&deg0, coeff0) in self.coeffs.iter() {
            for (&deg1, coeff1) in other.coeffs.iter() {
                *p.coeffs.entry(deg0 + deg1).or_default() += coeff0 * coeff1;
            }
        }
        p.simplify()
    }
}

impl Div for Polynomial {
    type Output = Polynomial;
    fn div(self, other: Polynomial) -> Polynomial {
        assert_eq!(other.coeffs.len(), 1);
        let (rdeg, rcoeff) = other.coeffs.into_iter().next().unwrap();
        Polynomial {
            coeffs: self
                .coeffs
                .into_iter()
                .map(move |(ldeg, lcoeff)| (ldeg - rdeg, lcoeff / rcoeff.clone()))
                .collect(),
        }
    }
}

fn what2(rules: &mut HashMap<String, Rule>, target: &str) -> Polynomial {
    if target == "humn" {
        return Polynomial::x();
    }

    match &rules[target] {
        Rule::Num(n) => Polynomial::constant(*n),
        Rule::Expr(x) => x.clone(),
        Rule::Job(left, op, right) => {
            let left = left.to_string();
            let right = right.to_string();
            let op = *op;
            let ln = what2(rules, &left);
            let rn = what2(rules, &right);
            let out = match op {
                Op::Add => ln + rn,
                Op::Sub => ln - rn,
                Op::Mul => ln * rn,
                Op::Div => ln / rn,
            };
            rules.insert(target.to_string(), Rule::Expr(out.clone())); // cache
            out
        }
    }
}

#[aoc(day21, part2, jorendorff)]
fn part_2(input: &Input) -> Ratio {
    let mut h = input.iter().cloned().collect::<HashMap<String, Rule>>();
    if let Rule::Job(_left, op, _right) = h.get_mut("root").unwrap() {
        *op = Op::Sub;
    }
    let diff: Polynomial = what2(&mut h, "root");

    for deg in diff.coeffs.keys() {
        assert!(*deg == 0 || *deg == 1);
    }

    let c0 = diff
        .coeffs
        .get(&0)
        .cloned()
        .unwrap_or_else(|| Ratio::new(0.into(), 1.into()));
    let c1 = diff
        .coeffs
        .get(&1)
        .cloned()
        .unwrap_or_else(|| Ratio::new(0.into(), 1.into()));

    -c0 / c1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 152);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(&parse_input(EXAMPLE).unwrap()),
            Ratio::from(BigInt::from(301))
        );
    }
}
