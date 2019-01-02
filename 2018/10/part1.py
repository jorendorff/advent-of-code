import re

class P:
    def __init__(self, line):
        m = re.match(r'^position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>', line)
        if m is None:
            raise ValueError("bad line: " + line)
        self.px, self.py, self.vx, self.vy = map(int, m.groups())

    def tick(self):
        self.px += self.vx
        self.py += self.vy

def dump_map(pts):
    x0 = min(p.px for p in pts)
    width = max(p.px for p in pts) + 1 - x0
    y0 = min(p.py for p in pts)
    height = max(p.py for p in pts) + 1 - y0

    grid = [['.'] * width for i in range(height)]
    for p in pts:
        grid[p.py - y0][p.px - x0] = '#'


    for row in grid:
        print(''.join(row))

sample_input = '''\
position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>
'''

with open('puzzle-input.txt') as f:
    text = f.read()

lines = text.strip().splitlines()
points = [P(line) for line in lines]

prev_size = max(p.px for p in points) - min(p.px for p in points)
t = 0
while True:
    for p in points:
        p.tick()
    t += 1
    size = max(p.px for p in points) - min(p.px for p in points)
    if size < 140:
        print(t)
        dump_map(points)
    if size > prev_size:
        break
    prev_size = size
