class EquivalenceClasses:
    def __init__(self):
        self.groups = {}

    def get(self, a):
        if a not in self.groups:
            self.groups[a] = set([a])
        return self.groups[a]

    def add_edge(self, a, b):
        g1 = self.get(a)
        if b not in g1:
            g2 = self.get(b)
            assert g1 & g2 == set()
            for e in g2:
                g1.add(e)
                self.groups[e] = g1

def solve(lines):
    groups = EquivalenceClasses()
    for line in lines:
        left, right = line.strip().split(" <-> ", 1)
        a = int(left)
        for b in right.split(", "):
            b = int(b)
            groups.add_edge(a, b)
    return len(groups.get(0)), len(set(frozenset(v) for v in groups.groups.values()))


test_input = '''\
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5
'''

assert solve(test_input.splitlines()) == (6, 2)

with open("puzzle-input.txt") as f:
    print(solve(f))

