def is_caught(range, t):
    if range == 1:
        return True
    else:
        return t % (2 * (range - 1)) == 0

def is_successful(patrols, delay):
    n = max(patrols.keys()) + 1
    t = delay
    for depth in range(n):
        patrol_range = patrols.get(depth)
        if patrol_range is not None and is_caught(patrol_range, t):
            return False
        t += 1
    return True

def least_delay(patrols):
    for i in range(10_000_000):
        if is_successful(patrols, i):
            return i
    raise ValueError("sick of trying")

assert not is_successful({0: 3, 1: 2, 4: 4, 6: 4}, 0)
assert least_delay({0: 3, 1: 2, 4: 4, 6: 4}) == 10

with open("puzzle-input.txt") as f:
    patrols = dict(map(int, line.strip().split(": ")) for line in f)
print(least_delay(patrols))
