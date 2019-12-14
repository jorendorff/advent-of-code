"""Part Two

After collecting ORE for a while, you check your cargo hold: 1 trillion (1000000000000) units of ORE.

With that much ore, given the examples above:

    The 13312 ORE-per-FUEL example could produce 82892753 FUEL.
    The 180697 ORE-per-FUEL example could produce 5586022 FUEL.
    The 2210736 ORE-per-FUEL example could produce 460664 FUEL.

Given 1 trillion ORE, what is the maximum amount of FUEL you can produce?
"""

import part1

# There may be a cleverer way to solve this, but the simplest thing is to
# bisect. Surely this relation is monotonic.

def can_produce(reactions, ore_amount, fuel_amount):
    return part1.solve(reactions, fuel_amount) <= ore_amount

def bisect(predicate):
    """Return the maximum integer i such that predicate(i).

    The predicate must be monotonic -- true for all i up to some limit, false above the limit.
    This function returns the limit.
    """

    # Find a lower bound.
    if not predicate(0):
        raise ValueError("predicate can't be satisfied even at 0")
    lo = 0

    # Find an upper bound.
    hi = 1
    while predicate(hi):
        hi *= 16

    # Bisect! Invariant: lo < hi and predicate(lo) and not predicate(hi).
    while lo + 1 < hi:
        mid = (hi + lo) // 2
        if predicate(mid):
            lo = mid
        else:
            hi = mid

    assert lo + 1 == hi
    return lo


ORE_AMOUNT = 1_000_000_000_000  # One Trillion


def solve(reactions):
    if isinstance(reactions, str):
        reactions = part1.parse_reactions(reactionss)
    
    return bisect(lambda n: can_produce(reactions, ORE_AMOUNT, n))


assert solve(EXAMPLE3) == 82892753
assert solve(EXAMPLE4) == 5586022
assert solve(EXAMPLE5) == 460664

if __name__ == '__main__':
    print(solve(puzzle_input()))

