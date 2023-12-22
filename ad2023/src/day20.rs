use std::collections::*;

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

#[derive(Clone)]
struct Machine {
    modules: BTreeMap<String, Module>,
    hi_count: u64,
    lo_count: u64,
    mg_signaled: bool,
}

#[derive(Clone)]
struct Module {
    kind: ModuleKind,
    dest: Vec<String>,
}

#[derive(Clone)]
enum ModuleKind {
    FlipFlop { is_on: bool },
    Conjunction { memory: BTreeMap<String, bool> },
    Broadcaster,
}

use ModuleKind::*;

type Input = Machine;

#[aoc_generator(day20, part1, jorendorff)]
#[aoc_generator(day20, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(btree_map(lines({
        '%' name:string(alpha+) " -> " dest:repeat_sep(string(alpha+), ", ")
            => (name, Module { kind: FlipFlop { is_on: false }, dest }),
        '&' name:string(alpha+) " -> " dest:repeat_sep(string(alpha+), ", ")
            => (name, Module { kind: Conjunction { memory: BTreeMap::new() }, dest }),
        "broadcaster -> " dest:repeat_sep(string(alpha+), ", ")
            => ("broadcaster".to_string(), Module { kind: Broadcaster, dest })
    })));
    Ok(Machine { modules: p.parse(text)?, lo_count: 0, hi_count: 0, mg_signaled: false })
}


impl ModuleKind {
    fn dump_state(&self, out: &mut Vec<bool>) {
        match self {
            FlipFlop { is_on } => out.push(*is_on),
            Conjunction { memory } => {
                for v in memory.values() { out.push(*v); }
            }
            Broadcaster => {}
        }
    }
}

impl Machine {
    fn init(&mut self) {
        // Initialize all Conjunctions to have low for all inputs
        for (name, module) in self.modules.clone() {
            for d in module.dest {
                if let Some(Module { kind: Conjunction { memory }, ..}) = &mut self.modules.get_mut(&d) {
                    memory.insert(name.clone(), false);
                }
            }
        }
    }

    fn dump_state(&self) -> Vec<bool> {
        let mut v = vec![];
        for m in self.modules.values() {
            m.kind.dump_state(&mut v);
        }
        v
    }

    fn push_button(&mut self) {
        self.mg_signaled = false;
        let mut q: VecDeque<_> = [("button".to_string(), "broadcaster".to_string(), false)].into_iter().collect();

        while let Some((src, target, is_high)) = q.pop_front() {
            if target == "mg" && is_high {
                println!("{src} -{}-> {target}", if is_high { "high" } else { "low" });
                self.mg_signaled = true;
            }
            if is_high {
                self.hi_count += 1;
            } else {
                self.lo_count += 1;
            }
            match self.modules.get_mut(&target) {
                Some(Module { kind: FlipFlop { is_on }, dest }) => {
                    if !is_high {
                        *is_on = !*is_on;
                        for d in dest {
                            q.push_back((target.clone(), d.clone(), *is_on));
                        }
                    }
                }
                Some(Module { kind: Conjunction { memory }, dest }) => {
                    memory.insert(src, is_high);
                    let send_high = !memory.values().all(|v| *v);
                    for d in dest {
                        q.push_back((target.clone(), d.clone(), send_high));
                    }
                }
                Some(Module { kind: Broadcaster, dest }) => {
                    for d in dest {
                        q.push_back((target.clone(), d.clone(), is_high));
                    }
                }
                None => {}
            }
        }
    }
}

#[aoc(day20, part1, jorendorff)]
fn part_1(input: &Input) -> u64 {
    let mut machine = input.clone();
    machine.init();
    for _ in 0..1000 {
        machine.push_button();
    }
    machine.lo_count * machine.hi_count
}

const MACHINE_1: &str = "\
broadcaster -> ht
%ht -> vp, nt
%vp -> qj, nt
%qj -> hj
%hj -> mq
%mq -> gz, nt
%gz -> ds, nt
%ds -> bd
%bd -> lj, nt
%lj -> cx, nt
%cx -> xr, nt
%xr -> xd, nt
%xd -> nt
&nt -> ds, hj, ht, rh, qj
&rh -> mg
";

const MACHINE_2: &str = "\
broadcaster -> gb
%gb -> fx, th
%fx -> th, bn
%bn -> fz
%fz -> bk, th
%bk -> zg
%zg -> th, zq
%zq -> tt, th
%tt -> pq
%pq -> th, mj
%mj -> th, sq
%sq -> th, cd
%cd -> th
&th -> bn, gb, tt, hf, bk
&hf -> mg
";

const MACHINE_3: &str = "\
broadcaster -> vk
%vk -> mp, ff
%mp -> bq, ff
%bq -> pr
%pr -> ql
%ql -> lt
%lt -> vd, ff
%vd -> xf
%xf -> db, ff
%db -> dz, ff
%dz -> gd, ff
%gd -> cv, ff
%cv -> ff
&ff -> vd, bq, pr, vk, ql, jm
&jm -> mg
";

const MACHINE_4: &str = "\
broadcaster -> zz
%zz -> rz, zs
%rz -> zc
%zc -> dh
%dh -> sh
%sh -> mr, zs
%mr -> bf
%bf -> gg, zs
%gg -> pj, zs
%pj -> xs
%xs -> dx, zs
%dx -> bm, zs
%bm -> zs
&zs -> mr, pj, zz, dh, jg, zc, rz
&jg -> mg
";

fn analyze_machine(desc: &str) -> u64 {
    println!("BEGIN");
    let mut machine = parse_input(desc).unwrap();
    machine.init();

    let mut seen = HashMap::new();
    seen.insert(machine.dump_state(), 0);
    for i in 1.. {
        machine.push_button();
        if machine.mg_signaled {
            println!("After {i} presses, mg signaled high");
        }
        let s = machine.dump_state();
        if let Some(v) = seen.get(&s) {
            println!("After {} presses, at same state as after {v}", i);
            println!();
            return i - 1;
        }
        seen.insert(s, i);
    }
    unreachable!();
}

#[aoc(day20, part2, jorendorff)]
fn part_2(_input: &Input) -> u64 {
    let a = analyze_machine(MACHINE_1);
    let b = analyze_machine(MACHINE_2);
    let c = analyze_machine(MACHINE_3);
    let d = analyze_machine(MACHINE_4);
    num::integer::lcm(num::integer::lcm(num::integer::lcm(a, b), c), d)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";

    const EXAMPLE_2: &str = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 32000000);
        assert_eq!(part_1(&parse_input(EXAMPLE_2).unwrap()), 11687500);
    }

    #[test]
    fn test_part_2() {
    }
}
