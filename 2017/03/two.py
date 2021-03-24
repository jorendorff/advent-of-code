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

def stress():
    store = {(0, 0): 1}
    yield 1

    def get(x, y):
        return store.get((x, y), 0)

    for x, y in spiral():
        if (x, y) != (0, 0):
            total = (get(x - 1, y + 1) + get(x, y + 1) + get(x + 1, y + 1) +
                     get(x - 1, y)                     + get(x + 1, y) +
                     get(x - 1, y - 1) + get(x, y - 1) + get(x + 1, y - 1))
            store[(x, y)] = total
            yield total

import itertools
assert list(itertools.islice(stress(), 23)) == [
    1, 1, 2, 4, 5, 10, 11, 23, 25, 26,
    54, 57, 59, 122, 133, 142, 147, 304, 330, 351,
    362, 747, 806]

for v in stress():
    if v > N:
        print(v)
        break
