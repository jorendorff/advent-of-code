N = int(open('puzzle-input.txt').read())

def spiral():
    x = 0
    y = 0
    yield (x, y)
    n = 1
    while True:
        for i in range(n):
            x += 1
            yield (x, y)
        for i in range(n):
            y += 1
            yield (x, y)
        n += 1
        for i in range(n):
            x -= 1
            yield (x, y)
        for i in range(n):
            y -= 1
            yield (x, y)
        n += 1

import itertools
assert list(itertools.islice(spiral(), 23)) == [
    (0, 0),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
    (2, -1),
    (2, 0),
    (2, 1),
    (2, 2),
    (1, 2),
    (0, 2),
    (-1, 2),
    (-2, 2),
    (-2, 1),
    (-2, 0),
    (-2, -1),
    (-2, -2),
    (-1, -2),
    (0, -2)
]

def distance(square):
    it = itertools.islice(spiral(), square - 1, None)
    x, y = next(it)
    return abs(x) + abs(y)

assert distance(1) == 0
assert distance(12) == 3
assert distance(23) == 2
assert distance(1024) == 31

print(distance(N))
