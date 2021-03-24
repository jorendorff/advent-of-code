from collections import defaultdict


def infinite_grid(finite_grid):
    points = defaultdict(lambda: '.')
    height = len(finite_grid)
    if height % 2 != 1:
        raise ValueError("expected odd grid height")
    yoff = -(height // 2)
    width = len(finite_grid[0])
    if width % 2 != 1:
        raise ValueError("expected odd grid width")
    xoff = -(width // 2)

    for y, row in enumerate(finite_grid):
        for x, c in enumerate(row):
            points[x + xoff, y + yoff] = c

    return points


class Sim:
    def __init__(self, grid):
        self.grid = infinite_grid(grid)
        self.x = 0
        self.y = 0
        self.dx = 0
        self.dy = -1
        self.infection_bursts = 0

    def draw_map(self, rx, ry):
        return ''.join(
            ''.join(
                self.grid[x, y] + ('[' if (x, y) == (self.x - 1, self.y) else
                                   ']' if (x, y) == (self.x, self.y) else ' ')
                for x in rx).rstrip() + '\n'
            for y in ry)

    def burst(self):
        p = self.x, self.y
        if self.grid[p] == '#':
            self.turn_right()
            self.grid[p] = '.'
        else:
            self.turn_left()
            self.grid[p] = '#'
            self.infection_bursts += 1
        self.x += self.dx
        self.y += self.dy

    def turn_right(self):
        self.dx, self.dy = -self.dy, self.dx

    def turn_left(self):
        self.dx, self.dy = self.dy, -self.dx

sample_map = '''\
..#
#..
...
'''.splitlines()

s = Sim(sample_map)
assert s.draw_map(range(-4, 5), range(-4, 4)) == '''\
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
. . . . . # . . .
. . . #[.]. . . .
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
'''

s.burst()
assert s.draw_map(range(-4, 5), range(-4, 4)) == '''\
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
. . . . . # . . .
. . .[#]# . . . .
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
'''
assert s.infection_bursts == 1

s.burst()
assert s.draw_map(range(-4, 5), range(-4, 4)) == '''\
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
. . .[.]. # . . .
. . . . # . . . .
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
'''

s.burst()
assert s.infection_bursts == 2
s.burst()
assert s.infection_bursts == 3
s.burst()
assert s.infection_bursts == 4
s.burst()
assert s.infection_bursts == 5
assert s.draw_map(range(-4, 5), range(-4, 4)) == '''\
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
. . #[#]. # . . .
. . # # # . . . .
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
'''

s.burst()
assert s.draw_map(range(-4, 5), range(-4, 4)) == '''\
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
. . # .[.]# . . .
. . # # # . . . .
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
'''
assert s.infection_bursts == 5

for i in range(7, 70):
    s.burst()
assert s.draw_map(range(-4, 5), range(-4, 4)) == '''\
. . . . . # # . .
. . . . # . . # .
. . . # . . . . #
. . # . #[.]. . #
. . # . # . . # .
. . . . . # # . .
. . . . . . . . .
. . . . . . . . .
'''
assert (s.dx, s.dy) == (0, -1)
assert s.infection_bursts == 41

for i in range(70, 10000):
    s.burst()
assert s.infection_bursts == 5587

with open('puzzle-input.txt') as f:
    map = f.read().strip().splitlines()
    s = Sim(map)
    for i in range(10000):
        s.burst()
    print(s.infection_bursts)
