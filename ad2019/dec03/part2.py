"""Part Two

It turns out that this circuit is very timing-sensitive; you actually need to
minimize the signal delay.

To do this, calculate the number of steps each wire takes to reach each
intersection; choose the intersection where the sum of both wires' steps is
lowest. If a wire visits a position on the grid multiple times, use the steps
value from the first time it visits that position when calculating the total
value of a specific intersection.

The number of steps a wire takes is the total number of grid squares the wire
has entered to get to that location, including the intersection being
considered. Again consider the example from above:

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

In the above example, the intersection closest to the central port is reached
after `8+5+5+2 = 20` steps by the first wire and `7+6+4+3 = 20` steps by the second
wire for a total of `20+20 = 40` steps.

However, the top-right intersection is better: the first wire takes only `8+5+2
= 15` and the second wire takes only `7+6+2 = 15`, a total of `15+15 = 30`
steps.

Here are the best steps for the extra examples from above:

*   `R75,D30,R83,U83,L12,D49,R71,U7,L72
    U62,R66,U55,R34,D71,R55,D58,R83` = `610` steps
*   `R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
    U98,R91,D20,R16,D67,R40,U7,R15,U6,R7` = `410` steps

What is the fewest combined steps the wires must take to reach an intersection?
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
    """Convert a path-string to a dict mapping points to number of steps."""
    result = {}
    for steps, point in enumerate(partial_sums(displacements(s))):
        if point not in result:
            result[point] = steps
    return result


test_wire_1 = wire_points('R8,U5,L5,D3')
assert len(test_wire_1) == 22
assert test_wire_1[0] == 0
assert test_wire_1[1] == 1
assert test_wire_1[8] == 8
assert test_wire_1[8+4j] == 12
assert test_wire_1[3+3j] == 20
assert sorted(test_wire_1.values()) == list(range(22))
test_wire_2 = wire_points('U7,R6,D4,L4')
assert len(test_wire_2) == 22


def intersections(wire1, wire2):
    return {p: wire1[p] + wire2[p] for p in wire1 if p in wire2}


assert intersections(test_wire_1, test_wire_2) == {0: 0, 3 + 3j: 40, 6 + 5j: 30}


def distance(p):
    """Manhattan distance from the origin to p."""
    return abs(p.real) + abs(p.imag)


assert distance(3 - 4j) == 7


def solve(first, second):
    step_mapping = intersections(wire_points(first), wire_points(second))
    return min(t for t in step_mapping.values() if t != 0)


assert solve('R8,U5,L5,D3', 'U7,R6,D4,L4') == 30
assert solve('R75,D30,R83,U83,L12,D49,R71,U7,L72',
             'U62,R66,U55,R34,D71,R55,D58,R83') == 610
assert solve('R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51',
             'U98,R91,D20,R16,D67,R40,U7,R15,U6,R7') == 410


with open("puzzle-input.txt") as f:
    [first, second] = f.read().split()
print(int(solve(first, second)))
