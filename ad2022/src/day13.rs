use std::cmp::Ordering;

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

enum Token {
    OpenBkt,
    CloseBkt,
    Int(u64),
    Comma,
}
use Token::*;

#[derive(Clone)]
enum Value {
    Int(u64),
    List(Vec<Value>),
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => i.fmt(f),
            Value::List(v) => v.fmt(f),
        }
    }
}

fn cmp_lists(a: &[Value], b: &[Value]) -> Ordering {
    a.cmp(b)
}

impl std::cmp::PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Value {}

impl std::cmp::Ord for Value {
    fn cmp(&self, b: &Value) -> Ordering {
        match (self, b) {
            (Value::Int(a), Value::Int(b)) => a.cmp(b),
            (Value::Int(a), Value::List(b)) => cmp_lists(&[Value::Int(*a)], b),
            (Value::List(a), Value::Int(b)) => cmp_lists(a, &[Value::Int(*b)]),
            (Value::List(a), Value::List(b)) => cmp_lists(a, b),
        }
    }
}

fn parse_list(tokens: &[Token]) -> Vec<Value> {
    let mut lists = vec![vec![]];
    for t in tokens {
        match t {
            OpenBkt => lists.push(vec![]),
            CloseBkt => {
                let top = lists.pop().unwrap();
                lists.last_mut().unwrap().push(Value::List(top));
            }
            Int(n) => lists.last_mut().unwrap().push(Value::Int(*n)),
            Comma => {}
        }
    }
    assert_eq!(lists.len(), 1);
    let mut list = lists.pop().unwrap();
    assert_eq!(list.len(), 1);
    match list.pop().unwrap() {
        Value::Int(_) => panic!("invalid input"),
        Value::List(data) => data,
    }
}

type Input1 = Vec<(Vec<Value>, Vec<Value>)>;

#[aoc_generator(day13, part1, jorendorff)]
fn parse_input_1(text: &str) -> anyhow::Result<Input1> {
    let signal = parser!(tokens:{
        '[' => OpenBkt,
        ']' => CloseBkt,
        n:u64 => Int(n),
        ',' => Comma,
    }+ => parse_list(&tokens));
    let p = parser!(sections(line(signal) line(signal)));
    aoc_parse(text, p)
}

#[aoc_generator(day13, part2, jorendorff)]
fn parse_input_2(text: &str) -> anyhow::Result<Vec<Vec<Value>>> {
    let signal = parser!(tokens:{
        '[' => OpenBkt,
        ']' => CloseBkt,
        n:u64 => Int(n),
        ',' => Comma,
    }+ => parse_list(&tokens));
    let p =
        parser!(s:sections(lines(signal)) => s.into_iter().flatten().collect::<Vec<Vec<Value>>>());
    aoc_parse(text, p)
}

#[aoc(day13, part1, jorendorff)]
fn part_1(input: &Input1) -> usize {
    input
        .iter()
        .enumerate()
        .filter(|(_i, (a, b))| cmp_lists(a, b) != Ordering::Greater)
        .map(|(i, _)| 1 + i)
        .sum()
}

#[aoc(day13, part2, jorendorff)]
fn part_2(signals: &[Vec<Value>]) -> usize {
    let first = vec![Value::List(vec![Value::Int(2)])];
    let second = vec![Value::List(vec![Value::Int(6)])];

    let first_idx = signals.iter().filter(|s| *s < &first).count() + 1;
    let second_idx = signals.iter().filter(|s| *s < &second).count() + 2;
    first_idx * second_idx
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input_1(EXAMPLE).unwrap()), 13);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input_2(EXAMPLE).unwrap()), 140);
    }
}
