from collections import defaultdict

def parse_range(s):
    before, dots, after = s.partition('..')
    if dots:
        return range(int(before), int(after) + 1)
    else:
        return [int(s)]

class Map:
    def __init__(self, text):
        x_seen = set()
        y_seen = set()

        points = defaultdict(lambda: '.')
        for line in text.splitlines():
            x, y = sorted(line.split(', '))
            assert x.startswith('x=')
            assert y.startswith('y=')
            for i in parse_range(x[2:]):
                x_seen.add(i)
                for j in parse_range(y[2:]):
                    y_seen.add(j)
                    points[i, j] = '#'

        x_seen.add(500)
        points[500, 0] = '+'

        self.x_min = min(x_seen)
        self.x_max = max(x_seen)
        self.y_min = min(y_seen)
        self.y_max = max(y_seen)

        assert self.x_min >= 0
        self.map = [[points[x, y]
                     for x in range(0, self.x_max + 2)]
                    for y in range(0, self.y_max + 1)]
        self.flowing = set([(500, 0)])
        self.count = 0
        self.settled_count = 0

    def draw(self):
        s = ''
        for y in range(min(0, self.y_min), self.y_max + 1):
            for x in range(self.x_min - 1, self.x_max + 2):
                s += self.map[y][x]
            s += '\n'
        return s

    def flow(self, x, y):
        assert self.map[y][x] == '.'
        self.map[y][x] = '|'
        self.flowing.add((x, y))
        if y >= self.y_min:
            self.count += 1

    def settle(self, x, y):
        assert self.map[y][x] == '|'
        self.map[y][x] = '~'
        self.flowing.remove((x, y))
        self.settled_count += 1

    def tick(self):
        anything_happened = False
        for x, y in self.flowing.copy():
            row = self.map[y]
            if (x, y) not in self.flowing:
                continue
            assert row[x] in '|+'

            if y < self.y_max:
                row1 = self.map[y + 1]
                below = row1[x]
                if below == '.':
                    # flow down
                    self.flow(x, y + 1)
                    anything_happened = True
                elif below in '~#':
                    # spread sideways
                    i = x - 1
                    while i >= 0 and row1[i + 1] in '~#' and row[i] == '.':
                        self.flow(i, y)
                        anything_happened = True
                        i -= 1
                    i = x + 1
                    while i < len(row) and row1[i - 1] in '~#' and row[i] == '.':
                        self.flow(i, y)
                        anything_happened = True
                        i += 1

                    # settle
                    if row[x - 1] == '#':
                        for x2 in range(x + 1, self.x_max + 1):
                            if row[x2] == '#':
                                anything_happened = True
                                for i in range(x, x2):
                                    self.settle(i, y)
                                break
                            elif row[x2] != '|':
                                break
        return anything_happened

    def check_count(self):
        return self.count == self.accurate_count()

    def accurate_count(self):
        return sum(1
                   for row in self.map[self.y_min:]
                   for v in row
                   if v in '|~')



sample_input = '''\
x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504
'''

sample_map = '''\
......+.......
............#.
.#..#.......#.
.#..#..#......
.#..#..#......
.#.....#......
.#.....#......
.#######......
..............
..............
....#.....#...
....#.....#...
....#.....#...
....#######...
'''

expected_output = '''\
......+.......
......|.....#.
.#..#||||...#.
.#..#~~#|.....
.#..#~~#|.....
.#~~~~~#|.....
.#~~~~~#|.....
.#######|.....
........|.....
...|||||||||..
...|#~~~~~#|..
...|#~~~~~#|..
...|#~~~~~#|..
...|#######|..
'''

# tests
m = Map(sample_input)
assert m.draw() == sample_map
while m.tick():
    pass
print(m.draw())
assert m.draw() == expected_output
assert m.count == 57
assert m.accurate_count() == 57

# main event
with open('puzzle-input.txt') as f:
    text = f.read()
m = Map(text)
while m.tick():
    print(m.count)
print(m.draw())
print(m.count)
print(m.accurate_count())
print(m.settled_count)
