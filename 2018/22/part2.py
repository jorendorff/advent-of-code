import graph
from collections import namedtuple
import textwrap

ROCKY = '.'
WET = '='
NARROW = '|'

TYPES = [ROCKY, WET, NARROW]
def region_type(erosion_level):
    return TYPES[erosion_level % 3]

MODULUS = 20183

def erosion_level(geologic_index, depth):
    return (geologic_index + depth) % MODULUS

def clean_room_map(depth, map_width, map_height, target):
    grid = [[None] * map_width for i in range(map_height)]
    for x in range(map_width):
        grid[0][x] = erosion_level(x * 16807, depth)
    for y in range(1, map_height):
        grid[y][0] = erosion_level(y * 48271, depth)
        for x in range(1, map_width):
            if (x, y) == target:
                grid[y][x] = erosion_level(0, depth)
            else:
                grid[y][x] = erosion_level(grid[y][x - 1] * grid[y - 1][x], depth)
    return [[region_type(elevel) for elevel in row] for row in grid]

def draw_map(grid, target, max_width, max_height):
    return ''.join(''.join('M' if (x, y) == (0, 0) else 'T' if (x, y) == target else c
                           for x, c in enumerate(row[:max_width])) + '\n'
                   for y, row in enumerate(grid[:max_height]))

DT_MOVE = 1
DT_SWITCH = 7

def round_up_divide(n, d):
    return (n + d - 1) // d

def max_coords(target):
    x, y = target
    max_time = sum(target) * (DT_MOVE + DT_SWITCH) + DT_SWITCH
    
    # what's the farthest you could reach in that amount of time and still make it back to the target?
    # (width + y + (width - x)) * DT_MOVE <= max_time
    # ...solve for width...
    # width <= (max_time / DT_MOVE + x - y) / 2
    max_width = round_up_divide(round_up_divide(max_time, DT_MOVE) + x - y + 1, 2)
    # (height + x + (height - y)) * DT_MOVE <= max_time
    # ...solve for height...
    # height <= (max_time / DT_MOVE + y - x) / 2
    max_height = round_up_divide(round_up_divide(max_time, DT_MOVE) + y - x + 1, 2)

    # add another 1 for good luck
    return max_width + 1, max_height + 1

EQUIPS = ['torch', 'gear', 'neither']
DIRS = [(0, -1), (-1, 0), (1, 0), (0, 1)]

Edge = namedtuple('Edge', 'move destination cost')

def compatible(equip, region):
    if region == ROCKY:
        return equip != 'neither'
    elif region == NARROW:
        return equip != 'gear'
    else:
        assert region == WET
        return equip != 'torch'

def solve(depth, target):
    w, h = max_coords(target)
    my_map = clean_room_map(depth, w, h, target)

    def edges(state):
        x, y, equip = state
        assert compatible(equip, my_map[y][x])
        for dx, dy in DIRS:
            x1 = x + dx
            y1 = y + dy
            if x1 >= 0 and y1 >= 0 and compatible(equip, my_map[y1][x1]):
                yield Edge((dx, dy), (x + dx, y + dy, equip), DT_MOVE)
        for eq in EQUIPS:
            if eq != equip and compatible(eq, my_map[y][x]):
                yield Edge((0, 0), (x, y, eq), DT_SWITCH)

    return graph.cheapest_path((0, 0, 'torch'), target + ('torch',), edges)

def test():
    sample_depth = 510
    sample_target = (10, 10)

    sample_expected = textwrap.dedent('''\
    M=.|=.|.|=.|=|=.
    .|=|=|||..|.=...
    .==|....||=..|==
    =.|....|.==.|==.
    =|..==...=.|==..
    =||.=.=||=|=..|=
    |.=.===|||..=..|
    |..==||=.|==|===
    .=..===..=|.|||.
    .======|||=|=.|=
    .===|=|===T===||
    =|||...|==..|=.|
    =.=|=.=..=.||==|
    ||=|=...|==.=|==
    |=.=||===.|||===
    ||.|==.|.|.||=||
    ''')

    sample_map_str = draw_map(clean_room_map(sample_depth, 16, 16, sample_target), sample_target, 16, 16)
    assert sample_map_str == sample_expected

    assert solve(sample_depth, sample_target).cost == 45

test()

puzzle_input_depth = 11817
puzzle_input_target = (9, 751)

print(solve(puzzle_input_depth, puzzle_input_target).cost)
