from lib.advent import *
from lib.grid import Grid

def solve(start):
    grid = Grid(start)

    def transition(point, value):
        if value == 'L':
            if grid.count_adjacent(point, '#') == 0:
                value = '#'
        elif value == '#' and grid.count_adjacent(point, '#') >= 4:
            value = 'L'
        return value

    c = grid.cycle(transition)
    if len(c.cycle) != 1:
        raise ValueError("oscillation detected")
    [s] = c.cycle
    return s.count('#')


example = """\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
"""

assert solve(example) == 37

if __name__ == '__main__':
    print(solve(puzzle_input()))
