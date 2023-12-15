use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<String>;

#[aoc_generator(day15, part1, jorendorff)]
fn parse_input_1(text: &str) -> anyhow::Result<Input> {
    let p = parser!(line(
        repeat_sep(string(char_of("abcdefghijklmnopqrstuvwxyz=-0123456789")+), ",")
    ));
    Ok(p.parse(text)?)
}

enum Insn {
    Remove { label: String },
    Insert { label: String, focal_length: u64 },
}

type Input2 = Vec<Insn>;

#[aoc_generator(day15, part2, jorendorff)]
fn parse_input_2(text: &str) -> anyhow::Result<Input2> {
    let p = parser!(line(
        repeat_sep(
            {
                label:string(alpha+) '-' => Insn::Remove { label },
                label:string(alpha+) '=' focal_length:u64 => Insn::Insert { label, focal_length },
            },
            ","
        )
    ));
    Ok(p.parse(text)?)
}

fn hash(s: &str) -> u64 {
    let mut v = 0;
    for c in s.chars() {
        // Determine the ASCII code for the current character of the string.
        // Increase the current value by the ASCII code you just determined.
        v += c as u32;
        // Set the current value to itself multiplied by 17.
        v *= 17;
        // Set the current value to the remainder of dividing itself by 256.
        v %= 256;
    }
    v as u64
}

#[aoc(day15, part1, jorendorff)]
fn part_1(input: &Input) -> u64 {
    input.iter().map(|s| hash(s)).sum()
}

#[aoc(day15, part2, jorendorff)]
fn part_2(input: &Input2) -> u64 {
    // 788 on the global leaderboard
    let mut boxes: Vec<Vec<(String, u64)>> = vec![vec![]; 256];
    for insn in input {
        match insn {
            Insn::Remove { label } => boxes[hash(label) as usize].retain(|entry| &entry.0 != label),
            Insn::Insert {
                label,
                focal_length,
            } => {
                let target = &mut boxes[hash(label) as usize];
                let mut found = false;
                for pair in target.iter_mut() {
                    if &pair.0 == label {
                        pair.1 = *focal_length;
                        found = true;
                        break;
                    }
                }
                if !found {
                    target.push((label.to_string(), *focal_length));
                }
            }
        }
    }

    boxes
        .into_iter()
        .enumerate()
        .map(|(box_index, lenses)| {
            lenses
                .into_iter()
                .enumerate()
                .map(|(slot, (_label, fl))| (box_index + 1) as u64 * (slot + 1) as u64 * fl)
                .sum::<u64>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
";

    #[test]
    fn test_part_1() {
        assert_eq!(hash("HASH"), 52);
        assert_eq!(part_1(&parse_input_1(EXAMPLE).unwrap()), 1320);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input_2(EXAMPLE).unwrap()), 145);
    }
}
