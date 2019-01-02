from collections import deque

class Unit:
    def __init__(self, c, x, y, ap):
        self.c = c
        self.x = x
        self.y = y
        self.hp = 200
        self.ap = ap

    def pos(self):
        return (self.y, self.x)

    def distance_to(self, other):
        return abs(self.y - other.y) + abs(self.x - other.x)

    def __repr__(self):
        return "{}@{},{} ({}hp)".format(self.c, self.x, self.y, self.hp)

def neighbors(p):
    y, x = p
    return [(y - 1, x),
            (y, x - 1),
            (y, x + 1),
            (y + 1, x)]

def parse_map(ascii_map, elf_power=3):
    units = []
    grid = [list(row) for row in ascii_map.splitlines()]
    for y, row in enumerate(grid):
        for x, c in enumerate(row):
            if c in 'GE':
                units.append(Unit(c, x, y, 3 if c == 'G' else elf_power))

    return units, grid

def tick(units, grid):
    def free(p):
        return grid[p[0]][p[1]] == '.'

    def move():
        pos = active.pos()

        goals = set()
        for t in units:
            if active.c != t.c:
                for p in neighbors(t.pos()):
                    if pos == p:
                        return  # already got enemies in range
                    if free(p):
                        goals.add(p)

        if not goals:
            return

        todo = [p for p in neighbors(pos) if free(p)]
        how_to_get_to = {p: p for p in todo}
        done = any(p in goals for p in todo)  # maybe one step does the trick
        while not done and todo:
            next_todo = []
            for p in todo:
                first_step = how_to_get_to[p]
                for q in neighbors(p):
                    if q not in how_to_get_to and free(q):
                        how_to_get_to[q] = first_step
                        next_todo.append(q)
                        if q in goals:
                            done = True
            todo = next_todo
        if not done:
            return

        # which goal is best?
        goal = min(p for p in goals if p in how_to_get_to)
        grid[active.y][active.x] = '.'
        active.y, active.x = how_to_get_to[goal]
        grid[active.y][active.x] = active.c

    def attack():
        nonlocal i
        # attack
        targets = [
            (ti, target)
            for ti, target in enumerate(units)
            if active.c != target.c
            and active.distance_to(target) == 1]
        if not targets:
            return
        ti, target = min(targets, key=lambda pair: (pair[1].hp, pair[1].pos()))
        target.hp -= active.ap
        if target.hp <= 0:
            grid[target.y][target.x] = '.'
            del units[ti]
            if ti < i:
                i -= 1

    units.sort(key=Unit.pos)
    i = 0
    while i < len(units):
        active = units[i]
        if not any(u.c != active.c for u in units):
            return 'game over'

        move()
        attack()
        i += 1

def grid_to_ascii(units, g):
    s = ''
    for y, row in enumerate(g):
        s += "{}   {}".format(
            ''.join(row),
            ", ".join(
                "{}({})".format(u.c, u.hp)
                for u in sorted(units, key=Unit.pos)
                if u.y == y))
        s = s.rstrip()
        s += "\n"
    return s


example_map = '''\
#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########
'''

u, g = parse_map(example_map)
tick(u, g)
assert grid_to_ascii(u, g) == '''\
#########
#.G...G.#   G(200), G(200)
#...G...#   G(197)
#...E..G#   E(200), G(200)
#.G.....#   G(200)
#.......#
#G..G..G#   G(200), G(200), G(200)
#.......#
#########
'''

tick(u, g)
assert grid_to_ascii(u, g) == '''\
#########
#..G.G..#   G(200), G(200)
#...G...#   G(194)
#.G.E.G.#   G(200), E(197), G(200)
#.......#
#G..G..G#   G(200), G(200), G(200)
#.......#
#.......#
#########
'''


def combat_outcome(ascii_map, elf_power):
    units, grid = parse_map(ascii_map, elf_power)
    elf_count = sum(1 for u in units if u.c == 'E')
    
    log = "=" * 80
    rounds = 0
    while len(set(unit.c for unit in units)) > 1:  # both sides still have units
        if tick(units, grid) == 'game over':
            break
        rounds += 1

        if sum(1 for u in units if u.c == 'E') != elf_count:
            return 'Failure'

        log += "After {} rounds:\n".format(rounds)
        log += grid_to_ascii(units, grid)
        log += "\n"

    if sum(1 for u in units if u.c == 'E') != elf_count:
        return 'Failure'
    assert all(u.c == 'E' for u in units)
    return rounds * sum(u.hp for u in units)

def boosted_outcome(ascii_map):
    elf_power = 3
    outcome = combat_outcome(ascii_map, elf_power)
    while outcome == 'Failure':
        elf_power += 1
        outcome = combat_outcome(ascii_map, elf_power)

    return elf_power, outcome




example_ascii_map = '''\
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######
'''

assert boosted_outcome(example_ascii_map) == (15, 4988)


samples = [

('''\
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######
''',

#Combat ends after 33 full rounds
#Elves win with 948 total hit points left
#Outcome: 33 * 948 = 31284
 (4, 31284)
),

('''\
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######
''',
#Combat ends after 37 full rounds
#Elves win with 94 total hit points left
#Outcome: 37 * 94 = 3478
 (15, 3478)
),

('''\
#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######
''',
 (12, 6474)
),

('''\
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########
''',
 (34, 1140)),

]


for ascii_map, expected_outcome in samples:
    assert boosted_outcome(ascii_map) == expected_outcome


with open("puzzle-input.txt") as f:
    print(boosted_outcome(f.read())[1])
