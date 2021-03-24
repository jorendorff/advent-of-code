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
        c = self.grid[p]
        if c == '.':
            self.turn_left()
            self.grid[p] = 'W'
        elif c == 'W':
            self.grid[p] = '#'
            self.infection_bursts += 1
        elif c == '#':
            self.turn_right()
            self.grid[p] = 'F'
        elif c == 'F':
            self.reverse()
            self.grid[p] = '.'

        self.x += self.dx
        self.y += self.dy

    def turn_right(self):
        self.dx, self.dy = -self.dy, self.dx

    def turn_left(self):
        self.dx, self.dy = self.dy, -self.dx

    def reverse(self):
        self.dx, self.dy = -self.dx, -self.dy

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
print(s.draw_map(range(-4, 5), range(-4, 4)))
assert s.draw_map(range(-4, 5), range(-4, 4)) == '''\
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
. . . . . # . . .
. . .[#]W . . . .
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
'''

s.burst()
assert s.draw_map(range(-4, 5), range(-4, 4)) == '''\
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
. . .[.]. # . . .
. . . F W . . . .
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
'''

s.burst()
s.burst()
s.burst()
assert s.draw_map(range(-4, 5), range(-4, 4)) == '''\
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
. . W W . # . . .
. . W[F]W . . . .
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
'''

s.burst()
assert s.draw_map(range(-4, 5), range(-4, 4)) == '''\
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
. . W W . # . . .
. .[W]. W . . . .
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
'''

s.burst()
assert s.draw_map(range(-4, 5), range(-4, 4)) == '''\
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
. . W W . # . . .
.[.]# . W . . . .
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
'''

for i in range(7, 100):
    s.burst()
assert s.infection_bursts == 26

for i in range(100, 10000000):
    s.burst()
assert s.infection_bursts == 2511944

with open('puzzle-input.txt') as f:
    map = f.read().strip().splitlines()
    s = Sim(map)
    for i in range(10000000):
        s.burst()
    print(s.infection_bursts)
