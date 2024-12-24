// Part 1 rank 545, Part 2 rank 202

use std::collections::{HashMap, HashSet};

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = (HashMap<String, u8>, HashMap<String, (String, Op, String)>);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Op {
    And,
    Xor,
    Or
}

#[aoc_generator(day24, part1, jorendorff)]
#[aoc_generator(day24, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let ident = parser!(string(alnum+));
    let op = parser!({
        "AND" => Op::And,
        "XOR" => Op::Xor,
        "OR" => Op::Or,
    });
    let p = parser!(
        section(hash_map(lines(
            ident ": " u8
        )))
        section(hash_map(lines(
            l:ident ' ' op:op ' ' r:ident " -> " out:ident => (out, (l, op, r))
        )))
    );
    Ok(p.parse(text)?)
}

#[aoc(day24, part1, jorendorff)]
fn part_1(input: &Input) -> u64 {
    let (inputs, rules) = input;
    let mut inputs = inputs.clone();
    loop {
        let mut any = false;
        for (out, (lhs, op, rhs)) in rules {
            if !inputs.contains_key(out) {
                if let (Some(&lv), Some(&rv)) = (inputs.get(lhs), inputs.get(rhs)) {
                    let out_v = match op {
                        Op::And => lv & rv,
                        Op::Xor => lv ^ rv,
                        Op::Or => lv | rv,
                    };
                    inputs.insert(out.clone(), out_v);
                    any = true;
                }
            }
        }
        if !any {
            break;
        }
    }
    let mut ans = 0;
    for i in 0u32.. {
        if let Some(&v) = inputs.get(&format!("z{i:02}")) {
            ans |= (v as u64) << i;
        } else {
            break;
        }
    }
    ans
}

#[aoc(day24, part2, jorendorff)]
fn part_2(input: &Input) -> String {
    let (_inputs, rules) = input;

    let mut output = String::new();
    let mut seen = HashSet::new();
    for i in 0..=45 {
        let mut round = vec![];
        let mut todo = vec![format!("z{i:02}")];
        seen.insert(todo[0].clone());
        while let Some(name) = todo.pop() {
            let Some((lhs, op, rhs)) = rules.get(&name) else {
                continue;
            };
            if !seen.contains(lhs) {
                seen.insert(lhs.clone());
                todo.push(lhs.clone());
            }
            if !seen.contains(rhs) {
                seen.insert(rhs.clone());
                todo.push(rhs.clone());
            }
            let mut operands = [lhs.as_str(), rhs.as_str()];
            operands.sort();
            round.push(format!("{op:?} {} -> {name}\n", operands.join(" ")));
        }

        round.reverse();
        let n = round.len();
        round[..n - 1].sort_by_key(|s| s.split_whitespace().next().unwrap().to_string());
        output += &round.join("");
        output += "\n";
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
";

    const EXAMPLE2: &str = "\
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 4);
        assert_eq!(part_1(&parse_input(EXAMPLE2).unwrap()), 2024);
        
    }
}
