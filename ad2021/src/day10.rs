use aoc_runner_derive::*;

#[aoc_generator(day10, part1, jorendorff)]
#[aoc_generator(day10, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Vec<String>> {
    Ok(text.lines().map(str::to_string).collect())
}

#[derive(Clone, PartialEq, Debug)]
enum ChunkError {
    Corrupted(u8, u8), // expected, found
    Noise(u8),         // first always-illegal character
    Incomplete(Vec<u8>),
    Unmatched(u8),
}

fn flip(b: u8) -> u8 {
    match b {
        b'(' => b')',
        b'[' => b']',
        b'{' => b'}',
        b'<' => b'>',
        _ => panic!("bad argument: {:?}", b as char),
    }
}

fn read_line(line: &str) -> Result<(), ChunkError> {
    let mut stack = vec![];

    for b in line.bytes() {
        match b {
            b'{' | b'(' | b'[' | b'<' => stack.push(b),
            b'}' | b')' | b']' | b'>' => match stack.pop() {
                Some(x) if flip(x) == b => {}
                Some(x) => return Err(ChunkError::Corrupted(flip(x), b)),
                None => return Err(ChunkError::Unmatched(b)),
            },
            b => return Err(ChunkError::Noise(b)),
        }
    }
    if stack.is_empty() {
        Ok(())
    } else {
        Err(ChunkError::Incomplete(stack))
    }
}

#[aoc(day10, part1, jorendorff)]
fn part_1(lines: &[String]) -> u64 {
    lines
        .iter()
        .map(|line| match read_line(line) {
            Err(ChunkError::Corrupted(_expected, actual)) => match actual {
                b')' => 3,
                b']' => 57,
                b'}' => 1197,
                b'>' => 25137,
                _ => panic!("bad character {:?}", actual as char),
            },
            Err(ChunkError::Incomplete(_)) => 0,
            _ => panic!("unexpected outcome"),
        })
        .sum()
}

#[aoc(day10, part2, jorendorff)]
fn part_2(lines: &[String]) -> u64 {
    let mut scores: Vec<u64> = lines
        .iter()
        .filter_map(|line| {
            if let Err(ChunkError::Incomplete(stack)) = read_line(line) {
                Some(
                    stack
                        .into_iter()
                        .rev()
                        .map(|c| match c {
                            b'(' => 1,
                            b'[' => 2,
                            b'{' => 3,
                            b'<' => 4,
                            _ => panic!("unexpected character {:?}", c as char),
                        })
                        .fold(0, |acc, point_value| acc * 5 + point_value),
                )
            } else {
                None
            }
        })
        .collect();

    println!("scores: {:?}", scores);

    // find median score
    let mid = scores.len() / 2;
    *scores.select_nth_unstable(mid).1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_line() {
        assert_eq!(read_line("()"), Ok(()));
        assert_eq!(read_line("[]"), Ok(()));
        assert_eq!(read_line("([])"), Ok(()));
        assert_eq!(read_line("{()()()}"), Ok(()));
        assert_eq!(read_line("<([{}])>"), Ok(()));
        assert_eq!(read_line("[<>({}){}[([])<>]]"), Ok(()));
        assert_eq!(read_line("(((((((((())))))))))"), Ok(()));

        assert_eq!(read_line("(]"), Err(ChunkError::Corrupted(b')', b']')));
        assert_eq!(
            read_line("{()()()>"),
            Err(ChunkError::Corrupted(b'}', b'>'))
        );
        assert_eq!(
            read_line("(((()))}"),
            Err(ChunkError::Corrupted(b')', b'}'))
        );
        assert_eq!(
            read_line("<([]){()}[{}])"),
            Err(ChunkError::Corrupted(b'>', b')'))
        );

        assert_eq!(
            read_line("{([(<{}[<>[]}>{[]{[(<()>"),
            Err(ChunkError::Corrupted(b']', b'}'))
        );
        assert_eq!(
            read_line("[[<[([]))<([[{}[[()]]]"),
            Err(ChunkError::Corrupted(b']', b')'))
        );
        assert_eq!(
            read_line("[{[{({}]{}}([{[{{{}}([]"),
            Err(ChunkError::Corrupted(b')', b']'))
        );
        assert_eq!(
            read_line("[<(<(<(<{}))><([]([]()"),
            Err(ChunkError::Corrupted(b'>', b')'))
        );
        assert_eq!(
            read_line("<{([([[(<>()){}]>(<<{{"),
            Err(ChunkError::Corrupted(b']', b'>'))
        );
    }

    const EXAMPLE: &str = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 26397);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 288957);
    }
}
