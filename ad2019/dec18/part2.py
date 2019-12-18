from lib.advent import *


State = namedtuple("State", "steps positions keys sequence")

def goal(grid):
    return len([
        c
        for row in grid
        for c in row
        if c.islower()
    ])


def imagined_start(grid):
    for y, row in enumerate(grid):
        for x, c in enumerate(row):
            if c == '@':
                return x, y
    raise ValueError("No starting point in grid!")


def fix_grid(grid):
    grid = grid[:]
    x, y = imagined_start(grid)
    grid[y - 1] = grid[y - 1][:x - 1] + ".#." + grid[y - 1][x + 2:]
    grid[y]     = grid[y]    [:x - 1] + "###" + grid[y]    [x + 2:]
    grid[y + 1] = grid[y + 1][:x - 1] + ".#." + grid[y + 1][x + 2:]
    return grid, (x, y)



def neighbors(point):
    x, y = point
    yield x + 1, y
    yield x, y + 1
    yield x - 1, y
    yield x, y - 1


def successors(positions):
    positions = list(positions)
    for i in range(len(positions)):
        for q in neighbors(positions[i]):
            x, y = q
            pc = positions[:]
            pc[i] = q
            yield i, (x, y), tuple(pc)


def compute_targets(grid, positions, keys):
    def get_targets(p):
        targets = {}

        seen = {p}
        todo = deque([(0, p)])
        while todo:
            old_steps, old_position = todo.popleft()
            for new_position in neighbors(old_position):
                x, y = new_position
                c = grid[y][x]
                if c.islower() and c not in keys:
                    # target acquired!
                    if new_position not in targets:
                        #print("robot at", p, "can get to", new_position, "in", old_steps + 1)
                        targets[new_position] = old_steps + 1
                elif c == '.' or c == '@' or c.islower() or (c.isupper() and c.lower() in keys):
                    if new_position not in seen:
                        todo.append((old_steps + 1, new_position))
                        seen.add(new_position)
                elif c == '#' or c.isupper():
                    pass
                else:
                    raise ValueError("what is this: " + repr(c))
        return tuple(targets.items())

    return tuple(get_targets(p) for p in positions)


def solve(text):
    print()
    print("SOLVE")
    grid = text.splitlines()
    grid, center = fix_grid(grid)
    goal_number = goal(grid)
    cx, cy = center
    positions = (
        (cx - 1, cy - 1),
        (cx + 1, cy - 1),
        (cx - 1, cy + 1),
        (cx + 1, cy + 1)
    )
    start_state = State(0, positions, "", "")

    seen = {(positions, ""): 0}

    todo = [start_state]

    while todo:
        todo.sort(key=lambda s: s.steps, reverse=True)
        state = todo.pop()
        print(state)

        if len(state.keys) == goal_number:
            return state.steps


        for i, targets in enumerate(compute_targets(grid, state.positions, state.keys)):
            for new_position, steps in targets:
                x, y = new_position
                new_key = grid[y][x]
                assert new_key.islower() and new_key not in state.keys
                print(f"robot {i} can get to key {new_key} at {new_position} in {steps} steps")
                new_positions = list(state.positions)
                new_positions[i] = new_position
                new_keys = ''.join(sorted(state.keys + new_key))
                new_positions = tuple(new_positions)
                seen_key = new_positions, new_keys
                new_state = State(
                    state.steps + steps,
                    new_positions,
                    new_keys,
                    state.sequence + new_key
                )
                if seen_key in seen and seen[seen_key] > new_state.steps:
                    print(f"  that's faster than our previous time of {seen[seen_key]}!")
                    del seen[seen_key]
                    todo = [s for s in todo if (s.positions, s.keys) != seen_key]
                if seen_key in seen:
                    print("  nah already been there")
                else:
                    seen[seen_key] = new_state.steps
                    print("  ====>", new_state)
                    todo.append(new_state)

EXAMPLE_1 = """\
#######
#a.#Cd#
##...##
##.@.##
##...##
#cB#Ab#
#######
"""

assert solve(EXAMPLE_1) == 8

EXAMPLE_2 = """\
###############
#d.ABC.#.....a#
#######.#######
######.@.######
#######.#######
#b.....#.....c#
###############
"""

assert solve(EXAMPLE_2) == 24

EXAMPLE_3 = """\
#############
#DcBa.#.GhKl#
#.####.##I###
#e#d#.@.#j#k#
###C##.####J#
#fEbA.#.FgHi#
#############
"""

assert solve(EXAMPLE_3) == 32

EXAMPLE_4 = """\
#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba#.#BcIJ#
#####.@.#####
#nK.L#.#G...#
#M###N#H###.#
#o#m..#i#jk.#
#############
"""

assert solve(EXAMPLE_4) == 72


def main():
    print(solve(puzzle_input()))


if __name__ == '__main__':
    main()
