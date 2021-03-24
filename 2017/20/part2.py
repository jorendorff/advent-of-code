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

    def position(self, t):
        x, y, z = self.coeffs
        return (x[2] * t * (t + 1) // 2 + x[1] * t + x[0],
                y[2] * t * (t + 1) // 2 + y[1] * t + y[0],
                z[2] * t * (t + 1) // 2 + z[1] * t + z[0])

class IntSet:
    def __init__(self, values):
        self._set = set(values)

    @classmethod
    def all(cls):
        s = cls([])
        s._set = 'all'
        return s

    def __and__(self, other):
        if self._set == 'all':
            return other
        elif other._set == 'all':
            return self
        else:
            return IntSet(self._set & other._set)

    def min(self):
        if self._set == 'all':
            return 0
        elif len(self._set) == 0:
            return None
        else:
            return min(self._set)

    def is_empty(self):
        return not self._set

def integer_solutions(f1, f2):
    """ Return an IntSet of all nonnegative integers t such that f1(t) == f2(t). """
    (p1, v1, a1), (p2, v2, a2) = f1, f2
    a = a2 - a1
    if a == 0:
        # solve (v2 - v1)t + (p2 - p1) == 0
        v = v2 - v1
        if v == 0:
            # solve p2 - p1 == 0 for t
            if p1 == p2:
                return IntSet.all()
            else:
                return IntSet([])
        else:
            p = p1 - p2
            if p % v == 0:
                t = p // v
                if t >= 0:
                    return IntSet([t])
            return IntSet([])
    else:
        # solve a1 * t * (t + 1) / 2 + v1 * t + p1 ==
        #       a2 * t * (t + 1) / 2 + v2 * t + p2
        # which simplifies to:
        #     (a2 - a1) * t**2 + (a2 - a1 + 2 * (v2 - v1)) * t + 2 * (p2 - p1) == 0
        a = a2 - a1
        b = a + 2 * (v2 - v1)
        c = 2 * (p2 - p1)

        det = b * b - 4 * a * c
        if det < 0:
            return IntSet([])
        q = int(round(sqrt(det)))
        hits = []
        if q * q == det:
            denom = 2 * a
            if (-b + q) % denom == 0:
                t = (-b + q) // denom
                if t >= 0:
                    hits.append(t)
            if (-b - q) % denom == 0:
                t = (-b - q) // denom
                if t >= 0:
                    hits.append(t)
        return IntSet(hits)
    DIE

def all_collision_triples(points):
    for i in range(len(points)):
        P = points[i]
        for j in range(i + 1, len(points)):
            Q = points[j]

            ct = IntSet.all()  # collision times
            for p_d, q_d in zip(P.coeffs, Q.coeffs):
                ct = ct & integer_solutions(p_d, q_d)

            if not ct.is_empty():
                yield ct.min(), i, j

def simulate(lines):
    points = [Point(line) for line in lines]

    def purge():
        for i, pt in enumerate(points):
            if pt is not None and pt.hit:
                points[i] = None

    t_prev = 0
    for t, i, j in sorted(all_collision_triples(points)):
        if t != t_prev:
            t_prev = t
            purge()
        if points[i] is not None and points[j] is not None:
            points[i].hit = True
            points[j].hit = True
    purge()
    return sum(1
               for pt in points
               if pt is not None)

sample_input = '''\
p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>
p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>
p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>
p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>
'''

assert sorted(all_collision_triples([Point(line) for line in sample_input.splitlines()])) == [
    (2, 0, 1),
    (2, 0, 2),
    (2, 1, 2),
]

assert simulate(sample_input.splitlines()) == 1

with open('puzzle-input.txt') as f:
    print(simulate(f))
