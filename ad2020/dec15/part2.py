from itertools import islice

puzzle_input = [6, 19, 0, 5, 7, 13, 1]

def play(starting_numbers):
    last = None
    gap = None
    seen = {}
    i = 0
    for n in starting_numbers:
        yield n
        if n in seen:
            gap = i - seen[n]
        else:
            gap = 0
        seen[n] = i
        i += 1

    while True:
        n = gap
        yield n
        if n in seen:
            gap = i - seen[n]
        else:
            gap = 0
        seen[n] = i
        i += 1


assert list(islice(play([0, 3, 6]), 0, 10)) == [0, 3, 6, 0, 3, 3, 1, 0, 4, 0]


N = 30000000

def solve(starting_numbers):
    [result] = islice(play(starting_numbers), N - 1, N)
    return result


assert solve([0, 3, 6]) == 175594
assert solve([1, 3, 2]) == 2578
assert solve([2, 1, 3]) == 3544142
assert solve([1, 2, 3]) == 261214
assert solve([2, 3, 1]) == 6895259
assert solve([3, 2, 1]) == 18
assert solve([3, 1, 2]) == 362


if __name__ == '__main__':
    print(solve(puzzle_input))
