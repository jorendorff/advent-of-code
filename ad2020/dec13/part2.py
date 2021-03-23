from lib.advent import *
from math import gcd, lcm


def relatively_prime(a, b):
    """True if a and b do not share any prime factors in common."""
    return gcd(a, b) == 1


def parse_notes(notes):
    _line1, line2 = notes.splitlines()
    return [
        (i, int(s))
        for i, s in enumerate(line2.split(","))
        if s != 'x'
    ]

def departure_time(bus_id, t):
    """Return the earliest departure time of the given bus that's not before timestamp `t`.

    Or equivalently, the least multiple of `bus_id` that is at least `t`.
    """
    return (t + bus_id - 1) // bus_id * bus_id


def is_departing_at(bus_id, t):
    return t % bus_id == 0

def solve(notes):
    buses = parse_notes(notes)

    # There are always infinitely many solutions,
    # of the form `range(t0, ∞, period)`.
    # With no requirements, we have solutions at 0 and every minute thereafter:
    t0 = 0
    period = 1
    # Each bus we consider restricts this.
    for i, (bus_offset, bus_id) in enumerate(buses):
        #print(f"considering bus {bus_id} which must leave at Δt={bus_offset}...")
        while True:
            if is_departing_at(bus_id, t0 + bus_offset):
                break
            t0 += period
        period = lcm(period, bus_id)
        #print(f"  solutions = range({t0}, ∞, {period}).")

        assert is_departing_at(bus_id, t0 + bus_offset)
        assert is_departing_at(bus_id, t0 + period + bus_offset)
        assert all(is_departing_at(b_id, t0 + b_offset)
                   for b_offset, b_id in buses[:i+1])
        assert all(is_departing_at(b_id, t0 + period + b_offset)
                   for b_offset, b_id in buses[:i+1])

    #print(f"answer is: {t0}")
    #print()
    return t0  # return the first of infinitely many solutions


example = """\
939
7,13,x,x,59,x,31,19
"""

# looking for a number `t` such that
#   t % 7 == 0
#   t % 13 == -1 % 13
#   t % 59 == -4 % 59
#   t % 31 == -6 % 31
#   t % 19 == -7 % 19

assert solve(example) == 1068781


if __name__ == '__main__':
    print(solve(puzzle_input()))
