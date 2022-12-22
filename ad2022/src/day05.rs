use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

#[derive(Debug, Clone)]
struct Input {
    model: Model,
    moves: Vec<Move>,
}

#[derive(Debug, Clone)]
struct Model {
    stacks: Vec<Vec<char>>,
}

impl Model {
    fn apply_move(&mut self, m: Move) {
        for _ in 0..m.quantity {
            let c = self.stacks[m.source].pop().unwrap();
            self.stacks[m.target].push(c);
        }
    }

    fn apply_move_9001(&mut self, m: Move) {
        let n = self.stacks[m.source].len();
        let crates = self.stacks[m.source].split_off(n - m.quantity);
        self.stacks[m.target].extend(crates);
    }

    fn tops(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack[stack.len() - 1])
            .collect()
    }
}

#[derive(Debug, Clone)]
struct Move {
    quantity: usize,
    source: usize,
    target: usize,
}

#[aoc_generator(day5, part1, jorendorff)]
#[aoc_generator(day5, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let move_parser = parser!(
        "move " quantity:usize " from " source:usize " to " target:usize
            =>
            Move { quantity, source: source - 1, target: target - 1 }
    );

    let model = parser!(
        rows:lines(
            repeat_sep(
                {
                    "   " => None,
                    "[" x:alpha "]" => Some(x),
                },
                " "
            )
        )
        =>
        {
            let mut stacks = vec![];
            stacks.resize(rows[0].len(), vec![]);
            for row in rows.iter().rev() {
                for (i, c) in row.iter().copied().enumerate() {
                    if let Some(c) = c {
                        stacks[i].push(c);
                    }
                }
            }
            Model { stacks }
        }
    );

    let p = parser!(
        m:model
        line(repeat_sep(" " digit " ", " "))
        line("")
        moves:lines(move_parser)
        =>
        {
            Input { model: m, moves }
        }
    );
    Ok(p.parse(text)?)
}

#[aoc(day5, part1, jorendorff)]
fn part_1(inp: &Input) -> String {
    let mut inp = inp.clone();

    for m in inp.moves {
        inp.model.apply_move(m);
    }

    inp.model.tops()
}

#[aoc(day5, part2, jorendorff)]
fn part_2(inp: &Input) -> String {
    let mut inp = inp.clone();

    for m in inp.moves {
        inp.model.apply_move_9001(m);
    }

    inp.model.tops()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), "CMZ");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), "MCD");
    }
}
