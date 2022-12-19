use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<Blueprint>;

#[derive(Debug, Clone)]
struct Blueprint {
    id: usize,
    oo: u64,
    co: u64,
    bo: u64,
    bc: u64,
    go: u64,
    gb: u64,
}

#[aoc_generator(day19, part1, jorendorff)]
#[aoc_generator(day19, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines(
        "Blueprint " id:usize ": "
            "Each ore robot costs " oo:u64 " ore. "
            "Each clay robot costs " co:u64 " ore. "
            "Each obsidian robot costs " bo:u64 " ore and " bc:u64 " clay. "
            "Each geode robot costs " go:u64 " ore and " gb:u64 " obsidian."
            => Blueprint { id, oo, co, bo, bc, go, gb }
    ));
    aoc_parse(text, p)
}

#[derive(Debug, Default, Clone)]
struct State {
    time_left: u64,
    ore: u64,
    ore_robots: u64,
    clay: u64,
    clay_robots: u64,
    obsidian: u64,
    obsidian_robots: u64,
    geodes: u64,
    geode_robots: u64,
}

// From state s, find max number of geodes we can finish in the time we have.
// BUT: if the result would be <= `floor`, it's OK to return 0 instead.
fn search(b: &Blueprint, mut s: State, floor: u64) -> u64 {
    if s.time_left == 0 {
        return s.geodes;
    }

    // Prune if we can't possibly beat `floor`.
    if s.geodes + s.time_left * s.geode_robots + s.time_left * (s.time_left - 1) / 2 <= floor {
        return 0;
    }

    // Only try ore if we (a) can afford it AND (b) don't already have as many
    // ore robots as we could possibly ever need.
    let try_ore = s.ore >= b.oo && s.ore_robots < b.co.max(b.bo).max(b.go);
    let try_clay = s.ore >= b.co;
    let try_obsidian = s.ore >= b.bo && s.clay >= b.bc;
    let try_geode = s.ore >= b.go && s.obsidian >= b.gb;

    s.time_left -= 1;
    s.ore += s.ore_robots;
    s.clay += s.clay_robots;
    s.obsidian += s.obsidian_robots;
    s.geodes += s.geode_robots;

    // Try building a geode robot first, since that's virtually guaranteed to
    // be the best choice; maxing out `best` early improves the `floor` for the
    // other moves, a huge speedup.
    let mut best = floor;
    if try_geode {
        let mut s = s.clone();
        s.ore -= b.go;
        s.obsidian -= b.gb;
        s.geode_robots += 1;
        best = best.max(search(b, s, best));
    }
    if try_obsidian {
        let mut s = s.clone();
        s.ore -= b.bo;
        s.clay -= b.bc;
        s.obsidian_robots += 1;
        best = best.max(search(b, s, best));
    }
    if try_clay {
        let mut s = s.clone();
        s.ore -= b.co;
        s.clay_robots += 1;
        best = best.max(search(b, s, best));
    }
    if try_ore {
        let mut s = s.clone();
        s.ore -= b.oo;
        s.ore_robots += 1;
        best = best.max(search(b, s, best));
    }
    best = best.max(search(b, s.clone(), best));
    best
}

fn max_geodes(b: &Blueprint, time_left: u64) -> u64 {
    let state = State {
        time_left,
        ore_robots: 1,
        ..State::default()
    };
    search(b, state, 0)
}

#[aoc(day19, part1, jorendorff)]
fn part_1(input: &Input) -> u64 {
    // Rank 22 on this star's leaderboard.
    input
        .iter()
        .map(|blueprint| blueprint.id as u64 * max_geodes(blueprint, 24))
        .sum()
}

#[aoc(day19, part2, jorendorff)]
fn part_2(input: &Input) -> u64 {
    // Rank 11 on this star's leaderboard.
    input[0..3]
        .iter()
        .map(|blueprint| max_geodes(blueprint, 32))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 33);
    }

    #[test]
    fn test_part_2() {
        let blueprints = parse_input(EXAMPLE).unwrap();
        assert_eq!(max_geodes(&blueprints[0], 32), 56);
        assert_eq!(max_geodes(&blueprints[1], 32), 62);
    }
}
