"""Part Two

Once you give them the coordinates, the Elves quickly deploy an Instant
Monitoring Station to the location and discover the worst: there are simply too
many asteroids.

The only solution is complete vaporization by giant laser.

Fortunately, in addition to an asteroid scanner, the new monitoring station
also comes equipped with a giant rotating laser perfect for vaporizing
asteroids. The laser starts by pointing up and always rotates clockwise,
vaporizing any asteroid it hits.

If multiple asteroids are exactly in line with the station, the laser only has
enough power to vaporize one of them before continuing its rotation. In other
words, the same asteroids that can be detected can be vaporized, but if
vaporizing one asteroid makes another one detectable, the newly-detected
asteroid won't be vaporized until the laser has returned to the same position
by rotating a full 360 degrees.

For example, consider the following map, where the asteroid with the new
monitoring station (and laser) is marked X:

    .#....#####...#..
    ##...##.#####..##
    ##...#...#.#####.
    ..#.....X...###..
    ..#.#.....#....##

The first nine asteroids to get vaporized, in order, would be:

    .#....###24...#..
    ##...##.13#67..9#
    ##...#...5.8####.
    ..#.....X...###..
    ..#.#.....#....##

Note that some asteroids (the ones behind the asteroids marked 1, 5, and 7)
won't have a chance to be vaporized until the next full rotation. The laser
continues rotating; the next nine to be vaporized are:

    .#....###.....#..
    ##...##...#.....#
    ##...#......1234.
    ..#.....X...5##..
    ..#.9.....8....76

The next nine to be vaporized are then:

    .8....###.....#..
    56...9#...#.....#
    34...7...........
    ..2.....X....##..
    ..1..............

Finally, the laser completes its first full rotation (1 through 3), a second
rotation (4 through 8), and vaporizes the last asteroid (9) partway through its
third rotation:

    ......234.....6..
    ......1...5.....7
    .................
    ........X....89..
    .................

In the large example above (the one with the best monitoring station location at 11,13):

*   The 1st asteroid to be vaporized is at 11,12.
*   The 2nd asteroid to be vaporized is at 12,1.
*   The 3rd asteroid to be vaporized is at 12,2.
*   The 10th asteroid to be vaporized is at 12,8.
*   The 20th asteroid to be vaporized is at 16,0.
*   The 50th asteroid to be vaporized is at 16,9.
*   The 100th asteroid to be vaporized is at 10,16.
*   The 199th asteroid to be vaporized is at 9,6.
*   The 200th asteroid to be vaporized is at 8,2.
*   The 201st asteroid to be vaporized is at 10,9.
*   The 299th and final asteroid to be vaporized is at 11,1.

The Elves are placing bets on which will be the 200th asteroid to be
vaporized. Win the bet by determining which asteroid that will be; what do you
get if you multiply its X coordinate by 100 and then add its Y coordinate? (For
example, 8,2 becomes 802.)
"""

import itertools
from part1 import all_lines_of_sight, trace_line_of_sight, parse_grid, best


def angle(dp):
    # There is probably a more precise way to do this.
    dx, dy = dp
    return math.atan2(dx, dy)

def vaporization_order(size, asteroids, p):
    lines = sorted(all_lines_of_sight(size, p), key=angle)
    remaining = set(asteroid) - {p}
    while remaining:
        for dp in lines:
            for q in trace_line_of_sight(p, dp):
                if q in remaining:
                    yield q
                    remaining.remove(q)

test_grid_1 = '''\
.#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##
'''


def test_1():
    size, asteroids = parse_grid(test_grid_1)
    p = (8, 3)
    hits = list(vaporization_order(size, asteroids, p))
    assert hits[:5] == [(8, 1), (9, 0), (9, 1), (10, 0), (9, 2)]


test_1()


test_grid_2 = '''\
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
'''

def test_2():
    size, asteroids = parse_grid(test_grid_2)
    p, count = best(size, asteroids)
    hits = [None] + list(vaporization_order(size, asteroids, p))

    assert hits[1] == (11, 12)  # The 1st asteroid to be vaporized is at 11,12.
    assert hits[2] == (12, 1)  # The 2nd asteroid to be vaporized is at 12,1.
    assert hits[3] == (12, 2)  # The 3rd asteroid to be vaporized is at 12,2.
    assert hits[10] == (12, 8)  # The 10th asteroid to be vaporized is at 12,8.
    assert hits[20] == (16, 0)  # The 20th asteroid to be vaporized is at 16,0.
    assert hits[50] == (16, 9)  # The 50th asteroid to be vaporized is at 16,9.
    assert hits[100] == (10, 16)  # The 100th asteroid to be vaporized is at 10,16.
    assert hits[199] == (9, 6)  # The 199th asteroid to be vaporized is at 9,6.
    assert hits[200] == (8, 2)  # The 200th asteroid to be vaporized is at 8,2.
    assert hits[201] == (10, 9)  # The 201st asteroid to be vaporized is at 10,9.
    assert hits[299] == (11, 1)  # The 299th and final asteroid to be vaporized is at 11,1.
    assert len(hits) == 300


def main():
    with open("puzzle-input.txt") as f:
        text = f.read()
    size, asteroids = parse_grid(text)
    p, count = best(size, asteroids)
    x, y = list(vaporization_order(size, asteroids, p))[199]
    print(x * 100 + y)


if __name__ == '__main__':
    main()
