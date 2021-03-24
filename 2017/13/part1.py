


def is_caught(range, t):
    if range == 1:
        return True
    else:
        return t % (2 * (range - 1)) == 0

def severity(patrols):
    n = max(patrols.keys()) + 1
    total = 0
    for t in range(n):
        depth = t
        patrol_range = patrols.get(depth)
        if patrol_range is not None and is_caught(patrol_range, t):
            total += depth * patrol_range
    return total

assert severity({0: 3, 1: 2, 4: 4, 6: 4}) == 24

with open("puzzle-input.txt") as f:
    patrols = dict(map(int, line.strip().split(": ")) for line in f)
print(severity(patrols))
