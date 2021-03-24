import re
from math import sqrt

class Point:
    __slots__ = 'hit coeffs'.split()

    def __init__(self, line):
        m = re.match(r'p=< *(-?\d+),(-?\d+),(-?\d+)>, v=< *(-?\d+),(-?\d+),(-?\d+)>, a=< *(-?\d+),(-?\d+),(-?\d+)>$',
                     line.strip())
        px, py, pz, vx, vy, vz, ax, ay, az = map(int, m.groups())
        self.coeffs = [
            [px, vx, ax],
            [py, vy, ay],
            [pz, vz, az],
        ]
        self.hit = False

    def tick(self):
        for d in self.coeffs:
            d[1] += d[2]
            d[0] += d[1]
        
    def position(self):
        return tuple(d[0] for d in self.coeffs)

def simulate(lines):
    points = [Point(line) for line in lines]

    def purge():
        for i, pt in enumerate(points):
            if pt is not None and pt.hit:
                points[i] = None

    for t in range(100):
        for i in range(len(points)):
            pi = points[i]
            if pi is not None:
                pi_t = pi.position()
                for pj in points[i + 1:]:
                    if pj is not None and pj.position() == pi_t:
                        pi.hit = True
                        pj.hit = True
        purge()
        for point in points:
            if point is not None:
                point.tick()

    return sum(1
               for pt in points
               if pt is not None)

sample_input = '''\
p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>
p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>
p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>
p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>
'''

assert simulate(sample_input.splitlines()) == 1

with open('puzzle-input.txt') as f:
    print(simulate(f))
