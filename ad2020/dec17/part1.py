from lib.advent import *


class Grid3D:
    def __init__(self, data, trust_me=False):
        if not trust_me:
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
            data = [grid2d]

        self.matrix = data
        self.shape = (len(data[0][0]), len(data[0]), len(data))

    def with_padding(self):
        m = self.matrix
        sx, sy, sz = self.shape
        return Grid3D(
            [
                [
                    [
                        m[z][y][x] if (0 <= x < sx and 0 <= y < sy and 0 <= z < sz) else 0
                        for x in range(-1, sx + 1)
                    ]
                    for y in range(-1, sy + 1)
                ]
                for z in range(-1, sz + 1)
            ],
            trust_me=True
        )

    def blur(self):
        sx, sy, sz = self.shape
        rx = range(sx)
        ry = range(sy)
        rz = range(sz)

        matrix = self.matrix
        # apply blur in x dimension
        matrix = [
            [
                [
                    sum(matrix[z][y][x2] for x2 in range(max(0, x - 1), min(sx, x + 2)))
                    for x in rx
                ]
                for y in ry
            ]
            for z in rz
        ]
        # apply blur in y dimension
        matrix = [
            [
                [
                    sum(matrix[z][y2][x] for y2 in range(max(0, y - 1), min(sy, y + 2)))
                    for x in rx
                ]
                for y in ry
            ]
            for z in rz
        ]
        # apply blur in z dimension
        matrix = [
            [
                [
                    sum(matrix[z2][y][x] for z2 in range(max(0, z - 1), min(sz, z + 2)))
                    for x in rx
                ]
                for y in ry
            ]
            for z in rz
        ]

        return matrix

    def cycle(self):
        padded = self.with_padding()
        neighbors_plus_self = padded.blur()
        sx, sy, sz = padded.shape
        rx = range(sx)
        ry = range(sy)
        rz = range(sz)

        prev = padded.matrix
        return Grid3D(
            [
                [
                    [
                        int(3 <= neighbors_plus_self[z][y][x] <= 4)
                        if prev[z][y][x] == 1
                        else int(neighbors_plus_self[z][y][x] == 3)
                        for x in rx
                    ]
                    for y in ry
                ]
                for z in rz
            ],
            trust_me=True
        )

    def count(self):
        return sum(
            sum(row)
            for plane in self.matrix
            for row in plane
        )
    
    def to_str(self):
        sx, sy, sz = self.shape
        return "\n".join(
            f"z={z}\n" + "".join(
                ''.join('.#'[c] for c in row) + "\n"
                for row in plane
            )
            for z, plane in enumerate(self.matrix)
        )
    
example = """\
.#.
..#
###
"""

test_grid = Grid3D(example)
assert test_grid.count() == 5
test_grid = test_grid.cycle()
assert test_grid.count() == 11
test_grid = test_grid.cycle()
assert test_grid.count() == 21
test_grid = test_grid.cycle()
test_grid = test_grid.cycle()
test_grid = test_grid.cycle()
test_grid = test_grid.cycle()
assert test_grid.count() == 112


if __name__ == '__main__':
    grid = Grid3D(puzzle_input())
    for _round in range(6):
        grid = grid.cycle()
    print(grid.count())
