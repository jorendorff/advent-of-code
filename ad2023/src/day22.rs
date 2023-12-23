use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy)]
struct Point(u64, u64, u64);

type Input = Vec<(Point, Point)>;

#[aoc_generator(day22, part1, jorendorff)]
#[aoc_generator(day22, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let point = parser!(x:u64 "," y:u64 "," z:u64 => Point(x, y, z));
    let p = parser!(lines(
        point "~" point
    ));
    Ok(p.parse(text)?)
}

fn brick_name(id: usize) -> String {
    const NAMES: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    if id < NAMES.len() {
        NAMES[id..id + 1].to_string()
    } else {
        format!("B{id}")
    }
}

#[aoc(day22, part1, jorendorff)]
fn part_1(input: &Input) -> usize {
    let mut bricks = input
        .iter()
        .enumerate()
        .map(|(i, &(p, q))| (i, p, q))
        .collect::<Vec<_>>();
    bricks.sort_by_key(|(_i, p, q)| p.2.min(q.2));
    let mut map: HashMap<(u64, u64), (usize, u64)> = HashMap::new();

    // (i, xs) means i is atop all of xs
    let mut deps: HashMap<usize, HashSet<usize>> = HashMap::new();

    for &(i, p, q) in bricks.iter() {
        println!("Brick {} {p:?}~{q:?}", brick_name(i));

        let Point(xlo, ylo, _) = p;
        let Point(xhi, yhi, _) = q;
        assert!(xlo <= xhi);
        assert!(ylo <= yhi);
        let floor: u64 = (xlo..=xhi)
            .flat_map(|x| {
                let my_map = &map;
                (ylo..=yhi).map(move |y| -> u64 {
                    my_map.get(&(x, y)).copied().unwrap_or((usize::MAX, 0)).1
                })
            })
            .max()
            .unwrap();

        println!("Brick {} ends up at z={}", brick_name(i), floor + 1);

        let top = floor + p.2.abs_diff(q.2) + 1;
        for x in xlo..=xhi {
            for y in ylo..=yhi {
                if let Some(&(piece, ht)) = map.get(&(x, y)) {
                    if ht == floor {
                        println!(
                            "Brick {} is resting on brick {}",
                            brick_name(i),
                            brick_name(piece)
                        );
                        deps.entry(i).or_default().insert(piece);
                    }
                }
                map.insert((x, y), (i, top));
            }
        }
    }

    let mut has_sole_dependees = vec![false; bricks.len()];
    for (i, xs) in deps {
        if xs.len() == 1 {
            let supporter = xs.into_iter().next().unwrap();
            has_sole_dependees[supporter] = true;
            println!(
                "Brick {} cannot be disintegrated; brick {} would fall",
                brick_name(supporter),
                brick_name(i)
            );
        }
    }

    has_sole_dependees.into_iter().filter(|b| !*b).count()
}

#[aoc(day22, part2, jorendorff)]
fn part_2(input: &Input) -> usize {
    let mut bricks = input
        .iter()
        .enumerate()
        .map(|(i, &(p, q))| (i, p, q))
        .collect::<Vec<_>>();
    bricks.sort_by_key(|(_i, p, q)| p.2.min(q.2));
    let mut map: HashMap<(u64, u64), (usize, u64)> = HashMap::new();

    // (i, xs) means i is supported by the bricks in xs
    let mut deps: HashMap<usize, HashSet<usize>> = HashMap::new();

    // (i, xs) means i supports each of the bricks in xs (the inverse relation)
    let mut supports: HashMap<usize, HashSet<usize>> = HashMap::new();

    for &(i, p, q) in bricks.iter() {
        println!("Brick {} {p:?}~{q:?}", brick_name(i));

        let Point(xlo, ylo, _) = p;
        let Point(xhi, yhi, _) = q;
        assert!(xlo <= xhi);
        assert!(ylo <= yhi);
        let floor: u64 = (xlo..=xhi)
            .flat_map(|x| {
                let my_map = &map;
                (ylo..=yhi).map(move |y| -> u64 {
                    my_map.get(&(x, y)).copied().unwrap_or((usize::MAX, 0)).1
                })
            })
            .max()
            .unwrap();

        println!("Brick {} ends up at z={}", brick_name(i), floor + 1);

        let top = floor + p.2.abs_diff(q.2) + 1;
        for x in xlo..=xhi {
            for y in ylo..=yhi {
                if let Some(&(piece, ht)) = map.get(&(x, y)) {
                    if ht == floor {
                        println!(
                            "Brick {} is resting on brick {}",
                            brick_name(i),
                            brick_name(piece)
                        );
                        deps.entry(i).or_default().insert(piece);
                        supports.entry(piece).or_default().insert(i);
                    }
                }
                map.insert((x, y), (i, top));
            }
        }
    }

    bricks
        .iter()
        .map(|&(i, _p, _q)| how_many_would_fall(deps.clone(), supports.clone(), i))
        .sum()
}

fn how_many_would_fall(
    mut deps: HashMap<usize, HashSet<usize>>,
    mut supports: HashMap<usize, HashSet<usize>>,
    i: usize,
) -> usize {
    let mut todo = VecDeque::new();
    todo.push_back(i);

    let mut fall_count = 0;

    while let Some(n) = todo.pop_front() {
        let supportees = supports.remove(&n).unwrap_or_default();
        for k in supportees {
            let k_supporters = deps.get_mut(&k).unwrap();
            k_supporters.remove(&n);
            if k_supporters.is_empty() {
                todo.push_back(k);
                fall_count += 1;
            }
        }
    }
    fall_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 5);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 7);
    }
}
