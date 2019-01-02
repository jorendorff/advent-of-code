import re
import functools
import itertools

def parse(text):
    bots = []
    for line in text.strip().splitlines():
        m = re.match(r'^pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)$', line)
        x, y, z, r = map(int, m.groups())
        bots.append(((x, y, z), r))
    return bots

def distance(p, q):
    return sum(abs(a - b) for a, b in zip(p, q))

def overlap(a, b):
    ap, ar = a
    bp, br = b
    return distance(ap, bp) <= ar + br

def adjacency_matrix(bots):
    return [[overlap(a, b) for b in bots] for a in bots]

def sorted_bots(bots):
    decorated = [(sum(row), bot) for row, bot in zip(adjacency_matrix(bots), bots)]
    decorated.sort(reverse=True)
    return [bot for n, bot in decorated]

def some_clique(bots):
    """Return some clique of elements of `bots` such that every bot in the clique overlaps
    ranges with every other bot in the clique.

    This simple greedy algorithm tries to find a largeish clique but there is no guarnatee
    it finds the largest one.
    Heuristically this happens to produce the right solution for the given puzzle input.
    """
    clique = []
    for b in sorted_bots(bots):
        if all(overlap(a, b) for a in clique):
            clique.append(b)
    return clique

def bot_to_octobox(bot):
    (x, y, z), r = bot
    coords = [
        x + y + z,
        x + y - z,
        x - y + z,
        x - y - z,
    ]
    return tuple((c - r, c + r) for c in coords)  # inclusive, not half-open ranges

def range_intersection(a, b):
    a0, a1 = a
    b0, b1 = b
    return (max(a0, b0), min(a1, b1))

def octobox_intersection(abox, bbox):
    return tuple([range_intersection(a, b) for a, b in zip(abox, bbox)])

def all_octobox_points(box):
    points_hijk = list(itertools.product(*[range(lo, hi + 1) for lo, hi in box]))
    for h, i, j, k in points_hijk:
        x = (i + j) / 2
        y = (h - j) / 2
        z = (h - i) / 2
        if any(c != int(c) for c in (x, y, z)):
            continue
        x = int(x)
        y = int(y)
        z = int(z)
        hh = x + y + z
        ii = x + y - z
        jj = x - y + z
        kk = x - y - z
        assert (hh, ii, jj) == (h, i, j)
        if kk == k:
            yield x, y, z

with open('puzzle-input.txt') as f:
    text = f.read()
bots = parse(text)

clique = some_clique(bots)

result = functools.reduce(octobox_intersection, map(bot_to_octobox, clique))

for p in all_octobox_points(result):
    print(distance(p, (0, 0, 0)))
