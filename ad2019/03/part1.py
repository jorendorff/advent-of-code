"""Day 3: Crossed Wires

The gravity assist was successful, and you're well on your way to the Venus
refuelling station. During the rush back on Earth, the fuel management system
wasn't completely installed, so that's next on the priority list.

Opening the front panel reveals a jumble of wires. Specifically, two wires are
connected to a central port and extend outward on a grid. You trace the path
each wire takes as it leaves the central port, one wire per line of text (your
puzzle input).

The wires twist and turn, but the two wires occasionally cross paths. To fix
the circuit, you need to find the intersection point closest to the central
port. Because the wires are on a grid, use the Manhattan distance for this
measurement. While the wires do technically cross right at the central port
where they both start, this point does not count, nor does a wire count as
crossing with itself.

For example, if the first wire's path is `R8,U5,L5,D3`, then starting from the
central port (`o`), it goes right `8`, up `5`, left `5`, and finally down `3`:

    ...........
    ...........
    ...........
    ....+----+.
    ....|....|.
    ....|....|.
    ....|....|.
    .........|.
    .o-------+.
    ...........

Then, if the second wire's path is `U7,R6,D4,L4`, it goes up `7`, right `6`, down `4`, and left `4`:

    ...........
    .+-----+...
    .|.....|...
    .|..+--X-+.
    .|..|..|.|.
    .|.-X--+.|.
    .|..|....|.
    .|.......|.
    .o-------+.
    ...........

These wires cross at two locations (marked `X`), but the lower-left one is
closer to the central port: its distance is `3 + 3 = 6`.

Here are a few more examples:

*   `R75,D30,R83,U83,L12,D49,R71,U7,L72
    U62,R66,U55,R34,D71,R55,D58,R83` = distance `159`

*   `R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
    U98,R91,D20,R16,D67,R40,U7,R15,U6,R7` = distance 135

What is the Manhattan distance from the central port to the closest
intersection?
"""

STEPS = {
    'R': 1 + 0j,
    'U': 1j,
    'L': -1 + 0j,
    'D': -1j,
}


def displacements(s):
    for word in s.split(','):
        step = STEPS[word[0]]
        length = int(word[1:])
        yield from [step] * length


assert list(displacements('U2,R1')) == [1j, 1j, 1]

def partial_sums(it, start=0):
    point = start
    yield point
    for step in it:
        point += step
        yield point


assert set(partial_sums(displacements('R5'))) == {0, 1, 2, 3, 4, 5}


def wire_points(s):
    """Convert a path-string to the set of points touched by the wire."""
    return set(partial_sums(displacements(s)))


test_wire_1 = wire_points('R8,U5,L5,D3')
assert len(test_wire_1) == 22
test_wire_2 = wire_points('U7,R6,D4,L4')
assert len(test_wire_2) == 22
assert test_wire_1 & test_wire_2 == {0, 3 + 3j, 6 + 5j}


def distance(p):
    """Manhattan distance from the origin to p."""
    return abs(p.real) + abs(p.imag)


assert distance(3 - 4j) == 7


def solve(first, second):
    intersections = wire_points(first) & wire_points(second)
    return min(distance(p) for p in intersections if p != 0)


assert solve('R8,U5,L5,D3', 'U7,R6,D4,L4') == 6
assert solve('R75,D30,R83,U83,L12,D49,R71,U7,L72',
             'U62,R66,U55,R34,D71,R55,D58,R83') == 159
assert solve('R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51',
             'U98,R91,D20,R16,D67,R40,U7,R15,U6,R7') == 135


with open("puzzle-input.txt") as f:
    [first, second] = f.read().split()
print(int(solve(first, second)))
