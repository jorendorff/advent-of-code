import copy, re

class Point:
    def __init__(self, i, line):
        self.i = i
        m = re.match(r'p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>$',
                     line.strip())
        px, py, pz, vx, vy, vz, ax, ay, az = map(int, m.groups())

        self.coeffs = [
            [px, vx, ax],
            [py, vy, ay],
            [pz, vz, az]
        ]

        # eventual distance at time t is
        # sum, for all dimensions d, of abs(p_d + v_d * t + a_d * t * (t + 1) / 2).
        # For each term, if a_d is positive, then eventually the term is equal to
        #     p_d + v_d * t + a_d * t * (t + 1) / 2
        # and if a_d is negative, then eventually it's equal to
        # the additive inverse of that.
        # In other words, we can flip signs arbitrarily so as to make this quantity positive; let's do it

        coeffs = copy.deepcopy(self.coeffs)
        for dim in coeffs:
            if (dim[2] < 0) or (dim[2] == 0 and dim[1] < 0) or (dim[2] == 0 and dim[1] == 0 and dim[0] < 0):
                for degree in range(len(dim)):
                    dim[degree] = -dim[degree]
        assert all(dim[2] >= 0 for dim in coeffs)

        # now the absolute values fall away in the long run, and we're left with
        #     sum(forall d, p_d + v_d * t + a_d * (t*(t+1)/2))
        # which is equal to
        #     sum(forall d, p_d)
        #     + sum(forall d, v_d)*t
        #     + sum(forall d, a_d)*(t*(t+1)/2)
        self._key = tuple(sum(dim[degree] for dim in coeffs) for degree in (2, 1, 0))

    def key(self):
        return self._key

def compare(lines):
    q = [Point(i, line) for i, line in enumerate(lines)]
    return min(q, key=Point.key).i

sample_input = '''\
p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>
'''

assert compare(sample_input.splitlines()) == 0

with open('puzzle-input.txt') as f:
    print(compare(f))
