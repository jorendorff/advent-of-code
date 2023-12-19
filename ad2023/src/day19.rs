use std::collections::HashMap;
use std::ops::Range;

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = (RuleSet, Vec<Part>);

#[derive(Clone, PartialEq, Eq)]
enum Rule {
    Gt(usize, u64, String),
    Lt(usize, u64, String),
    Imm(String),
}

struct RuleSet {
    rules: HashMap<String, Vec<Rule>>,
}

type Part = [u64; 4];

#[aoc_generator(day19, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(
        rules:section(lines(
            string(alpha+) "{"
                repeat_sep({
                    a:char_of("xmas") ">" b:u64 ":" s:string(alpha+) => Rule::Gt(a, b, s),
                    a:char_of("xmas") "<" b:u64 ":" s:string(alpha+) => Rule::Lt(a, b, s),
                    s:string(alpha+) => Rule::Imm(s),
                }, ",")
            "}"
        ))
        parts:section(lines(
            "{x=" x:u64 ",m=" m:u64 ",a=" a:u64 ",s=" s:u64 "}" => [x, m, a, s]
        ))
        => (RuleSet { rules: rules.into_iter().collect() }, parts)
    );
    Ok(p.parse(text)?)
}

impl RuleSet {
    fn accept(&self, part: &Part) -> bool {
        let mut name = "in";
        while name != "A" && name != "R" {
            for rule in self.workflow(name) {
                match rule {
                    Rule::Gt(a, b, s) => {
                        if part[*a] > *b {
                            name = s;
                            break;
                        }
                    }
                    Rule::Lt(a, b, s) => {
                        if part[*a] < *b {
                            name = s;
                            break;
                        }
                    }
                    Rule::Imm(s) => {
                        name = s;
                        break;
                    }
                }
            }
        }
        name == "A"
    }

    fn how_many(&self) -> u64 {
        self.how_many_match_name("in", [1..4001, 1..4001, 1..4001, 1..4001])
    }

    fn workflow(&self, name: &str) -> &[Rule] {
        self.rules.get(name).unwrap()
    }

    fn how_many_match_name(&self, name: &str, ranges: [Range<u64>; 4]) -> u64 {
        match name {
            "A" => ranges.into_iter().map(|r| r.end - r.start).product(),
            "R" => 0,
            s => self.how_many_match(self.workflow(s), ranges),
        }
    }

    fn how_many_match(&self, workflow: &[Rule], ranges: [Range<u64>; 4]) -> u64 {
        match &workflow[0] {
            Rule::Gt(a, b, s) => {
                let ra = &ranges[*a];
                if ra.start > *b {
                    // all values in range are > b, all match this rule, ignore rest of range
                    self.how_many_match(self.workflow(s), ranges)
                } else if ra.end <= *b {
                    // no values in this range are > b, none match this rule, move on
                    self.how_many_match(&workflow[1..], ranges)
                } else {
                    let mut non_matching_range = ranges.clone();
                    non_matching_range[*a].end = *b + 1;
                    let mut matching_range = ranges.clone();
                    matching_range[*a].start = *b + 1;
                    self.how_many_match(&workflow[1..], non_matching_range)
                        + self.how_many_match_name(s, matching_range)
                }
            }
            Rule::Lt(a, b, s) => {
                let ra = &ranges[*a];
                if ra.end < *b {
                    // all values in range are < b, all match this rule, ignore rest of range
                    self.how_many_match(self.workflow(s), ranges)
                } else if ra.start >= *b {
                    // no values in this range are < b, none match this rule, move on
                    self.how_many_match(&workflow[1..], ranges)
                } else {
                    let mut matching_range = ranges.clone();
                    matching_range[*a].end = *b;
                    let mut non_matching_range = ranges.clone();
                    non_matching_range[*a].start = *b;
                    self.how_many_match(&workflow[1..], non_matching_range)
                        + self.how_many_match_name(s, matching_range)
                }
            }
            Rule::Imm(s) => self.how_many_match_name(s, ranges),
        }
    }
}

fn rating(part: &[u64; 4]) -> u64 {
    part.iter().copied().sum()
}

#[aoc(day19, part1, jorendorff)]
fn part_1(input: &Input) -> u64 {
    // #359 on the global leaderboard
    let (ruleset, parts) = input;

    parts
        .iter()
        .copied()
        .filter(|part| ruleset.accept(part))
        .map(|part| rating(&part))
        .sum()
}

#[aoc(day19, part2, jorendorff)]
fn part_2(input: &Input) -> u64 {
    // #711 on the global leaderboard
    input.0.how_many()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 19114);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 167409079868000);
    }
}
