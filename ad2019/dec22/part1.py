"""

"""

from lib.advent import *
from ad2019.intcode.interpreter import IntcodeVM, parse
from math import gcd


def shuf(size, instructions):
    deck = list(range(size))
    for line in instructions.splitlines():
        line = line.strip()
        words = line.split()
        if line == 'deal into new stack':
            deck.reverse()
        elif line.startswith('deal with increment '):
            assert len(words) == 4
            incr = int(words[-1])
            assert gcd(incr, size) == 1
            # the tests unfortunately require us to deal with nonprime sizes
            new_deck = [None] * size
            for i, card in enumerate(deck):
                new_deck[i * incr % size] = card
            deck[:] = new_deck

        elif line.startswith('cut '):
            assert len(words) == 2
            amt = int(words[-1])
            # this can be positive or negative
            deck = deck[amt:] + deck[:amt]
        else:
            raise ValueError("unexpected instruction: " + line)
    return deck


TESTS = """\
deal with increment 7
deal into new stack
deal into new stack
Result: 0 3 6 9 2 5 8 1 4 7

cut 6
deal with increment 7
deal into new stack
Result: 3 0 7 4 1 8 5 2 9 6

deal with increment 7
deal with increment 9
cut -2
Result: 6 3 0 7 4 1 8 5 2 9

deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1
Result: 9 2 5 8 1 4 7 0 3 6
"""


def test():
    for test in TESTS.split("\n\n"):
        instructions, result = test.rsplit('\n', 1)
        assert result.startswith('Result: ')
        expected = [int(n) for n in result[8:].strip().split()]
        actual = shuf(10, instructions)
        if actual != expected:
            raise ValueError("Test failed:\n"
                             + "actual: " + repr(actual) + "\n"
                             + "expected: " + repr(expected) + "\n")


test()


def main():
    instructions = puzzle_input()
    deck = shuf(10007, instructions)
    print(deck.index(2019))


if __name__ == '__main__':
    main()
