from collections import *
import re


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
    rows = [[sum(row[x:x+n]) for x in range(0, width - n)]
            for row in grid]

    def score(x, y):
        return sum([rows[yi][x] for yi in range(y, y + n)])
        
    return max(((x, y, n, score(x, y))
                for x in range(0, width - n) for y in range(height)),
               key= lambda t: t[-1])

def best(grid):
    
    
    def score(p):
        x0, y0, N = p
        return sum(grid[y][x]
                   for y in range(y0, y0+N)
                   for x in range(x0, x0+N))
    bx, by = max(((x, y, n)
                  for x in range(300)
                  for y in range(300)
                  for n in range(1, min(301 - x, 301 - y))),
                 key=score)
    return bx + 1, by + 1

print(best(init(42)))

grid_serial = 7689
grid = init(grid_serial)
print(best(grid))

