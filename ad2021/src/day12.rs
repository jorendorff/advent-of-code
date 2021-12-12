use std::collections::HashMap;

use aoc_runner_derive::*;

const START: usize = 0;
const END: usize = 1;

struct Room {
    name: String,
    large: bool,
    adj: Vec<usize>,
}

#[aoc_generator(day12, part1, jorendorff)]
#[aoc_generator(day12, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Vec<Room>> {
    let mut names: HashMap<String, usize> = HashMap::new();
    let mut rooms: Vec<Room> = vec![];

    let name_to_id =
        |names: &mut HashMap<String, usize>, rooms: &mut Vec<Room>, name: &str| -> usize {
            *names.entry(name.to_string()).or_insert_with(|| {
                let n = rooms.len();
                rooms.push(Room {
                    name: name.to_string(),
                    large: name.to_uppercase() == name,
                    adj: vec![],
                });
                n
            })
        };

    assert_eq!(name_to_id(&mut names, &mut rooms, "start"), START);
    assert_eq!(name_to_id(&mut names, &mut rooms, "end"), END);

    for line in text.lines() {
        let bits = line.split('-').collect::<Vec<&str>>();
        anyhow::ensure!(bits.len() == 2);
        let origin = name_to_id(&mut names, &mut rooms, bits[0]);
        let dest = name_to_id(&mut names, &mut rooms, bits[1]);
        rooms[origin].adj.push(dest);
        rooms[dest].adj.push(origin);
    }

    Ok(rooms)
}

fn solve(rooms: &[Room], can_revisit: bool) -> u64 {
    let mut visited = vec![0; rooms.len()];
    visited[START] = 1;
    let mut any_small_visited_multi = false;
    let mut count = 0;
    let mut breadcrumbs = vec![(START, 0)];
    while let Some((i, j)) = breadcrumbs.pop() {
        if j == rooms[i].adj.len() {
            visited[i] -= 1;
            if !rooms[i].large && visited[i] == 1 {
                any_small_visited_multi = false;
            }
        } else {
            breadcrumbs.push((i, j + 1));
            let next = rooms[i].adj[j];
            if next == END {
                count += 1;
            } else if rooms[next].large
                || visited[next] == 0
                || (can_revisit && next != START && !any_small_visited_multi && visited[next] == 1)
            {
                visited[next] += 1;
                breadcrumbs.push((next, 0));
                if !rooms[next].large && visited[next] == 2 {
                    any_small_visited_multi = true;
                }
            }
        }
    }
    count
}

#[aoc(day12, part1, jorendorff)]
fn part_1(rooms: &[Room]) -> u64 {
    solve(rooms, false)
}

#[aoc(day12, part2, jorendorff)]
fn part_2(rooms: &[Room]) -> u64 {
    solve(rooms, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end
";

    const EXAMPLE2: &str = "\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
";

    const EXAMPLE3: &str = "\
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 10);
        assert_eq!(part_1(&parse_input(EXAMPLE2).unwrap()), 19);
        assert_eq!(part_1(&parse_input(EXAMPLE3).unwrap()), 226);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 36);
        assert_eq!(part_2(&parse_input(EXAMPLE2).unwrap()), 103);
        assert_eq!(part_2(&parse_input(EXAMPLE3).unwrap()), 3509);
    }
}
