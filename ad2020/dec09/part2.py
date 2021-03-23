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

def find_flaw(seq, size):
    working = sorted(seq[:size])
    for old, new in zip(seq, seq[size:]):
        if not is_sum_of_two(working, new):
            return new
        working.remove(old)
        insort(working, new)
    return None

def solve(seq, size):
    flaw = find_flaw(seq, size)
    if flaw is None:
        raise ValueError("no flaw found")

    i = 0
    j = 0
    total = 0
    while total != flaw:
        if total < flaw:
            if j == len(seq):
                raise ValueError("no solution found")
            total += seq[j]
            j += 1
        else:
            assert total > flaw
            total -= seq[i]
            i += 1
    print(f"elements seq[{i}..{j}] add up to {flaw}")
    assert sum(seq[i:j]) == flaw
    return min(seq[i:j]) + max(seq[i:j])

example = [
    35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127,
    219, 299, 277, 309, 576,
]

assert solve(example, 5) == 62

if __name__ == '__main__':
    nums = [int(s) for s in puzzle_input().strip().split()]
    print(solve(nums, 26))

