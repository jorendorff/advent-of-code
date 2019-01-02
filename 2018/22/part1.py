

MODULUS = 20183

def erosion_level(geologic_index, depth):
    return (geologic_index + depth) % MODULUS

ROCKY = '.'
WET = '='
NARROW = '|'


TYPES = [ROCKY, WET, NARROW]
def region_type(erosion_level):
    return TYPES[erosion_level % 3]

def risk_level(geologic_index, depth):
    return erosion_level(geologic_index, depth) % 3

def geologic_index_map(depth, w, h, target):
    rows = []
    for y in range(h):
        if y == 0:
            row = [(x * 16807) % MODULUS for x in range(w)]
        else:
            prev_row = row
            row = [(y * 48271) % MODULUS]
            for x in range(1, w):
                if (x, y) == target:
                    row.append(0)
                else:
                    el1 = erosion_level(prev_row[x], depth)
                    el2 = erosion_level(row[-1], depth)
                    row.append((el1 * el2) % MODULUS)
        rows.append(row)
    return rows

def draw_map(depth, map_width, map_height, target):
    gi_map = geologic_index_map(depth, map_width, map_height, target)
    cave_map = [[region_type(erosion_level(gi, depth)) for gi in row] for row in gi_map]
    cave_map[0][0] = 'M'
    tx, ty = target
    cave_map[ty][tx] = 'T'
    cave_map_str = ''.join(''.join(row) + '\n' for row in cave_map)
    return cave_map_str

def clean_room_map(depth, map_width, map_height, target):
    grid = [[None] * map_width for i in range(map_height)]
    for x in range(map_width):
        grid[0][x] = erosion_level(x * 16807, depth)
    for y in range(1, map_height):
        grid[y][0] = erosion_level(y * 48271, depth)
        for x in range(1, map_width):
            if (x, y) == target:
                grid[y][x] = 0
            else:
                grid[y][x] = erosion_level(grid[y][x - 1] * grid[y - 1][x], depth)
    for y in range(map_height):
        for x in range(map_width):
            grid[y][x] = region_type(grid[y][x])
    return grid

def total_risk_level(depth, target):
    tx, ty = target
    gi = geologic_index_map(depth, tx + 1, ty + 1, target)
    return sum(risk_level(gi_value, depth) for row in gi for gi_value in row)

sample_depth = 510

sample_expected = '''\
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
'''

sample_map_str = draw_map(sample_depth, 16, 16, (10, 10))
#print(sample_map_str)
assert sample_map_str == sample_expected
assert total_risk_level(sample_depth, (10, 10)) == 114

puzzle_input_depth = 11817
puzzle_input_target = (9, 751)
print(draw_map(puzzle_input_depth, 70, 800, puzzle_input_target))
#print(total_risk_level(puzzle_input_depth, puzzle_input_target))


