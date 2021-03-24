DIR = {
    'n': (0, -1),
    'ne': (+1, 0),
    'se': (+1, +1),
    's': (0, +1),
    'sw': (-1, 0),
    'nw': (-1, -1),
}

def distance(x, y):
    if y < 0:
        x = -x
        y = -y
    assert y >= 0
    if 0 <= x <= y:
        x = 0
    elif y <= x:
        x -= y
    return abs(x) + y

assert distance(20, 15) == 20
assert distance(10, 15) == 15
assert distance(-5, 15) == 20

def distances(path):
    x = y = 0
    yield distance(x, y)
    for segment in path.split(","):
        dx, dy = DIR[segment]
        x += dx
        y += dy
        yield distance(x, y)

with open("puzzle-input.txt") as f:
    print(max(distances(f.read().strip())))
