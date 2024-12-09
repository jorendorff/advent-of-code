use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<usize>;

#[aoc_generator(day9, part1, jorendorff)]
#[aoc_generator(day9, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(line(digit+));
    Ok(p.parse(text)?)
}

#[aoc(day9, part1, jorendorff)]
fn part_1(input: &Input) -> usize {
    let mut disk = vec![];
    let mut is_file = true;
    let mut file_id = 0usize;
    for &len in input {
        if is_file {
            for _ in 0..len {
                disk.push(Some(file_id));
            }
            file_id += 1;
        } else {
            for _ in 0..len {
                disk.push(None);
            }
        }
        is_file = !is_file;
    }

    let mut checksum = 0;
    let mut left = 0;
    let mut right = disk.len();
    while left < right {
        if disk[right - 1].is_none() {
            right -= 1;
        } else if let Some(id) = disk[left] {
            checksum += id * left;
            left += 1;
        } else {
            disk.swap(left, right - 1);
        }
    }

    checksum
}

#[aoc(day9, part2, jorendorff)]
#[allow(clippy::mut_range_bound)]
fn part_2(input: &Input) -> usize {
    struct Slot {
        file_id: usize,
        file_len: usize,
        gap_after: usize,
    }

    let mut disk = input
        .chunks(2)
        .enumerate()
        .map(|(file_id, pair)| Slot {
            file_id,
            file_len: pair[0],
            gap_after: pair.get(1).copied().unwrap_or(0),
        })
        .collect::<Vec<Slot>>();

    let mut right = disk.len() - 1;

    // defrag disk
    while right > 0 {
        let file_id = disk[right].file_id;
        let file_len = disk[right].file_len;
        for gap_index in 0..right {
            let slot = &mut disk[gap_index];
            if slot.gap_after >= file_len {
                // copy file into the gap
                let new_gap = slot.gap_after - file_len;
                slot.gap_after = 0;
                disk.insert(gap_index + 1, Slot {
                    file_id,
                    file_len,
                    gap_after: new_gap,
                });
                right += 1;

                // remove file from its old location, adding its space to the preceding slot
                let extra = file_len + disk[right].gap_after;
                disk[right - 1].gap_after += extra;
                disk.remove(right);

                break;
            }
        }
        right -= 1;
    }

    // compute checksum
    let mut checksum = 0;
    let mut position = 0;
    for slot in disk {
        for _ in 0..slot.file_len {
            checksum += slot.file_id * position;
            position += 1;
        }
        position += slot.gap_after;
    }

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2333133121414131402";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 1928);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 2858);
    }
}
