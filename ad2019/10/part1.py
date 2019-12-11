"""Day 10: Monitoring Station

You fly into the asteroid belt and reach the Ceres monitoring station. The
Elves here have an emergency: they're having trouble tracking all of the
asteroids and can't be sure they're safe.

The Elves would like to build a new monitoring station in a nearby area of
space; they hand you a map of all of the asteroids in that region (your puzzle
input).

The map indicates whether each position is empty (.) or contains an asteroid
(#). The asteroids are much smaller than they appear on the map, and every
asteroid is exactly in the center of its marked position. The asteroids can be
described with X,Y coordinates where X is the distance from the left edge and Y
is the distance from the top edge (so the top-left corner is 0,0 and the
position immediately to its right is 1,0).

Your job is to figure out which asteroid would be the best place to build a new
monitoring station. A monitoring station can detect any asteroid to which it
has direct line of sight - that is, there cannot be another asteroid exactly
between them. This line of sight can be at any angle, not just lines aligned to
the grid or diagonally. The best location is the asteroid that can detect the
largest number of other asteroids.

For example, consider the following map:

    .#..#
    .....
    #####
    ....#
    ...##

The best location for a new monitoring station on this map is the highlighted
asteroid at 3,4 because it can detect 8 asteroids, more than any other
location. (The only asteroid it cannot detect is the one at 1,0; its view of
this asteroid is blocked by the asteroid at 2,2.) All other asteroids are worse
locations; they can detect 7 or fewer other asteroids. Here is the number of
other asteroids a monitoring station on each asteroid could detect:

    .7..7
    .....
    67775
    ....7
    ...87

Here is an asteroid (#) and some examples of the ways its line of sight might
be blocked. If there were another asteroid at the location of a capital letter,
the locations marked with the corresponding lowercase letter would be blocked
and could not be detected:

    #.........
    ...A......
    ...B..a...
    .EDCG....a
    ..F.c.b...
    .....c....
    ..efd.c.gb
    .......c..
    ....f...c.
    ...e..d..c

Here are some larger examples:

-   Best is 5,8 with 33 other asteroids detected:

        ......#.#.
        #..#.#....
        ..#######.
        .#.#.###..
        .#..#.....
        ..#....#.#
        #..#....#.
        .##.#..###
        ##...#..#.
        .#....####

-   Best is 1,2 with 35 other asteroids detected:

        #.#...#.#.
        .###....#.
        .#....#...
        ##.#.#.#.#
        ....#.#.#.
        .##..###.#
        ..#...##..
        ..##....##
        ......#...
        .####.###.

-   Best is 6,3 with 41 other asteroids detected:

        .#..#..###
        ####.###.#
        ....###.#.
        ..###.##.#
        ##.##.#.#.
        ....###..#
        ..#.#..#.#
        #..#.#.###
        .##...##.#
        .....#.#..

-   Best is 11,13 with 210 other asteroids detected:

        .#..##.###...#######
        ##.############..##.
        .#.######.########.#
        .###.#######.####.#.
        #####.##.#.##.###.##
        ..#####..#.#########
        ####################
        #.####....###.#.#.##
        ##.#################
        #####.##.###..####..
        ..######..##.#######
        ####.##.####...##..#
        .#####..#.######.###
        ##...#.##########...
        #.##########.#######
        .####.#.###.###.#.##
        ....##.##.###..#####
        .#.#.###########.###
        #.#.#.#####.####.###
        ###.##.####.##.#..##

Find the best location for a new monitoring station. How many other asteroids
can be detected from that location?
"""

import math


def all_lines_of_sight(size, p):
    w, h = size
    x0, y0 = p
    for y1 in range(h):
        for x1 in range(w):
            if (x1, y1) != p:
                dx = x1 - x0
                dy = y1 - y0
                if math.gcd(abs(dx), abs(dy)) == 1:
                    yield dx, dy


assert sorted(all_lines_of_sight((5, 5), (3, 4)))[:16] == [
    (-3, -4),
    (-3, -2),
    (-3, -1),
    (-2, -3),
    (-2, -1),
    (-1, -4),
    (-1, -3),
    (-1, -2),
    (-1, -1),
    (-1, 0),
    (0, -1),
    (1, -4),
    (1, -3),
    (1, -2),
    (1, -1),
    (1, 0),
]

def point_is_in_range(size, p):
    w, h = size
    x, y = p
    return 0 <= x < w and 0 <= y < h


def trace_line_of_sight(size, p, dp):
    x, y = p
    dx, dy = dp
    x += dx
    y += dy
    while point_is_in_range(size, (x, y)):
        yield x, y
        x += dx
        y += dy


def find_asteroids(lines):
    """Yield coordinate-pairs of asteroids in the given map"""
    for y, line in enumerate(lines):
        for x, c in enumerate(line):
            if c == '#':
                yield x, y
            elif c == '.':
                pass
            else:
                raise ValueError(f"unexpected character {c!r} at ({x}, {y})")


def parse_grid(text):
    lines = text.splitlines()
    width = len(lines[0])
    height = len(lines)
    return (width, height), list(find_asteroids(lines))


def count_visible_asteroids(size, asteroids, p):
    asteroids = set(asteroids)
    count = 0
    for dp in all_lines_of_sight(size, p):
        if any(q in asteroids for q in trace_line_of_sight(size, p, dp)):
            count += 1
    return count


def test_parse_grid():
    grid = '''\
.#..#
.....
#####
....#
...##
'''
    (w, h), a = parse_grid(grid)
    assert w == 5
    assert h == 5
    assert len(a) == 10
    assert a[0] == (1, 0)
    assert a[-1] == (4, 4)

    assert count_visible_asteroids((w, h), a, (1, 0)) == 7
    assert count_visible_asteroids((w, h), a, (4, 0)) == 7


test_parse_grid()


def candidates(size, asteroids):
    for p in asteroids:
        yield p, count_visible_asteroids(size, asteroids, p)


def best(size, asteroids):
    return max(candidates(size, asteroids), key=lambda pair: pair[1])


def test(grid, expected_p, expected_count):
    size, asteroids = parse_grid(grid)
    p, count = best(size, asteroids)
    assert p == expected_p
    assert count == expected_count

test('''\
.#..#
.....
#####
....#
...##
''', (3, 4), 8)

test('''\
......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####
''', (5, 8), 33)

test('''\
#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.
''', (1, 2), 35)

test('''\
.#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..
''', (6, 3), 41)

test('''\
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
''', (11, 13), 210)


def main():
    with open("puzzle-input.txt") as f:
        text = f.read()
    size, asteroids = parse_grid(text)
    p, count = best(size, asteroids)
    print(count)

if __name__ == '__main__':
    main()
