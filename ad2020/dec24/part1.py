from lib.advent import *

DIRS = {
    'nw': (0, 1),
    'ne': (1, 1),
    'e': (1, 0),
    'se': (0, -1),
    'sw': (-1, -1),
    'w': (-1, 0)
}

def parse_point(line):
    x = y = 0
    line = line.strip()
    while line:
        if line[:2] in DIRS:
            dx, dy = DIRS[line[:2]]
            line = line[2:]
        elif line[:1] in DIRS:
            dx, dy = DIRS[line[:1]]
            line = line[1:]
        else:
            DIE
        x += dx
        y += dy
    return x, y


def step(m):
    neighbor_count = Counter(
        (x + dx, y + dy)
        for (x, y), val in m.items()
        if val == 1
        for dx, dy in DIRS.values()
    )
    out = {}
    for p, n in neighbor_count.items():
        if m.get(p) == 1:
            if n in (1, 2):
                out[p] = 1
        else:
            if n == 2:
                out[p] = 1
    return out


def solve(text):
    m = defaultdict(int)
    for line in text.split():
        p = parse_point(line)
        m[p] = 1 - m[p]
    for _ in range(100):
        print(f"Day {_}: {len(m)}")
        m = step(m)
    return sum(m.values())


example = """\
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew
"""

assert solve(example) == 2208

if __name__ == '__main__':
    print(solve(puzzle_input()))
