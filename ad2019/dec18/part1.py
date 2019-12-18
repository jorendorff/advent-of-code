from lib.advent import *


State = namedtuple("State", "steps pos prev keys")

def goal(grid):
    return len([
        c
        for row in grid
        for c in row
        if c.islower()
    ])


def start(grid):
    for y, row in enumerate(grid):
        for x, c in enumerate(row):
            if c == '@':
                return State(0, (x, y), None, "")
    raise ValueError("No starting point in grid!")


def neighbors(point):
    x, y = point
    yield x + 1, y
    yield x, y + 1
    yield x - 1, y
    yield x, y - 1

def solve(text):
    seen = set()

    grid = text.splitlines()
    goal_number = goal(grid)
    todo = deque([start(grid)])
    while todo:
        state = todo.popleft()
        if (state.pos, state.keys) in seen:
            continue
        seen.add((state.pos, state.keys))
        print(state)

        for p in neighbors(state.pos):
            if p != state.prev:
                print(p)
                x, y = p
                c = grid[y][x]
                if c.islower() and c not in state.keys:
                    if len(state.keys) + 1 == goal_number:
                        return state.steps + 1
                    todo.append(State(state.steps + 1, p, None, ''.join(sorted(state.keys + c))))
                elif c == '.' or c == '@' or c.islower() or (c.isupper() and c.lower() in state.keys):
                    todo.append(State(state.steps + 1, p, state.pos, state.keys))
                elif c == '#' or c.isupper():
                    pass
                else:
                    raise ValueError("what is this: " + repr(c))




EXAMPLE_1 = """\
#########
#b.A.@.a#
#########
"""

assert solve(EXAMPLE_1) == 8

EXAMPLE_2 = """\
########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################
"""

assert solve(EXAMPLE_2) == 86

EXAMPLE_3 = """\
########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################
"""

assert solve(EXAMPLE_3) == 132

EXAMPLE_4 = """\
#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################
"""

assert solve(EXAMPLE_4) == 136

EXAMPLE_5 = """\
########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################
"""

assert solve(EXAMPLE_5) == 81





def main():
    text = puzzle_input()
    print(solve(text))


if __name__ == '__main__':
    main()
