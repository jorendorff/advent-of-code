use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<Monkey>;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Op {
    Add,
    Mul,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Operand {
    Old,
    Lit(u64),
}

#[derive(Debug, Clone)]
struct Monkey {
    i: usize,
    items: Vec<u64>,
    oper: (Operand, Op, Operand),
    test: u64,
    if_true: usize,
    if_false: usize,
}

#[aoc_generator(day11, part1, jorendorff)]
#[aoc_generator(day11, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let op = parser!({'+' => Op::Add, '*' => Op::Mul});

    let operand = parser!({
        "old" => Operand::Old,
        x:u64 => Operand::Lit(x),
    });

    let p = parser!(sections(
        i:        line("Monkey " usize ":")
        items:    line("  Starting items: " repeat_sep(u64, ", "))
        oper:     line("  Operation: new = " operand ' ' op ' ' operand)
        test:     line("  Test: divisible by " u64)
        if_true:  line("    If true: throw to monkey " usize)
        if_false: line("    If false: throw to monkey " usize)
            => Monkey { i, items, oper, test, if_true, if_false }
    ));
    Ok(p.parse(text)?)
}

fn eval_operand(operand: Operand, old: u64) -> u64 {
    match operand {
        Operand::Old => old,
        Operand::Lit(x) => x,
    }
}

fn eval(oper: (Operand, Op, Operand), old: u64) -> u64 {
    let a = eval_operand(oper.0, old);
    let b = eval_operand(oper.2, old);
    match oper.1 {
        Op::Add => a + b,
        Op::Mul => a * b,
    }
}

#[aoc(day11, part1, jorendorff)]
fn part_1(input: &Input) -> u64 {
    assert!(input.iter().enumerate().all(|(i, monkey)| monkey.i == i));

    let mut monkeys = input.clone();
    let mut inspect_count = vec![0u64; monkeys.len()];
    for _round in 0..20 {
        for i in 0..monkeys.len() {
            //println!("Monkey {i}:");
            let mut items = vec![];
            std::mem::swap(&mut items, &mut monkeys[i].items);
            for item in items {
                //println!("  Monkey inspects an item with a worry level of {item}.");
                let monkey = &mut monkeys[i];
                let new_worry_level = eval(monkey.oper, item);
                //let oper = monkey.oper;
                //println!("    Worry level is {oper:?} to {new_worry_level}.");
                let new_worry_level = new_worry_level / 3;
                //println!("    Monkey gets bored with item. Worry level is divided by 3 to {new_worry_level}.");
                let recip = if new_worry_level % monkey.test == 0 {
                    monkey.if_true
                } else {
                    monkey.if_false
                };
                //println!("    Item with worry level {new_worry_level} is thrown to monkey {recip}.");
                inspect_count[i] += 1;
                monkeys[recip].items.push(new_worry_level);
            }
        }
    }

    inspect_count.sort();
    inspect_count[monkeys.len() - 1] * inspect_count[monkeys.len() - 2]
}

#[aoc(day11, part2, jorendorff)]
fn part_2(input: &Input) -> u64 {
    assert!(input.iter().enumerate().all(|(i, monkey)| monkey.i == i));

    let modulus = input.iter().map(|m| m.test).product::<u64>();

    let mut monkeys = input.clone();
    let mut inspect_count = vec![0u64; monkeys.len()];
    for _round in 0..10_000 {
        for i in 0..monkeys.len() {
            let mut items = vec![];
            std::mem::swap(&mut items, &mut monkeys[i].items);
            for item in items {
                let monkey = &mut monkeys[i];
                let mut new_worry_level = eval(monkey.oper, item);
                let recip = if new_worry_level % monkey.test == 0 {
                    monkey.if_true
                } else {
                    monkey.if_false
                };
                inspect_count[i] += 1;
                new_worry_level %= modulus;
                monkeys[recip].items.push(new_worry_level);
            }
        }
    }

    inspect_count.sort();
    inspect_count[monkeys.len() - 1] * inspect_count[monkeys.len() - 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 10605);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 2713310158);
    }
}
