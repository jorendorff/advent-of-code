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

        self.points = points
        self.x_min = min(x_seen)
        self.x_max = max(x_seen)
        self.y_min = min(y_seen)
        self.y_max = max(y_seen)
        self.todo = [(500, 0)]

    def draw(self):
        s = ''
        for y in range(min(0, self.y_min), self.y_max + 1):
            for x in range(self.x_min - 1, self.x_max + 2):
                s += self.points[x, y]
            s += '\n'
        return s

    def tick(self):
        new_todo = []
        for (x, y) in self.todo[:]:
            c = self.points[x, y]
            if c in '|+':
                if y < self.y_max:
                    below = self.points[x, y + 1]
                    if below == '.':
                        # flow down
                        self.points[x, y + 1] = '|'
                        new_todo.append(x, y + 1)
                    elif below in '~#':
                        # spread sideways
                        if self.points[x - 1, y] == '.':
                            self.points[x - 1, y] = '|'
                            new_todo.append(x - 1, y)
                        if self.points[x + 1, y] == '.':
                            self.points[x + 1, y] = '|'
                            new_todo.append(x + 1, y)

                        # settle
                        if self.points[x - 1, y] == '#':
                            for x2 in range(x + 1, self.x_max + 1):
                                if self.points[x2, y] == '#':
                                    for i in range(x, x2):
                                        self.points[i, y] = '~'
                                        if y > self.y_min and self.points[:
                                            new_todo.append((i, y - 1))
                                    break
                                elif self.points[x2, y] != '|':
                                    break
        return anything_happened

    def count(self):
        return sum(1 for v in self.points.values()
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
assert m.draw() == expected_output
assert m.count() == 57

# main event
with open('puzzle-input.txt') as f:
    text = f.read()
m = Map(text)
while m.tick():
    pass
print(m.draw())
print(m.count())

