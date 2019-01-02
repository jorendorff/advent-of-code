
def step(grid):
    grid = grid.strip().splitlines()
    w = len(grid[0])
    h = len(grid)
    def get(x, y):
        if 0 <= x < w and 0 <= y < h:
            return grid[y][x]
        else:
            return ' '

    def out(x, y):
        neighbor_trees = 0
        neighbor_yards = 0
        for i in range(x - 1, x + 2):
            for j in range(y - 1, y + 2):
                if (i, j) != (x, y):
                    c = get(i, j)
                    if c == '|':
                        neighbor_trees += 1
                    elif c == '#':
                        neighbor_yards += 1
        c = grid[y][x]
        if c == '.':
            return '|' if neighbor_trees >= 3 else '.'
        elif c == '|':
            return '#' if neighbor_yards >= 3 else '|'
        elif c == '#':
            return '#' if neighbor_yards >= 1 and neighbor_trees >= 1 else '.'
        else:
            assert False # what

    result = [[out(x, y) for x in range(w)]
              for y in range(h)]
    return ''.join(''.join(row) + '\n'
                   for row in result)
    
def value(grid):
    return grid.count('#') * grid.count('|')

example = '''\
.#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.
'''

example_after = '''\
.......##.
......|###
.|..|...#.
..|#||...#
..##||.|#|
...#||||..
||...|||..
|||||.||.|
||||||||||
....||..|.
'''

assert step(example) == example_after

example_after_10 = '''\
.||##.....
||###.....
||##......
|##.....##
|##.....##
|##....##|
||##.####|
||#####|||
||||#|||||
||||||||||
'''

actual_after_10 = example
for i in range(10):
    actual_after_10 = step(actual_after_10)
assert actual_after_10 == example_after_10
assert value(actual_after_10) == 1147


with open("puzzle-input.txt") as f:
    grid = f.read()

seen = {}
states = []

T = 1000000000
for i in range(T):
    print("After {} minutes:".format(i))
    print(grid)
    if grid in seen:
        break
    else:
        seen[grid] = i
        states.append(grid)
        assert seen[states[len(states) - 1]] == len(states) - 1
    grid = step(grid)

assert len(states) == i
assert len(seen) == i

current_time = i
previous_time = seen[grid]
print(current_time, previous_time)
assert grid == states[previous_time]

cycle_length = current_time - previous_time

def grid_after(t):
    return states[previous_time + (t - current_time) % cycle_length]

assert grid_after(current_time) == grid
assert grid_after(previous_time) == grid

while i < 500:
    i += 1
    grid = step(grid)

assert i == 500
assert grid_after(500) == grid


print(value(grid_after(T)))

