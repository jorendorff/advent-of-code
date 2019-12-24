"""

"""

from lib.advent import *


WIDTH = 5
HEIGHT = 5


def point_to_bit(p):
    x, y = p
    return 1 << (y * WIDTH + x)


assert point_to_bit((0, 0)) == 1
assert point_to_bit((0, 3)) == 32768
assert point_to_bit((1, 4)) == 2097152


def points_to_bits(points):
    return sum(point_to_bit(p) for p in points)

# bitset of all points
MASK = (1 << (WIDTH * HEIGHT)) - 1

# bitset of all points that are LEFT/RIGHT/UP/DOWN of another point
LEFT_MASK = points_to_bits((x, y) for y in range(HEIGHT) for x in range(WIDTH - 1))
RIGHT_MASK = points_to_bits((x, y) for y in range(HEIGHT) for x in range(1, WIDTH))
UP_MASK = points_to_bits((x, y) for y in range(HEIGHT - 1) for x in range(WIDTH))
DOWN_MASK = points_to_bits((x, y) for y in range(1, HEIGHT) for x in range(WIDTH))

N_BIT_CLASSES = 3
BIT_CLASS_0 = sum(1 << i for i in range(0, WIDTH * HEIGHT, N_BIT_CLASSES))
BIT_CLASSES = [BIT_CLASS_0 << k for k in range(N_BIT_CLASSES)]


def step(s):
    # bitset of all points that have a bug to the (left/right/up/down)
    x0 = (s & LEFT_MASK) >> 1
    x1 = (s & RIGHT_MASK) << 1
    y0 = (s & UP_MASK) >> WIDTH
    y1 = (s & DOWN_MASK) << WIDTH

    cx0 = ~x0 & ~x1
    cx1 = x0 ^ x1
    cx2 = x0 & x1
    cy0 = ~y0 & ~y1
    cy1 = y0 ^ y1
    cy2 = y0 & y1

    c1 = (cx0 & cy1) | (cx1 & cy0)
    c2 = (cx0 & cy2) | (cx1 & cy1) | (cx2 & cy0)

    survivors = s & ~c1
    infested = ~s & (c1 | c2)

    return survivors | infested


def parse(grid):
    lines = grid.splitlines()
    if len(lines) != HEIGHT:
        raise ValueError("bad height")
    for y, line in enumerate(lines):
        if len(line) != WIDTH:
            raise ValueError(f"bad width, line {y}")
        for x, c in enumerate(line):
            if c == '#':
                yield x, y
            elif c != '.':
                raise ValueError(f"what is this: {c!r}")


def solve(grid):
    grid_bits = points_to_bits(parse(grid))
    cy = Cycle(step, grid_bits)
    print(cy.cycle[0])


EXAMPLE = """\
....#
#..#.
#..##
..#..
#....
"""

assert solve(EXAMPLE) == 2129920


def main():
    grid = puzzle_input()
    print(solve(grid))


if __name__ == '__main__':
    main()
