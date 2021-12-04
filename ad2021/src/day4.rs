use aoc_runner_derive::*;

type Board = Vec<Vec<i32>>;

#[aoc_generator(day4)]
fn parse_input(text: &str) -> anyhow::Result<(Vec<i32>, Vec<Board>)> {
    let mut chunks = text.split("\n\n");
    let call_order = chunks
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let boards = chunks
        .map(|chunk| {
            let board: Board = chunk
                .trim()
                .split('\n')
                .map(|line| {
                    let row: Vec<i32> = line
                        .split_whitespace()
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect();
                    assert_eq!(row.len(), 5);
                    row
                })
                .collect();
            assert_eq!(board.len(), 5);
            board
        })
        .collect();

    Ok((call_order, boards))
}

fn common<'a>(
    call_order: &'a [i32],
    boards: &'a [Board],
) -> impl Iterator<Item = (usize, &'a Board)> + 'a {
    // `call_order` can be thought of as a function from time to Bingo numbers.
    // Invert the function to go in the other direction, from numbers to time.
    let nums = call_order.len();
    let mut num_to_time = vec![0; nums];
    for (order, num) in call_order.iter().copied().enumerate() {
        num_to_time[num as usize] = order;
    }

    let mut rows_and_cols: Vec<Vec<(usize, usize)>> =
        (0..5).map(|r| (0..5).map(|c| (r, c)).collect()).collect();
    rows_and_cols.extend((0..5).map(|c| (0..5).map(|r| (r, c)).collect()));

    let time_of_win = move |board: &'a Board| {
        rows_and_cols
            .iter()
            .map(|row_or_col| {
                // how long does it take to fill in this row or column?
                row_or_col
                    .iter()
                    .copied()
                    .map(|(r, c)| num_to_time[board[r][c] as usize])
                    .max()
                    .unwrap()
            })
            .min()
            .unwrap()
    };

    boards.iter().map(move |board| (time_of_win(board), board))
}

fn score(call_order: &[i32], time: usize, winning_board: &Board) -> i32 {
    let last_called = call_order[time];
    let total_unmarked_nums = winning_board
        .iter()
        .flatten()
        .map(|num| {
            if call_order[..time + 1].contains(num) {
                0
            } else {
                *num
            }
        })
        .sum::<i32>();

    total_unmarked_nums * last_called
}

#[aoc(day4, part1)]
fn part_1((call_order, boards): &(Vec<i32>, Vec<Board>)) -> i32 {
    let (time, winning_board) = common(call_order, boards).min().unwrap();
    score(call_order, time, winning_board)
}

#[aoc(day4, part2)]
fn part_2((call_order, boards): &(Vec<i32>, Vec<Board>)) -> i32 {
    let (time, winning_board) = common(call_order, boards).max().unwrap();
    score(call_order, time, winning_board)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 4512);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 1924);
    }
}
