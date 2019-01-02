def level(x, y, grid_serial):
    rack = x + 10
    p = (rack * y + grid_serial) * rack
    return p % 1000 // 100 - 5

assert level(3, 5, 8) == 4
assert level(122, 79, 57) == -5
assert level(217, 196, 39) == 0
assert level(101, 153, 71) == 4

def init(grid_serial):
    return [[level(x, y, grid_serial) for x in range(1, 301)] for y in range(1, 301)]

def best_per_n(grid, n):
    width = len(grid[0])
    height = len(grid)
    rows = [[sum(row[x:x+n]) for x in range(0, width + 1 - n)]
            for row in grid]
    assert len(rows[0]) == width + 1 - n

    def score(x, y):
        return sum([rows[yi][x] for yi in range(y, y + n)])

    yay = max(((x, y, n, score(x, y))
               for x in range(0, width + 1 - n)
               for y in range(0, height + 1 - n)),
              key= lambda t: t[-1])
    return yay

def best(grid):
    x, y, n, score = max((best_per_n(grid, n) for n in range(1, 33)),
                         key = lambda t: t[-1])
    return x+1, y+1, n

assert best(init(18)) == (90, 269, 16)
assert best(init(42)) == (232, 251, 12)

grid_serial = 7689
grid = init(grid_serial)
print(best(grid))


