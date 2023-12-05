use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

struct Game {
    id: usize,
    draws: Vec<Vec<(usize, String)>>,
}

#[aoc_generator(day2, part1, jorendorff)]
#[aoc_generator(day2, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Vec<Game>> {
    let p = parser!(
        lines(
            "Game " id:usize ": "
                draws:repeat_sep(repeat_sep(usize ' ' string(alpha+), ", "), "; ")
                => Game { id, draws }
        )
    );
    Ok(p.parse(text)?)
}

fn is_possible(game: &Game) -> bool {
    game.draws.iter().all(|draw| {
        draw.iter().all(|(count, color)| {
            *count
                <= match color.as_str() {
                    "red" => 12,
                    "green" => 13,
                    "blue" => 14,
                    _ => 0,
                }
        })
    })
}

#[aoc(day2, part1, jorendorff)]
fn part_1(input: &[Game]) -> usize {
    // Got 815 on the global leaderboard!
    input.iter().filter(|g| is_possible(g)).map(|g| g.id).sum()
}

fn power(game: &Game) -> u64 {
    use std::collections::HashMap;
    let mut maxes: HashMap<String, u64> = ["red", "green", "blue"]
        .into_iter()
        .map(|s| (s.to_string(), 0))
        .collect();
    for draw in &game.draws {
        for (count, color) in draw {
            let entry = maxes.get_mut(color).unwrap();
            *entry = (*entry).max(*count as u64);
        }
    }
    maxes["red"] * maxes["green"] * maxes["blue"]
}

#[aoc(day2, part2, jorendorff)]
fn part_2(input: &[Game]) -> u64 {
    input.iter().map(power).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 8);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 2286);
    }
}
