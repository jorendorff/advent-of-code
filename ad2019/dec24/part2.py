"""Part Two

After careful analysis, one thing is certain: you have no idea where all these bugs are coming from.

Then, you remember: Eris is an old Plutonian settlement! Clearly, the bugs are coming from recursively-folded space.

This 5x5 grid is only one level in an infinite number of recursion levels. The tile in the middle of the grid is actually another 5x5 grid, the grid in your scan is contained as the middle tile of a larger 5x5 grid, and so on. Two levels of grids look like this:

     |     |         |     |     
     |     |         |     |     
     |     |         |     |     
-----+-----+---------+-----+-----
     |     |         |     |     
     |     |         |     |     
     |     |         |     |     
-----+-----+---------+-----+-----
     |     | | | | | |     |     
     |     |-+-+-+-+-|     |     
     |     | | | | | |     |     
     |     |-+-+-+-+-|     |     
     |     | | |?| | |     |     
     |     |-+-+-+-+-|     |     
     |     | | | | | |     |     
     |     |-+-+-+-+-|     |     
     |     | | | | | |     |     
-----+-----+---------+-----+-----
     |     |         |     |     
     |     |         |     |     
     |     |         |     |     
-----+-----+---------+-----+-----
     |     |         |     |     
     |     |         |     |     
     |     |         |     |     

(To save space, some of the tiles are not drawn to scale.) Remember, this is only a small part of the infinitely recursive grid; there is a 5x5 grid that contains this diagram, and a 5x5 grid that contains that one, and so on. Also, the ? in the diagram contains another 5x5 grid, which itself contains another 5x5 grid, and so on.

The scan you took (your puzzle input) shows where the bugs are on a single level of this structure. The middle tile of your scan is empty to accommodate the recursive grids within it. Initially, no other levels contain bugs.

Tiles still count as adjacent if they are directly up, down, left, or right of a given tile. Some tiles have adjacent tiles at a recursion level above or below its own level. For example:

     |     |         |     |     
  1  |  2  |    3    |  4  |  5  
     |     |         |     |     
-----+-----+---------+-----+-----
     |     |         |     |     
  6  |  7  |    8    |  9  |  10 
     |     |         |     |     
-----+-----+---------+-----+-----
     |     |A|B|C|D|E|     |     
     |     |-+-+-+-+-|     |     
     |     |F|G|H|I|J|     |     
     |     |-+-+-+-+-|     |     
 11  | 12  |K|L|?|N|O|  14 |  15 
     |     |-+-+-+-+-|     |     
     |     |P|Q|R|S|T|     |     
     |     |-+-+-+-+-|     |     
     |     |U|V|W|X|Y|     |     
-----+-----+---------+-----+-----
     |     |         |     |     
 16  | 17  |    18   |  19 |  20 
     |     |         |     |     
-----+-----+---------+-----+-----
     |     |         |     |     
 21  | 22  |    23   |  24 |  25 
     |     |         |     |     

    Tile 19 has four adjacent tiles: 14, 18, 20, and 24.
    Tile G has four adjacent tiles: B, F, H, and L.
    Tile D has four adjacent tiles: 8, C, E, and I.
    Tile E has four adjacent tiles: 8, D, 14, and J.
    Tile 14 has eight adjacent tiles: 9, E, J, O, T, Y, 15, and 19.
    Tile N has eight adjacent tiles: I, O, S, and five tiles within the sub-grid marked ?.

The rules about bugs living and dying are the same as before.

For example, consider the same initial state as above:

....#
#..#.
#.?##
..#..
#....

The center tile is drawn as ? to indicate the next recursive grid. Call this level 0; the grid within this one is level 1, and the grid that contains this one is level -1. Then, after ten minutes, the grid at each level would look like this:

Depth -5:
..#..
.#.#.
..?.#
.#.#.
..#..

Depth -4:
...#.
...##
..?..
...##
...#.

Depth -3:
#.#..
.#...
..?..
.#...
#.#..

Depth -2:
.#.##
....#
..?.#
...##
.###.

Depth -1:
#..##
...##
..?..
...#.
.####

Depth 0:
.#...
.#.##
.#?..
.....
.....

Depth 1:
.##..
#..##
..?.#
##.##
#####

Depth 2:
###..
##.#.
#.?..
.#.##
#.#..

Depth 3:
..###
.....
#.?..
#....
#...#

Depth 4:
.###.
#..#.
#.?..
##.#.
.....

Depth 5:
####.
#..#.
#.?#.
####.
.....

In this example, after 10 minutes, a total of 99 bugs are present.

Starting with your scan, how many bugs are present after 200 minutes?

"""

from lib.advent import *
from functools import lru_cache
import collections


WIDTH = 5
HEIGHT = 5

MID_X = WIDTH // 2
MID_Y = HEIGHT // 2


def dump_bugs(bugs):
    min_z = min(z for x, y, z in bugs)
    max_z = max(z for x, y, z in bugs)
    for z in range(min_z, max_z + 1):
        print(f"Depth {z}:")
        for y in range(HEIGHT):
            for x in range(WIDTH):
                print('#' if (x, y, z) in bugs
                      else '?' if (x, y) == (MID_X, MID_Y)
                      else '.', end='')
            print()
        print()

def parse(grid):
    lines = grid.splitlines()
    if len(lines) != HEIGHT:
        raise ValueError("bad height")
    for y, line in enumerate(lines):
        if len(line) != WIDTH:
            raise ValueError(f"bad width, line {y}")
        for x, c in enumerate(line):
            if c == '#':
                yield (x, y, 0)
            elif c != '.':
                raise ValueError(f"what is this: {c!r}")


@lru_cache(maxsize=200*WIDTH*HEIGHT)
def neighbors(point):
    x, y, z = point
    results = set()

    # outer neighbors
    if x == 0:
        results.add((MID_X - 1, MID_Y, z - 1))
    elif x == WIDTH - 1:
        results.add((MID_X + 1, MID_Y, z - 1))
    if y == 0:
        results.add((MID_X, MID_Y - 1, z - 1))
    elif y == HEIGHT - 1:
        results.add((MID_X, MID_Y + 1, z - 1))

    # local neighbors
    if x > 0:
        results.add((x - 1, y, z))
    if x < WIDTH - 1:
        results.add((x + 1, y, z))
    if y > 0:
        results.add((x, y - 1, z))
    if y < HEIGHT - 1:
        results.add((x, y + 1, z))

    bad_point = (MID_X, MID_Y, z)
    if bad_point in results:
        results.remove(bad_point)

    # inner neighbors
    if x == MID_X:
        if y == MID_Y - 1:
            for nx in range(WIDTH):
                results.add((nx, 0, z + 1))
        elif y == MID_Y + 1:
            for nx in range(WIDTH):
                results.add((nx, HEIGHT - 1, z + 1))
    elif y == MID_Y:
        if x == MID_X - 1:
            for ny in range(HEIGHT):
                results.add((0, ny, z + 1))
        elif x == MID_X + 1:
            for ny in range(HEIGHT):
                results.add((WIDTH - 1, ny, z + 1))

    return results


assert len(neighbors((3, 3, 0))) == 4
assert len(neighbors((1, 1, 1))) == 4
assert len(neighbors((3, 0, 1))) == 4
assert len(neighbors((4, 0, 1))) == 4
print(neighbors((3, 2, 0)))
assert len(neighbors((3, 2, 0))) == 8
assert len(neighbors((3, 2, 1))) == 8



def step(bugs):
    c = collections.Counter(n for p in bugs for n in neighbors(p))
    return set(p
               for p, count in c.items()
               if count == 1 or (count == 2 and p not in bugs))


def count_bugs_after(grid, steps):
    bugs = set(parse(grid))
    dump_bugs(bugs)
    for _ in range(steps):
        bugs = step(bugs)
        dump_bugs(bugs)
    return len(bugs)


EXAMPLE = """\
....#
#..#.
#..##
..#..
#....
"""

assert count_bugs_after(EXAMPLE, 10) == 99


def main():
    print(count_bugs_after(puzzle_input(), 200))


if __name__ == '__main__':
    main()
