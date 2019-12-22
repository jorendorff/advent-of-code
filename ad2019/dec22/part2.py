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


class Shuffle:
    """Certain permutations of finite sets of integers."""
    def __init__(self, a, b, n):
        """Return the Shuffle that maps every position x to ax+b (mod n)."""
        self.a = a
        self.b = b
        self.n = n

    def __eq__(self, other):
        return (self.a, self.b, self.n) == (other.a, other.b, other.n)

    @classmethod
    def identity(cls, n):
        return cls(1, 0, n)

    @classmethod
    def cut(cls, k, n):
        return cls(1, -k % n, n)

    @classmethod
    def deal(cls, incr, n):
        return cls(incr, 0, n)

    @classmethod
    def reverse(cls, n):
        return cls(-1 % n, -1 % n, n)


    def __call__(self, x):
        """Return the index where this shuffle leaves a card that starts at position x."""
        if not 0 <= x < self.n:
            raise ValueError("out of range")
        return (self.a * x + self.b) % self.n

    def __add__(self, other):
        """Compose two shuffles."""
        assert self.n == other.n
        n = self.n

        # We're doing this shuffle first, then the other one.
        # x
        # ---self--->
        # self.a * x + self.b
        # ---other--->
        # other.a * (self.a * x + self.b) + other.b

        return Shuffle((other.a * self.a) % n,
                       (other.a * self.b + other.b) % n,
                       n)

    def __neg__(self):
        """Compute the inverse of this shuffle."""
        cls = self.__class__
        a, b, n = self.a, self.b, self.n
        assert self == cls(a, 0, n) + cls(1, b, n)
        return cls(1, n - b, n) + cls(modular_inverse(a, n), 0, n)

    def __mul__(self, count):
        n = self.n
        cls = self.__class__
        if isinstance(count, int):
            if count < 0:
                return -self * -count
            elif count == 0:
                return cls.identity(n)
            elif count == 1:
                return self
            elif count == 2:
                return self + self
            else:
                return (self * (count >> 1)) * 2 + self * (count & 1)

    __rmul__ = __mul__


def modular_inverse(a, m):
    """Find b in range(m) such that a * b == 1 modulo m.
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


def parse_steps(n, instructions):
    for line in instructions.splitlines():
        line = line.strip()
        if line == 'deal into new stack':
            yield Shuffle.reverse(n)
        elif line.startswith('deal with increment '):
            words = line.split()
            assert len(words) == 4
            yield Shuffle.deal(int(words[-1]), n)
        elif line.startswith('cut '):
            words = line.split()
            assert len(words) == 2
            yield Shuffle.cut(int(words[-1]), n)
        else:
            raise ValueError("unexpected instruction: " + line)


SIZE = 119315717514047

REPEAT_COUNT = 101741582076661

TARGET_POS = 2020


def solve(size, instructions, target, repeat_count):
    steps = parse_steps(size, instructions)
    f = sum(steps, Shuffle.identity(size))
    unshuffle_many = f * -repeat_count
    return unshuffle_many(target)


def test():
    for a in range(1, 5):
        for b in range(0, 5):
            f = Shuffle(a, b, 5)
            finv = f * -1
            assert f + finv == Shuffle(1, 0, 5)

    for test in part1.TESTS.split("\n\n"):
        instructions, result = test.strip().rsplit('\n', 1)

        assert result.startswith('Result: ')
        expected = [int(n) for n in result[8:].strip().split()]
        size = len(expected)

        # Assert solver works for repeat_count == 1
        actual = [solve(size, instructions, i, 1) for i in range(size)]
        if actual != expected:
            raise ValueError("Test failed:\n"
                             + "actual: " + repr(actual) + "\n"
                             + "expected: " + repr(expected) + "\n")


def main():
    text = puzzle_input()
    print(solve(SIZE, text, TARGET_POS, REPEAT_COUNT))


if __name__ == '__main__':
    test()
    main()

