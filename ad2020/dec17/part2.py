from lib.advent import *


class Grid4D:
    def __init__(self, data):
        if isinstance(data, str):
            data = data.strip().split('\n')
            width = len(data[0])
            grid2d = [
                [None] * width
                for y in range(len(data))
            ]
            for y, row in enumerate(data):
                if len(row) != width:
                    raise ValueError("ragged row")
                for x, c in enumerate(row):
                    if c == '.':
                        grid2d[y][x] = 0
                    elif c == '#':
                        grid2d[y][x] = 1
                    else:
                        raise ValueError(f"unrecognized value in input grid: {c!r}")
            data = [[grid2d]]

        self.matrix = data
        self.shape = (len(data[0][0][0]), len(data[0][0]), len(data[0]), len(data))

    def with_padding(self):
        m = self.matrix
        sx, sy, sz, sw = self.shape
        return Grid4D([
            [
                [
                    [
                        m[w][z][y][x] if (0 <= x < sx and 0 <= y < sy and 0 <= z < sz and 0 <= w < sw) else 0
                        for x in range(-1, sx + 1)
                    ]
                    for y in range(-1, sy + 1)
                ]
                for z in range(-1, sz + 1)
            ]
            for w in range(-1, sw + 1)
        ])

    def blur(self):
        sx, sy, sz, sw = self.shape
        rx = range(sx)
        ry = range(sy)
        rz = range(sz)
        rw = range(sw)

        matrix = self.matrix
        # apply blur in x dimension
        matrix = [
            [
                [
                    [
                        sum(matrix[w][z][y][x2] for x2 in range(max(0, x - 1), min(sx, x + 2)))
                        for x in rx
                    ]
                    for y in ry
                ]
                for z in rz
            ]
            for w in rw
        ]
        # apply blur in y dimension
        matrix = [
            [
                [
                    [
                        sum(matrix[w][z][y2][x] for y2 in range(max(0, y - 1), min(sy, y + 2)))
                        for x in rx
                    ]
                    for y in ry
                ]
                for z in rz
            ]
            for w in rw
        ]
        # apply blur in z dimension
        matrix = [
            [
                [
                    [
                        sum(matrix[w][z2][y][x] for z2 in range(max(0, z - 1), min(sz, z + 2)))
                        for x in rx
                    ]
                    for y in ry
                ]
                for z in rz
            ]
            for w in rw
        ]
        # apply blur in w dimension
        matrix = [
            [
                [
                    [
                        sum(matrix[w2][z][y][x] for w2 in range(max(0, w - 1), min(sw, w + 2)))
                        for x in rx
                    ]
                    for y in ry
                ]
                for z in rz
            ]
            for w in rw
        ]
        
        return matrix

    def cycle(self):
        padded = self.with_padding()
        neighbors_plus_self = padded.blur()
        sx, sy, sz, sw = padded.shape
        rx = range(sx)
        ry = range(sy)
        rz = range(sz)
        rw = range(sw)

        prev = padded.matrix
        return Grid4D([
            [
                [
                    [
                        int(3 <= neighbors_plus_self[w][z][y][x] <= 4)
                        if prev[w][z][y][x] == 1
                        else int(neighbors_plus_self[w][z][y][x] == 3)
                        for x in rx
                    ]
                    for y in ry
                ]
                for z in rz
            ]
            for w in rw
        ])

    def count(self):
        return sum(
            sum(row)
            for hyperplane in self.matrix
            for plane in hyperplane
            for row in plane
        )
    
example = """\
.#.
..#
###
"""

test_grid = Grid4D(example)
assert test_grid.count() == 5
test_grid = test_grid.cycle()
assert test_grid.count() == 29
test_grid = test_grid.cycle()
assert test_grid.count() == 60
test_grid = test_grid.cycle()
test_grid = test_grid.cycle()
test_grid = test_grid.cycle()
test_grid = test_grid.cycle()
assert test_grid.count() == 848


if __name__ == '__main__':
    grid = Grid4D(puzzle_input())
    for _round in range(6):
        grid = grid.cycle()
    print(grid.count())
