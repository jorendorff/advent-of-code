use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

struct Map {
    mappings: Vec<Mapping>,
}

struct Mapping {
    destination_start: usize,
    source_start: usize,
    len: usize,
}

impl Map {
    fn apply(&self, source: usize) -> usize {
        for mapping in &self.mappings {
            if (mapping.source_start..mapping.source_start + mapping.len).contains(&source) {
                return mapping.destination_start + (source - mapping.source_start);
            }
        }
        source
    }
}

#[aoc_generator(day5, part1, jorendorff)]
#[aoc_generator(day5, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Almanac> {
    let p = parser!(
        seeds:section(line("seeds: " repeat_sep(usize, " ")))
        maps:sections(
            string(alpha+) "-to-" string(alpha+) " map:\n"
            mappings:lines(
                destination_start: usize " " source_start:usize " " len:usize
                    => Mapping { destination_start, source_start, len }
            )
            => Map { mappings }
        )
        => Almanac { seeds, maps }
    );
    Ok(p.parse(text)?)
}

#[aoc(day5, part1, jorendorff)]
fn part_1(input: &Almanac) -> usize {
    // 395 on the global leaderboard
    input
        .seeds
        .iter()
        .copied()
        .map(|seed| {
            let mut i = seed;
            for map in &input.maps {
                i = map.apply(i);
            }
            i
        })
        .min()
        .unwrap()
}

impl Almanac {
    // find minimum location number that corresponds to any numbers in start..stop at layer
    fn solve(&self, layer: usize, mut start: usize, stop: usize) -> usize {
        assert!(start < stop);
        if layer == self.maps.len() {
            return start;
        }
        let mut best = usize::MAX;
        'outer_loop: while start < stop {
            for mapping in &self.maps[layer].mappings {
                if (mapping.source_start..mapping.source_start + mapping.len).contains(&start) {
                    let slice_stop = stop.min(mapping.source_start + mapping.len);
                    let out = self.solve(
                        layer + 1,
                        mapping.destination_start + (start - mapping.source_start),
                        mapping.destination_start + (slice_stop - mapping.source_start),
                    );
                    if out < best {
                        best = out;
                    }
                    start = slice_stop;
                    continue 'outer_loop;
                }
            }

            let slice_stop = match self.maps[layer]
                .mappings
                .iter()
                .filter(|mapping| mapping.source_start > start && mapping.len > 0)
                .map(|mapping| mapping.source_start)
                .min()
            {
                Some(next_mapping_start) => next_mapping_start.min(stop),
                None => stop,
            };

            let out = self.solve(layer + 1, start, slice_stop);
            if out < best {
                best = out;
            }
            start = slice_stop;
        }

        best
    }
}

#[aoc(day5, part2, jorendorff)]
fn part_2(input: &Almanac) -> usize {
    // 103 on the global leaderboard
    input
        .seeds
        .chunks_exact(2)
        .map(|chunk| {
            let start = chunk[0];
            let len = chunk[1];
            input.solve(0, start, start + len)
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 35);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 46);
    }
}
