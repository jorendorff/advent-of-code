"""Part Two

After a while, you realize your shuffling skill won't improve much more with
merely a single deck of cards. You ask every 3D printer on the ship to make you
some more cards while you check on the ship repairs. While reviewing the work
the droids have finished so far, you think you see Halley's Comet fly past!

When you get back, you discover that the 3D printers have combined their power
to create for you a single, giant, brand new, factory order deck of
119315717514047 space cards.

Finally, a deck of cards worthy of shuffling!

You decide to apply your complete shuffle process (your puzzle input) to the
deck 101741582076661 times in a row.

You'll need to be careful, though - one wrong move with this many cards and you
might overflow your entire ship!

After shuffling your new, giant, factory order deck that many times, what
number is on the card that ends up in position 2020?

"""

from lib.advent import *
from ad2019.intcode.interpreter import IntcodeVM, parse
from . import part1
from math import gcd


def compile(size, instructions):
    offset = 0
    coeff = 1
    # every card is moved from offset i to offset `(coeff * i + offset) % size`.
    for line in instructions.splitlines():
        line = line.strip()
        if line == 'deal into new stack':
            # pos = size - pos - 1
            # negate
            coeff = size - coeff
            offset = size - offset
            # add -1
            offset = (offset - 1) % size
        elif line.startswith('deal with increment '):
            words = line.split()
            assert len(words) == 4
            incr = int(words[-1])
            # pos = (pos * incr) % size
            coeff = (coeff * incr) % size
            offset = (offset * incr) % size
        elif line.startswith('cut '):
            words = line.split()
            assert len(words) == 2
            amt = int(words[-1])
            # pos = (pos - amt) % size
            offset = (offset - amt) % size
        else:
            raise ValueError("unexpected instruction: " + line)
    return (offset, coeff)


def shuf(size, instructions, start):
    pos = start
    for line in instructions.splitlines():
        line = line.strip()
        if line == 'deal into new stack':
            pos = size - pos - 1
        elif line.startswith('deal with increment '):
            words = line.split()
            assert len(words) == 4
            incr = int(words[-1])
            pos = (pos * incr) % size
        elif line.startswith('cut '):
            words = line.split()
            assert len(words) == 2
            amt = int(words[-1])
            # this can be positive or negative
            pos = (pos - amt) % size
        else:
            raise ValueError("unexpected instruction: " + line)
    return pos


SIZE = 119315717514047

REPEAT_COUNT = 101741582076661

TARGET_POS = 2020


def slow_solve(size, instructions, target, repeat_count):
    for start in range(size):
        pos = start
        for _ in range(repeat_count):
            pos = shuf(size, instructions, pos)
        if pos == target:
            return start
    assert False, "should not get here"

def compose(f, g, size):
    c0, c1 = f
    d0, d1 = g
    # (f . g) x = f (g x)
    # = f (d1 * x + d0)
    # = (d1 * x + d0) * c1 + c0
    # = (c1 * d1) * x + (c1 * d0 + c0)
    return ((c1 * d0 + c0) % size, (c1 * d1) % size)


def modular_inverse(a, m):
    """Find b in range(m) such that a * b is a multiple of m.
    Blatant ripoff of https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
    """
    t = 0
    newt = 1
    r = m
    newr = a
    while newr != 0:
        q = r // newr
        t, newt = newt, t - q * newt
        r, newr = newr, r - q * newr
    if r > 1:
        raise ValueError("not invertible")
    if t < 0:
        t += m
    return t



def exp(f, n, size):
    if n == 0:
        return (0, 1)
    elif n == 1:
        return f
    elif n & 1:
        return compose(f, exp(f, n - 1, size), size)
    else:
        assert n > 1
        half = exp(f, n >> 1, size)
        return compose(half, half, size)

def inverse(f, size):
    c0, c1 = f
    # f is compose((+ c0), (* c1)).
    assert f == compose((c0, 1), (0, c1), size)
    # The inverse is compose((* modular_inverse(c1)), (- c0)).
    return compose((0, modular_inverse(c1, size)), (size - c0, 1), size)


def solve(size, instructions, start, repeat_count):
    f = compile(size, instructions)
    fn = inverse(exp(f, repeat_count, size), size)
    c0, c1 = fn
    return (c1 * start + c0) % size


def test():
    for c0 in range(5):
        for c1 in range(1, 5):
            f = (c0, c1)
            finv = inverse(f, 5)
            assert compose(f, finv, 5) == (0, 1)

    for test in part1.TESTS.split("\n\n"):
        instructions, result = test.strip().rsplit('\n', 1)

        assert result.startswith('Result: ')
        expected = [int(n) for n in result[8:].strip().split()]
        size = len(expected)

        # Assert results are as expected for repeat_count==1
        actual = [slow_solve(size, instructions, i, 1) for i in range(size)]
        if actual != expected:
            raise ValueError("Test failed:\n"
                             + "actual: " + repr(actual) + "\n"
                             + "expected: " + repr(expected) + "\n")

        # Assert faster solver works for repeat_count == 1
        actual = [solve(size, instructions, i, 1) for i in range(size)]
        if actual != expected:
            raise ValueError("Test failed:\n"
                             + "actual: " + repr(actual) + "\n"
                             + "expected: " + repr(expected) + "\n")

        # Assert solvers agree for larger repeat_count
        for repeat_count in (1, 2, 3, 4, 5, 1517):
            for i in range(10):
                slow_answer = slow_solve(size, instructions, i, repeat_count)
                fast_answer = solve(size, instructions, i, repeat_count)
                assert slow_answer == fast_answer


def main():
    text = puzzle_input()
    print(solve(SIZE, text, TARGET_POS, REPEAT_COUNT))


if __name__ == '__main__':
    test()
    main()

