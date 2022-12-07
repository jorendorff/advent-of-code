use std::collections::HashMap;

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

#[derive(Debug)]
enum Listing {
    LsDir(String),
    LsFile(String, u64),
}
use Listing::*;

#[derive(Debug)]
enum Command {
    CdRoot,
    CdUp,
    Cd(String),
    Ls(Vec<Listing>),
}
use Command::*;

type InputLine = Command;

#[aoc_generator(day7, part1, jorendorff)]
#[aoc_generator(day7, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Vec<InputLine>> {
    let p = parser!({
        line("$ cd /") => CdRoot,
        line("$ cd ..") => CdUp,
        (d: line ("$ cd " string(any_char+))) => Cd(d),
        line("$ ls") (output: lines({
            (size: u64) " " (name: string(any_char+)) => LsFile(name, size),
            "dir " (name: string(any_char+)) => LsDir(name),
        })) => Ls(output),
    }*);
    aoc_parse(text, p)
}

#[derive(Debug, Default)]
struct Dir {
    dirs: HashMap<String, Dir>,
    files: HashMap<String, u64>,
    total_file_size: u64,
    deep_total_size: u64,
}

impl Dir {
    fn new() -> Dir {
        Dir::default()
    }

    fn get_mut(&mut self, path: &[String]) -> &mut Dir {
        let mut dir = self;
        for p in path {
            dir = dir
                .dirs
                .get_mut(p)
                .unwrap_or_else(|| panic!("failed to get path {path:?}"));
        }
        dir
    }

    fn compute_sizes(&mut self) -> u64 {
        let mut t = self.total_file_size;
        for d in self.dirs.values_mut() {
            t += d.compute_sizes();
        }
        self.deep_total_size += t;
        t
    }

    fn part1_sum(&self) -> u64 {
        let mut t = 0;
        if self.deep_total_size <= 100000 {
            t += self.deep_total_size;
        }
        for d in self.dirs.values() {
            t += d.part1_sum();
        }
        t
    }

    fn all_dir_sizes(&self, out: &mut Vec<u64>) {
        for d in self.dirs.values() {
            d.all_dir_sizes(out);
        }
        out.push(self.deep_total_size);
    }
}

fn build(input: &[InputLine]) -> Dir {
    let mut root = Dir::new();
    let mut cwd = vec![];
    for cmd in input {
        match cmd {
            CdRoot => {
                cwd.clear();
            }
            CdUp => {
                cwd.pop();
            }
            Cd(name) => {
                cwd.push(name.clone());
            }
            Ls(output) => {
                let d = root.get_mut(&cwd);
                for listing in output {
                    match listing {
                        LsDir(s) => {
                            d.dirs.insert(s.to_string(), Dir::new());
                        }
                        LsFile(name, size) => {
                            d.total_file_size -= d.files.get(name).copied().unwrap_or(0);
                            d.files.insert(name.to_string(), *size);
                            d.total_file_size += *size;
                        }
                    }
                }
            }
        }
    }
    root.compute_sizes();
    root
}

#[aoc(day7, part1, jorendorff)]
fn part_1(input: &[InputLine]) -> u64 {
    build(input).part1_sum()
}

#[aoc(day7, part2, jorendorff)]
fn part_2(input: &[InputLine]) -> u64 {
    let disk_size = 70000000;
    let required_size = 30000000;

    let d = build(input);
    assert!(d.deep_total_size < disk_size);
    assert!(disk_size - d.deep_total_size < required_size);
    let enough = required_size + d.deep_total_size - disk_size;

    let mut sizes = vec![];
    d.all_dir_sizes(&mut sizes);
    sizes
        .into_iter()
        .filter(|size| *size >= enough)
        .min()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 95437);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 24933642);
    }
}
