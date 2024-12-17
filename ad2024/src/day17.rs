// Part 1 rank 623, part 2 rank 61

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = ((i64, i64, i64), Vec<u8>);

#[aoc_generator(day17, part1, jorendorff)]
#[aoc_generator(day17, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(
        section(
            line("Register A: " i64)
            line("Register B: " i64)
            line("Register C: " i64)
        )
        section(line("Program: " repeat_sep(u8, ',')))
    );
    Ok(p.parse(text)?)
}

fn combo(op: u8, (a, b, c): (i64, i64, i64)) -> i64 {
    match op {
        0..=3 => op.into(),
        4 => a,
        5 => b,
        6 => c,
        _ => panic!("reserved"),
    }
}

fn run((mut a, mut b, mut c): (i64, i64, i64), program: &[u8]) -> Vec<u8> {
    let mut output = vec![];
    let mut ip = 0;
    while let Some(&opcode) = program.get(ip) {
        ip += 1;
        match opcode {
            0 => { a >>= combo(program[ip], (a, b, c)); }
            1 => { b ^= program[ip] as i64; }
            2 => { b = combo(program[ip], (a, b, c)) % 8; }
            3 => if a != 0 { ip = program[ip] as usize; continue; }
            4 => { b ^= c; }
            5 => { output.push(combo(program[ip], (a, b, c)) as u8 % 8); }
            6 => { b = a >> combo(program[ip], (a, b, c)); }
            7 => { c = a >> combo(program[ip], (a, b, c)); }
            _ => panic!("invalid opcode"),
        }
        ip += 1;
    }
    output
}

#[aoc(day17, part1, jorendorff)]
fn part_1(input: &Input) -> String {
    let ((a, b, c), program) = input.clone();
    let out = run((a, b, c), &program);
    let out = out.into_iter().map(|v| v.to_string()).collect::<Vec<String>>();
    out.join(",")
}

fn search(a: i64, b: i64, c: i64, program: &[u8], len: usize) -> Option<i64> {
    if len == program.len() + 1 {
        return Some(a);
    }

    let mut answers = vec![];
    for word in 0..8 {
        if run(((a << 3) | word, b, c), program) == program[program.len() - len..] {
            if let Some(a) = search((a << 3) | word, b, c, program, len + 1) {
                answers.push(a);
            }
        }
    }
    answers.into_iter().min()
}

#[aoc(day17, part2, jorendorff)]
fn part_2(input: &Input) -> i64 {
    let ((_a, b, c), program) = input.clone();
    let a = search(0, b, c, &program, 1).unwrap();
    assert_eq!(run((a, b, c), &program), program);
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), "4,6,3,5,6,3,5,2,1,0");
    }

    const EXAMPLE2: &str = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE2).unwrap()), 117440);
    }
}
