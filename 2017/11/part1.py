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

def distance_to_end(path):
    x = y = 0
    for segment in path.split(","):
        dx, dy = DIR[segment]
        x += dx
        y += dy
    return distance(x, y)

assert distance_to_end('ne,ne,ne') == 3
assert distance_to_end('ne,ne,sw,sw') == 0
assert distance_to_end('ne,ne,s,s') == 2
assert distance_to_end('se,sw,se,sw,sw') == 3

with open("puzzle-input.txt") as f:
    print(distance_to_end(f.read().strip()))
