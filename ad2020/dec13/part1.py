from lib.advent import *


def parse_notes(notes):
    line1, line2 = notes.splitlines()
    return int(line1), [int(s) for s in line2.split(",") if s != 'x']


def wait_time(bus_id, earliest):
    """Return how long in minutes we will wait for the given bus
    if `earliest` is the earliest timestamp we can be ready to leave."""
    return (earliest + bus_id - 1) // bus_id * bus_id - earliest

assert wait_time(7, 939) == 6
assert wait_time(59, 939) == 5


def solve(notes):
    t, buses = parse_notes(notes)
    bus = min(buses, key=lambda id: wait_time(id, t))
    return bus * wait_time(bus, t)


example = """\
939
7,13,x,x,59,x,31,19
"""

assert solve(example) == 295


if __name__ == '__main__':
    print(solve(puzzle_input()))
