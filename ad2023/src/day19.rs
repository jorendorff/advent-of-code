use std::collections::HashMap;

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

impl RuleSet {
    fn accept(&self, part: &Part) -> bool {
        let mut name = "in";
        while name != "A" && name != "R" {
            for rule in self.rules.get(name).unwrap() {
                match rule {
                    Rule::Gt(a, b, s) => if part.get(*a) > *b { name = s; break; }
                    Rule::Lt(a, b, s) => if part.get(*a) < *b { name = s; break; }
                    Rule::Imm(s) => {name = s; break; }
                }
            }
        }
        name == "A"
    }

    fn how_many(&self) -> u64 {
        // let mut break_points = [vec![1, 4001], vec![1, 4001], vec![1, 4001], vec![1, 4001]];
        // 
        // for rules in self.rules.values() {
        //     for rule in rules {
        //         match rule {
        //             Rule::Gt(a, b, _) => break_points[*a].push(*b + 1),
        //             Rule::Lt(a, b, _) => break_points[*a].push(*b),
        //             _ => {}
        //         }
        //     }
        // }
        // 
        // for v in &mut break_points {
        //     v.sort();
        //     v.dedup();
        // }
        // 
        // println!("{} {} {} {}", break_points[0].len(), break_points[1].len(), break_points[2].len(), break_points[3].len());
        // let mut total = 0;
        // 
        // for xr in break_points[0].windows(2) {
        //     let (x0, x1) = (xr[0], xr[1]);
        //     let xw = x1 - x0;
        //     for mr in break_points[1].windows(2) {
        //         let (m0, m1) = (mr[0], mr[1]);
        //         let mw = xw * (m1 - m0);
        //         for ar in break_points[1].windows(2) {
        //             let (a0, a1) = (ar[0], ar[1]);
        //             let aw = mw * (a1 - a0);
        //             for sr in break_points[1].windows(2) {
        //                 let (s0, s1) = (sr[0], sr[1]);
        //                 if self.accept(&Part { x: x0, m: m0, a: a0, s: s0 }) {
        //                     total += aw * (s1 - s0);
        //                 }
        //             }
        //         }
        //     }
        // }
        // 
        // total
        self.how_many_match(self.rules.get("in").unwrap(), [(1, 4001); 4])
    }

    fn workflow(&self, name: &str) -> &[Rule] {
        self.rules.get(name).unwrap()
    }

    fn how_many_match_name(&self, name: &str, ranges: [(u64, u64); 4]) -> u64 {
        match name {
            "A" => ranges.into_iter().map(|(v0, v1)| v1 - v0).product(),
            "R" => 0,
            s => self.how_many_match(self.workflow(s), ranges),
        }
    }
    
    fn how_many_match(&self, workflow: &[Rule], ranges: [(u64, u64); 4]) -> u64 {
        match &workflow[0] {
            Rule::Gt(a, b, s) => {
                let (a0, a1) = ranges[*a];
                if a0 > *b {
                    // all values in range are > b, all match this rule, ignore rest of range
                    self.how_many_match(self.workflow(s), ranges)
                } else if a1 <= *b {
                    // no values in this range are > b, none match this rule, move on
                    self.how_many_match(&workflow[1..], ranges)
                } else {
                    let mut non_matching_range = ranges;
                    non_matching_range[*a].1 = *b + 1;
                    let mut matching_range = ranges;
                    matching_range[*a].0 = *b + 1;
                    self.how_many_match(&workflow[1..], non_matching_range) + self.how_many_match_name(s, matching_range)
                }
            }
            Rule::Lt(a, b, s) => {
                let (a0, a1) = ranges[*a];
                if a1 < *b {
                    // all values in range are < b, all match this rule, ignore rest of range
                    self.how_many_match(self.workflow(s), ranges)
                } else if a0 >= *b {
                    // no values in this range are < b, none match this rule, move on
                    self.how_many_match(&workflow[1..], ranges)
                } else {
                    let mut matching_range = ranges;
                    matching_range[*a].1 = *b;
                    let mut non_matching_range = ranges;
                    non_matching_range[*a].0 = *b;
                    self.how_many_match(&workflow[1..], non_matching_range) + self.how_many_match_name(s, matching_range)
                }
            }
            Rule::Imm(s) => self.how_many_match_name(s, ranges),
        }
    }
        
//      fn combos(&self, symbol: &str, start: usize, mut range: PartRange) -> u64 {
//          for rule in &self.rules.get(symbol).unwrap()[start..] {
//              match rule {
//                  Rule::Gt(a, b, s) => {
//                      let r = range.get(*a);
//                      if r.end <= *b {
//                          // all values are <= b, none match this rule; continue to next rule
//                      } else if r.start <= *b {
//                          // some values are <= b, others are > b
//                          let under = self.combos();
//                      } else {
//                          
//                      }
//                  }
//                  Rule::Lt(a, b, s) => panic!(),
//                  Rule::Imm(s) => return self.combos(s),
//              }
//          }
//  }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}


// #[derive(Clone)]
// struct PartRange {
//     x: Range<u64>,
//     m: Range<u64>,
//     a: Range<u64>,
//     s: Range<u64>,
// }

impl Part {
    fn rating(&self) -> u64 { self.x+self.m+self.a+self.s}

    fn get(&self, attr: usize) -> u64 {
        [self.x, self.m, self.a, self.s][attr]
    }
}

// impl PartRange {
//     fn get(&mut self, attr: usize) -> &mut Range<u64> {
//         [&mut self.x, &mut self.m, &mut self.a, &mut self.s][attr]
//     }
// }

#[aoc_generator(day19, part1, jorendorff)]
#[aoc_generator(day19, part2, jorendorff)]
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
            "{x=" x:u64 ",m=" m:u64 ",a=" a:u64 ",s=" s:u64 "}" => Part {x, m, a, s}
        ))
            => (RuleSet { rules: rules.into_iter().collect() }, parts)
    );
    Ok(p.parse(text)?)
}

#[aoc(day19, part1, jorendorff)]
fn part_1(input: &Input) -> u64 {
    // #359 on the global leaderboard
    let (ruleset, parts) = input;

    parts.iter().copied().filter(|part| {
        ruleset.accept(part)
    }).map(|part| part.rating())
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
