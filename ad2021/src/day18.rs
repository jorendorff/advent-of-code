use aoc_runner_derive::*;

#[derive(Clone)]
enum Number {
    Regular(i64),
    Pair(Box<Number>, Box<Number>),
}

impl std::fmt::Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Regular(n) => write!(f, "{:?}", n),
            Number::Pair(left, right) => write!(f, "[{:?},{:?}]", *left, *right),
        }
    }
}

impl Number {
    fn magnitude(&self) -> i64 {
        match self {
            Number::Regular(n) => *n,
            Number::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }

    fn reduce(&mut self) {
        self.explode(0);
        while self.split_once() {
            self.explode(0);
        }
    }

    // Ok if successfully exploded, Err(self) unchanged if nothing was nested
    // deeply enough to explode.
    fn explode(&mut self, depth: usize) -> (i64, i64) {
        match self {
            Number::Regular(_n) => (0, 0),
            Number::Pair(left, right) if depth == 4 => {
                let left = left.magnitude();
                let right = right.magnitude();
                *self = Number::Regular(0);
                (left, right)
            }
            Number::Pair(left, right) => {
                let (ll, lr) = left.explode(depth + 1);
                if lr != 0 {
                    right.add_to_leftmost(lr);
                }
                let (rl, rr) = right.explode(depth + 1);
                if rl != 0 {
                    left.add_to_rightmost(rl);
                }
                (ll, rr)
            }
        }
    }

    fn split_once(&mut self) -> bool {
        match self {
            Number::Regular(n) => {
                if *n >= 10 {
                    *self = Number::Pair(
                        Box::new(Number::Regular(*n / 2)),
                        Box::new(Number::Regular((*n + 1) / 2)),
                    );
                    true
                } else {
                    false
                }
            }
            Number::Pair(left, right) => left.split_once() || right.split_once(),
        }
    }

    fn add_to_leftmost(&mut self, n: i64) {
        match self {
            Self::Regular(m) => *m += n,
            Self::Pair(left, _right) => left.add_to_leftmost(n),
        }
    }

    fn add_to_rightmost(&mut self, n: i64) {
        match self {
            Self::Regular(m) => *m += n,
            Self::Pair(_left, right) => right.add_to_rightmost(n),
        }
    }

    fn add(self, other: Number) -> Self {
        let mut out = Number::Pair(Box::new(self), Box::new(other));
        out.reduce();
        out
    }
}

struct Parser<'a> {
    text: &'a str,
    point: usize,
}

impl<'a> Parser<'a> {
    fn looking_at(&self, s: &str) -> bool {
        self.text[self.point..].starts_with(s)
    }

    fn at_end(&self) -> bool {
        self.point == self.text.len()
    }

    fn parse_number(&mut self) -> anyhow::Result<Number> {
        anyhow::ensure!(!self.at_end());
        if self.looking_at("[") {
            self.point += 1;
            let lhs = Box::new(self.parse_number()?);
            anyhow::ensure!(self.looking_at(","));
            self.point += 1;
            let rhs = Box::new(self.parse_number()?);
            anyhow::ensure!(self.looking_at("]"));
            self.point += 1;
            Ok(Number::Pair(lhs, rhs))
        } else {
            let mut j = self.point;
            while let Some(next_ch) = self.text[j..].chars().next() {
                if !next_ch.is_ascii_digit() {
                    break;
                }
                j += 1;
            }
            let n = self.text[self.point..j].parse::<i64>()?;
            self.point = j;
            Ok(Number::Regular(n))
        }
    }
}

fn parse_number(s: &str) -> anyhow::Result<Number> {
    let mut parser = Parser { text: s, point: 0 };
    let num = parser.parse_number()?;
    anyhow::ensure!(parser.at_end());
    Ok(num)
}

#[aoc_generator(day18, part1, jorendorff)]
#[aoc_generator(day18, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Vec<Number>> {
    text.lines().map(parse_number).collect()
}

fn sum(nums: impl IntoIterator<Item = Number>) -> Number {
    let mut nums = nums.into_iter();
    let first = nums.next().unwrap();
    nums.fold(first, |acc, next| acc.add(next))
}

#[aoc(day18, part1, jorendorff)]
fn part_1(input: &Vec<Number>) -> i64 {
    sum(input.clone()).magnitude()
}

#[aoc(day18, part2, jorendorff)]
fn part_2(input: &Vec<Number>) -> i64 {
    (0..input.len())
        .flat_map(|i| {
            (0..input.len()).filter_map(move |j| {
                if i == j {
                    None
                } else {
                    Some(input[i].clone().add(input[j].clone()).magnitude())
                }
            })
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[track_caller]
    fn check_parse_round_trip(s: &str) {
        let num = parse_number(s).unwrap();
        assert_eq!(format!("{:?}", num), s);
    }

    #[test]
    fn test_parser() {
        check_parse_round_trip("[1,2]");
        check_parse_round_trip("[[1,2],3]");
        check_parse_round_trip("[9,[8,7]]");
        check_parse_round_trip("[[1,9],[8,5]]");
        check_parse_round_trip("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]");
        check_parse_round_trip("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]");
        check_parse_round_trip("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]");
    }

    #[track_caller]
    fn check_add(s1: &str, s2: &str, expected: &str) {
        let num1 = parse_number(s1).unwrap();
        let num2 = parse_number(s2).unwrap();
        let actual = num1.add(num2);
        assert_eq!(format!("{:?}", actual), expected);
    }

    #[test]
    fn test_add() {
        check_add("[1,2]", "[[3,4],5]", "[[1,2],[[3,4],5]]");
        check_add(
            "[[[[4,3],4],4],[7,[[8,4],9]]]",
            "[1,1]",
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        );
    }

    #[track_caller]
    fn check_sum(lines: &str, expected: &str) {
        let nums = parse_input(lines).unwrap();
        let total = sum(nums);
        assert_eq!(format!("{:?}", total), expected);
    }

    #[test]
    fn test_sum() {
        check_sum(
            "\
[1,1]
[2,2]
[3,3]
[4,4]
",
            "[[[[1,1],[2,2]],[3,3]],[4,4]]",
        );

        check_sum(
            "\
[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
",
            "[[[[3,0],[5,3]],[4,4]],[5,5]]",
        );

        check_sum(
            "\
[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]
",
            "[[[[5,0],[7,4]],[5,5]],[6,6]]",
        );

        check_sum(
            "\
[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]
",
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        );
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(parse_number("[9,1]").unwrap().magnitude(), 29);
        assert_eq!(parse_number("[1,9]").unwrap().magnitude(), 21);
        assert_eq!(parse_number("[[9,1],[1,9]]").unwrap().magnitude(), 129);

        assert_eq!(parse_number("[[1,2],[[3,4],5]]").unwrap().magnitude(), 143);
        assert_eq!(
            parse_number("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
                .unwrap()
                .magnitude(),
            1384
        );
        assert_eq!(
            parse_number("[[[[1,1],[2,2]],[3,3]],[4,4]]")
                .unwrap()
                .magnitude(),
            445
        );
        assert_eq!(
            parse_number("[[[[3,0],[5,3]],[4,4]],[5,5]]")
                .unwrap()
                .magnitude(),
            791
        );
        assert_eq!(
            parse_number("[[[[5,0],[7,4]],[5,5]],[6,6]]")
                .unwrap()
                .magnitude(),
            1137
        );
        assert_eq!(
            parse_number("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
                .unwrap()
                .magnitude(),
            3488
        );
    }

    const EXAMPLE: &str = "\
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 4140);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 3993);
    }
}
