from lib.advent import *
from bisect import insort

def is_sum_of_two(vals, x):
    """True if x is the sum of two integers in the sorted list vals."""
    i = 0
    j = len(vals)
    while i + 2 <= j:
        t = vals[i] + vals[j - 1]
        if t > x:
            j -= 1
        elif t < x:
            i += 1
        else:
            assert x == vals[i] + vals[j - 1] and i != j - 1
            return True
    return False

def solve(seq, size):
    working = sorted(seq[:size])
    for old, new in zip(seq, seq[size:]):
        if not is_sum_of_two(working, new):
            return new
        working.remove(old)
        insort(working, new)
    return None

example = [
    35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127,
    219, 299, 277, 309, 576,
]

assert solve(example, 5) == 127

if __name__ == '__main__':
    nums = [int(s) for s in puzzle_input().strip().split()]
    print(solve(nums, 26))

